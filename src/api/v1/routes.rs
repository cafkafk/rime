// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::flakehub::routes::get_routes as get_flakehub_routes;
use super::forgejo::routes::get_redirect_routes as get_forgejo_redirect_routes;
use super::forgejo::routes::get_routes as get_forgejo_routes;
use super::github::routes::get_routes as get_github_routes;
use super::gitlab::routes::get_routes as get_gitlab_routes;
use axum::Router;

pub fn get_routes() -> Router {
    Router::new()
        .nest("/github", get_github_routes())
        .nest("/flakehub", get_flakehub_routes())
        .nest("/forgejo", get_forgejo_routes())
        .nest("/gitea", get_forgejo_routes())
        .nest("/gitlab", get_gitlab_routes())
        .merge(get_forgejo_redirect_routes())
}
