// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::async_trait;
use std::sync::Arc;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

mod error;
pub use error::ForgeError;
pub mod handlers;
mod releases;
pub use releases::ForgeReleases;

#[async_trait]
pub trait Forge {
    async fn get_flagship_host(&self) -> Result<String, ForgeError>;

    async fn get_api_releases_url(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        page_size: u8,
    ) -> Result<String, ForgeError>;

    async fn get_tarball_url_for_branch(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String, ForgeError>;

    async fn get_tarball_url_for_version(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError>;

    async fn get_repo_url(&self, host: &str, user: &str, repo: &str) -> Result<String, ForgeError>;
}

pub type DynForge = Arc<dyn Forge + Send + Sync>;
