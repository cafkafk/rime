// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::flakehub::routes::get_routes as get_flakehub_routes;
use super::forgejo::routes::get_routes as get_forgejo_routes;
use super::github::routes::get_routes as get_github_routes;
use axum::Router;

pub fn get_routes() -> Router {
    Router::new()
        .merge(get_github_routes())
        .merge(get_flakehub_routes())
        .merge(get_forgejo_routes())
}
