// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::{Forge, ForgeError};

#[derive(Clone)]
pub struct GitHub;

#[axum::async_trait]
impl Forge for GitHub {
    fn new() -> Self {
        Self
    }

    fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Ok("github.com".to_string())
    }

    fn get_api_releases_url(
        &self,
        _host: &str,
        user: &str,
        repo: &str,
        page_size: u8,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "http://api.github.com/repos/{}/{}/releases?per_page={}",
            user, repo, page_size
        ))
    }

    fn get_tarball_url_for_branch(
        &self,
        _host: &str,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "https://github.com/{}/{}/archive/refs/heads/{}.tar.gz",
            user, repo, branch
        ))
    }

    fn get_tarball_url_for_version(
        &self,
        _host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "https://github.com/{}/{}/archive/refs/tags/{}.tar.gz",
            user, repo, version
        ))
    }

    fn get_repo_url(&self, _host: &str, user: &str, repo: &str) -> String {
        format!("https://github.com/{}/{}", user, repo)
    }
}
