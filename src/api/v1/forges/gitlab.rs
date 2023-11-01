// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use reqwest::header::{ACCEPT, USER_AGENT};

use super::super::{Forge, ForgeError};

#[derive(Clone)]
pub struct Gitlab;

impl Gitlab {
    fn get_tarball_url_for_ref(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        git_ref: &str,
    ) -> Result<String, ForgeError> {
        let git_ref_dashed_name = git_ref.replace('/', "-");
        Ok(format!(
            "https://{}/{}/{}/-/archive/{}/{}-{}.tar.gz",
            host, user, repo, git_ref, repo, git_ref_dashed_name,
        ))
    }

    pub async fn is_host_gitlab(host: &str) -> Result<bool, ForgeError> {
        // Detecting GitLab is a bit tricky: while it has a /version and a /metadata
        // endpoint, both require authentication, so any data they may return is
        // unusable. Thankfully, every request that hits the GitLab API has an
        // `x-gitlab-meta` header, that's a strong enough indication that we're
        // dealing with GitLab.

        let uri = format!("https://{}/api/v4/projects?per_page=1", &host);
        let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        let res = client
            .get(uri)
            .header(ACCEPT, "application/json")
            .send()
            .await?;
        Ok(res.status() == 200 && res.headers().contains_key("x-gitlab-meta"))
    }
}

#[axum::async_trait]
impl Forge for Gitlab {
    fn new() -> Self {
        Self
    }

    fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Err(ForgeError::NoFlagshipInstance)
    }

    fn get_api_releases_url(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        page_size: u8,
    ) -> Result<String, ForgeError> {
        // TODO: The middle part, `{}%2F{}` is really `user/repo` URL-encoded. We
        // should do proper URL encoding.
        Ok(format!(
            "https://{}/api/v4/projects/{}%2F{}/releases?per_page={}",
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
