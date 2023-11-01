// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::{Forge, ForgeError};

#[derive(Clone)]
pub struct FlakeHub;

#[axum::async_trait]
impl Forge for FlakeHub {
    fn new() -> Self {
        Self
    }

    fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Ok("flakehub.com".to_string())
    }

    fn get_api_releases_url(
        &self,
        _host: &str,
        _user: &str,
        _repo: &str,
        _page_size: u8,
    ) -> Result<String, ForgeError> {
        Err(ForgeError::EndpointUnavailable)
    }

    fn get_tarball_url_for_branch(
        &self,
        _host: &str,
        _user: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<String, ForgeError> {
        Err(ForgeError::EndpointUnavailable)
    }

    fn get_tarball_url_for_version(
        &self,
        _host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!(
            "https://flakehub.com/f/{}/{}/{}.tar.gz",
            user, repo, version
        ))
    }

    fn get_repo_url(&self, _host: &str, user: &str, repo: &str) -> String {
        format!("https://flakehub.com/flake/{}/{}", user, repo)
    }
}
