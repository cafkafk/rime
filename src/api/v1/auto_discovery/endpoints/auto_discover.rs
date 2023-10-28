// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::api::v1::{forgejo::is_forgejo, gitlab::is_gitlab};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
struct AutoDiscovery(String, Option<String>);

impl AutoDiscovery {
    pub fn new_for(host: &str) -> Self {
        Self(host.to_string(), None)
    }

    pub async fn try_forgejo(self) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if self.1.is_some() {
            return Ok(self);
        }
        if is_forgejo(&self.0).await? {
            Ok(Self(
                self.0.clone(),
                Some(format!("/v1/forgejo/{}", self.0)),
            ))
        } else {
            Ok(Self(self.0, None))
        }
    }

    pub async fn try_gitlab(self) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if self.1.is_some() {
            return Ok(self);
        }
        if is_gitlab(&self.0).await? {
            Ok(Self(self.0.clone(), Some(format!("/v1/gitlab/{}", self.0))))
        } else {
            Ok(Self(self.0, None))
        }
    }

    pub fn url(self, url: &str) -> Option<String> {
        self.1.map(|forge_api| format!("{}/{}", forge_api, url))
    }
}

pub async fn auto_discover(Path((host, url)): Path<(String, String)>) -> impl IntoResponse {
    let target = AutoDiscovery::new_for(&host)
        .try_forgejo()
        .await
        .expect("failed to await try_forgejo")
        .try_gitlab()
        .await
        .expect("failed to await try_gitlab")
        .url(&url);
    trace!("target: {target:#?}");

    if let Some(target_url) = target {
        Redirect::to(&target_url).into_response()
    } else {
        let body = format!("Unable to auto-discover the forge at {:#?} :(", host);
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
