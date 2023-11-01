// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{async_trait, extract::Extension, routing::get, Router};
use std::sync::Arc;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

mod error;
pub use error::ForgeError;
mod handlers;
mod releases;
pub use releases::ForgeReleases;

#[async_trait]
pub trait Forge {
    fn new() -> Self
    where
        Self: Sized;

    fn get_flagship_host(&self) -> Result<String, ForgeError>;

    fn get_api_releases_url(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        page_size: u8,
    ) -> Result<String, ForgeError>;

    fn get_tarball_url_for_branch(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String, ForgeError>;

    fn get_tarball_url_for_version(
        &self,
        host: &str,
        user: &str,
        repo: &str,
        version: &str,
    ) -> Result<String, ForgeError>;

    fn get_repo_url(&self, host: &str, user: &str, repo: &str) -> String;

    // The functions below come with reasonable defaults. In most cases, structs
    // implementing the trait will not need to override them.

    fn get_routes(&self) -> Router
    where
        Self: Sized + Send + Sync + 'static,
    {
        Router::new()
            .route(
                "/:host/:user/:repo",
                get(handlers::get_tarball_url_for_latest_release),
            )
            .route(
                "/:host/:user/:repo/branch/*branch",
                get(handlers::get_tarball_url_for_branch),
            )
            .route(
                "/:host/:user/:repo/b/*branch",
                get(handlers::get_tarball_url_for_branch),
            )
            .route(
                "/:host/:user/:repo/version/:version",
                get(handlers::get_tarball_url_for_version),
            )
            .route(
                "/:host/:user/:repo/v/:version",
                get(handlers::get_tarball_url_for_version),
            )
            .route(
                "/:host/:user/:repo/tag/:version",
                get(handlers::get_tarball_url_for_version),
            )
            .route(
                "/:host/:user/:repo/t/:version",
                get(handlers::get_tarball_url_for_version),
            )
            .layer(Extension(Arc::new(Self::new()) as DynForge))
    }

    fn get_flagship_routes(&self) -> Router
    where
        Self: Sized + Send + Sync + 'static,
    {
        Router::new()
            .route(
                "/:user/:repo",
                get(handlers::get_tarball_url_for_latest_release_from_flagship),
            )
            .route(
                "/:user/:repo/branch/*branch",
                get(handlers::get_tarball_url_for_branch_from_flagship),
            )
            .route(
                "/:user/:repo/b/*branch",
                get(handlers::get_tarball_url_for_branch_from_flagship),
            )
            .route(
                "/:user/:repo/version/:version",
                get(handlers::get_tarball_url_for_version_from_flagship),
            )
            .route(
                "/:user/:repo/v/:version",
                get(handlers::get_tarball_url_for_version_from_flagship),
            )
            .route(
                "/:user/:repo/tag/:version",
                get(handlers::get_tarball_url_for_version_from_flagship),
            )
            .route(
                "/:user/:repo/t/:version",
                get(handlers::get_tarball_url_for_version_from_flagship),
            )
            .layer(Extension(Arc::new(Self::new()) as DynForge))
    }
}

pub type DynForge = Arc<dyn Forge + Send + Sync>;
