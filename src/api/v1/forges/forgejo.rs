// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    extract::Path,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use log::trace;
use reqwest::header::{ACCEPT, USER_AGENT};

use super::super::{Forge, ForgeError};

#[derive(Clone)]
pub struct Forgejo;

impl Forgejo {
    fn get_tarball_url_for_ref(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        git_ref: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "https://{}/{}/{}/archive/{}.tar.gz",
            host, user, repo, git_ref
        ))
    }

    pub async fn is_host_forgejo(host: &str) -> Result<bool, ForgeError> {
        // I couldn't find a more reasonable way to detect Forgejo, so we'll check
        // an API endpoint that is specific to this forge, and if it exists, we
        // assume it's a Forgejo instance.
        let uri = format!("https://{}/api/v1/settings/api", &host);
        let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        let res = client
            .get(&uri)
            .header(ACCEPT, "application/json")
            .send()
            .await?;
        Ok(res.status() == 200)
    }

    async fn codeberg_redirect(Path(url): Path<String>) -> Response {
        let target = format!("/v1/forgejo/codeberg.org/{}", url);
        trace!("codeberg_redirect_target: {target}");
        Redirect::to(&target).into_response()
    }

    pub fn get_redirect_routes(&self) -> Router {
        Router::new().route("/codeberg/*req", get(Self::codeberg_redirect))
    }
}

#[axum::async_trait]
impl Forge for Forgejo {
    fn new() -> Self {
        Self
    }

    fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Ok("codeberg.org".to_string())
    }

    fn get_api_releases_url(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        page_size: u8,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "https://{}/api/v1/repos/{}/{}/releases?limit={}",
            host, user, repo, page_size
        ))
    }

    fn get_tarball_url_for_branch(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String, ForgeError> {
        self.get_tarball_url_for_ref(host, user, repo, branch)
    }

    fn get_tarball_url_for_version(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError> {
        self.get_tarball_url_for_ref(host, user, repo, version)
    }

    fn get_repo_url(&self, host: &str, user: &str, repo: &str) -> String {
        format!("https://{}/{}/{}", host, user, repo)
    }
}
