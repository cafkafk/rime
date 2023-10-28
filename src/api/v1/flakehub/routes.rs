// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::endpoints::get_repo_version;

use axum::{routing::get, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/v1/flakehub/:user/:repo/v/:version", get(get_repo_version))
        .route(
            "/v1/flakehub/:user/:repo/version/:version",
            get(get_repo_version),
        )
}
