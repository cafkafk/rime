// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};

use super::super::super::{ForgeError, Forgejo, Gitlab};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
struct AutoDiscovery(String, Option<String>);

impl AutoDiscovery {
    pub fn new_for(host: &str) -> Self {
        Self(host.to_string(), None)
    }

    pub async fn try_forgejo(self) -> Result<Self, ForgeError> {
        if self.1.is_some() {
            return Ok(self);
        }

        if Forgejo::is_host_forgejo(&self.0).await? {
            Ok(Self(
                self.0.clone(),
                Some(format!("/v1/forgejo/{}", self.0)),
            ))
        } else {
            Ok(Self(self.0, None))
        }
    }

    pub async fn try_gitlab(self) -> Result<Self, ForgeError> {
        if self.1.is_some() {
            return Ok(self);
        }

        if Gitlab::is_host_gitlab(&self.0).await? {
            Ok(Self(self.0.clone(), Some(format!("/v1/gitlab/{}", self.0))))
        } else {
            Ok(Self(self.0, None))
        }
    }

    pub fn url(self, url: &str) -> Option<String> {
        self.1.map(|forge_api| format!("{}/{}", forge_api, url))
    }
}

pub async fn auto_discover(
    Path((host, url)): Path<(String, String)>,
) -> Result<Response, ForgeError> {
    let target = AutoDiscovery::new_for(&host)
        .try_forgejo()
        .await?
        .try_gitlab()
        .await?
        .url(&url);
    trace!("target: {target:#?}");

    if let Some(target_url) = target {
        Ok(Redirect::to(&target_url).into_response())
    } else {
        let body = format!("Unable to auto-discover the forge at {:#?} :(", host);
        Ok((StatusCode::BAD_REQUEST, body).into_response())
    }
}
