// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::{Forge, ForgeError};

#[derive(Clone)]
pub struct GitHub;

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature="ssr")] {
#[axum::async_trait]
impl Forge for GitHub {
    async fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Ok("github.com".to_string())
    }

    async fn get_api_releases_url(
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

    async fn get_tarball_url_for_branch(
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

    async fn get_tarball_url_for_version(
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

    async fn get_repo_url(
        &self,
        _host: &str,
        user: &str,
        repo: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!("https://github.com/{}/{}", user, repo))
    }
}
}}
