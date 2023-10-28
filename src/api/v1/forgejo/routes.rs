// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::endpoints::{codeberg_redirect, get_repo, get_repo_ref};

use axum::{routing::get, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/v1/forgejo/:host/:user/:repo/b/:branch", get(get_repo_ref))
        .route(
            "/v1/forgejo/:host/:user/:repo/branch/:branch",
            get(get_repo_ref),
        )
        .route(
            "/v1/forgejo/:host/:user/:repo/v/:version",
            get(get_repo_ref),
        )
        .route(
            "/v1/forgejo/:host/:user/:repo/version/:version",
            get(get_repo_ref),
        )
        .route(
            "/v1/forgejo/:host/:user/:repo/t/:version",
            get(get_repo_ref),
        )
        .route(
            "/v1/forgejo/:host/:user/:repo/tag/:version",
            get(get_repo_ref),
        )
        .route("/v1/forgejo/:host/:user/:repo", get(get_repo))
        .route("/v1/codeberg/*req", get(codeberg_redirect))
}
