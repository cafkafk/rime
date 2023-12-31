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
    async fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Ok("flakehub.com".to_string())
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

    async fn get_tarball_url_for_semantic_version(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<Option<String>, ForgeError> {
        let version: String = form_urlencoded::byte_serialize(version.as_bytes()).collect();
        let url = self
            .get_tarball_url_for_version(host, user, repo, &version)
            .await?;
        Ok(Some(url))
    }

    async fn get_tarball_url_for_latest_release(
        &self,
        host: &str,
        user: &str,
        repo: &str,
    ) -> Result<Option<String>, ForgeError> {
        Ok(Some(
            self.get_tarball_url_for_version(host, user, repo, "*")
                .await?,
        ))
    }

    async fn get_tarball_url_for_branch(
        &self,
        _host: &str,
        _user: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<String, ForgeError> {
        Err(ForgeError::EndpointUnavailable)
    }

    async fn get_tarball_url_for_version(
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

    async fn get_repo_url(
        &self,
        _host: &str,
        user: &str,
        repo: &str,
    ) -> Result<String, ForgeError> {
        Ok(format!("https://flakehub.com/flake/{}/{}", user, repo))
    }
}
