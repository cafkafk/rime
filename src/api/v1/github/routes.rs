// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::endpoints::{get_repo, get_repo_branch, get_repo_version};

use axum::{routing::get, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/:user/:repo/b/:branch", get(get_repo_branch))
        .route("/:user/:repo/branch/:branch", get(get_repo_branch))
        .route("/:user/:repo/v/:version", get(get_repo_version))
        .route("/:user/:repo/version/:version", get(get_repo_version))
        .route("/:user/:repo/t/:version", get(get_repo_version))
        .route("/:user/:repo/tag/:version", get(get_repo_version))
        .route("/:user/:repo", get(get_repo))
}
