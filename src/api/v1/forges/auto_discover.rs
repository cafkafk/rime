// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::super::{DynForge, Forge, ForgeError, Forgejo, Gitlab};
use log::trace;
use std::sync::Arc;

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature="ssr")] {
#[derive(Clone)]
pub struct AutoDiscover;

impl AutoDiscover {
    async fn try_autodiscovery_for(&self, host: &str) -> Result<DynForge, ForgeError> {
        if Forgejo::is_host_forgejo(host).await? {
            trace!("Forgejo discovered at {host}");
            Ok(Arc::new(Forgejo) as DynForge)
        } else if Gitlab::is_host_gitlab(host).await? {
            trace!("Gitlab discovered at {host}");
            Ok(Arc::new(Gitlab) as DynForge)
        } else {
            Err(ForgeError::AutodetectFailed(host.to_string()))
        }
    }
}

#[axum::async_trait]
impl Forge for AutoDiscover {
    async fn get_flagship_host(&self) -> Result<String, ForgeError> {
        Err(ForgeError::NoFlagshipInstance)
    }

    async fn get_api_releases_url(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        page_size: u8,
    ) -> Result<String, ForgeError> {
        let forge = self.try_autodiscovery_for(host).await?;
        forge
            .get_api_releases_url(host, user, repo, page_size)
            .await
    }

    async fn get_tarball_url_for_branch(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String, ForgeError> {
        let forge = self.try_autodiscovery_for(host).await?;
        forge
            .get_tarball_url_for_branch(host, user, repo, branch)
            .await
    }

    async fn get_tarball_url_for_version(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError> {
        let forge = self.try_autodiscovery_for(host).await?;
        forge
            .get_tarball_url_for_version(host, user, repo, version)
            .await
    }

    async fn get_repo_url(&self, host: &str, user: &str, repo: &str) -> Result<String, ForgeError> {
        let forge = self.try_autodiscovery_for(host).await?;
        forge.get_repo_url(host, user, repo).await
    }
}
}}
