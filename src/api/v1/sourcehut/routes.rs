// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{routing::get, Router};

use super::endpoints::get_repo_ref;

pub fn get_routes() -> Router {
    Router::new()
        .route("/:host/:user/:repo/b/*version", get(get_repo_ref))
        .route("/:host/:user/:repo/branch/*version", get(get_repo_ref))
        .route("/:host/:user/:repo/v/:version", get(get_repo_ref))
        .route("/:host/:user/:repo/version/:version", get(get_repo_ref))
        .route("/:host/:user/:repo/t/:version", get(get_repo_ref))
        .route("/:host/:user/:repo/tag/:version", get(get_repo_ref))
}
