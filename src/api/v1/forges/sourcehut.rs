// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::{Forge, ForgeError};

#[derive(Clone)]
pub struct SourceHut;

impl SourceHut {
    fn get_tarball_url_for_ref(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        git_ref: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "https://{}/~{}/{}/archive/{}.tar.gz",
            host, user, repo, git_ref
        ))
    }
}

#[axum::async_trait]
impl Forge for SourceHut {
    async fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Ok("git.sr.ht".to_string())
    }

    async fn get_api_releases_url(
        &self,
        _host: &str,
        _user: &str,
        _repo: &str,
        _page_size: u8,
    ) -> Result<String, ForgeError> {
        Err(ForgeError::EndpointUnavailable)
    }

    async fn get_tarball_url_for_branch(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String, ForgeError> {
        self.get_tarball_url_for_ref(host, user, repo, branch)
    }

    async fn get_tarball_url_for_version(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError> {
        self.get_tarball_url_for_ref(host, user, repo, version)
    }

    async fn get_repo_url(&self, host: &str, user: &str, repo: &str) -> Result<String, ForgeError> {
        Ok(format!("https://{}/~{}/{}", host, user, repo))
    }
}
