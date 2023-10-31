// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::Router;

use super::auto_discovery::routes::get_routes as get_auto_discovery_routes;
use super::flakehub::routes::get_routes as get_flakehub_routes;
use super::forgejo::routes::get_redirect_routes as get_forgejo_redirect_routes;
use super::forgejo::routes::get_routes as get_forgejo_routes;
use super::github::routes::get_routes as get_github_routes;
use super::gitlab::routes::get_routes as get_gitlab_routes;
use super::sourcehut::routes::get_routes as get_sourcehut_routes;

pub fn get_routes() -> Router {
    Router::new()
        .merge(get_auto_discovery_routes())
        .nest("/github", get_github_routes())
        .nest("/github.com", get_github_routes())
        .nest("/flakehub", get_flakehub_routes())
        .nest("/flakehub.com", get_flakehub_routes())
        .nest("/forgejo", get_forgejo_routes())
        .nest("/gitea", get_forgejo_routes())
        .nest("/gitlab", get_gitlab_routes())
        .nest("/sourcehut", get_sourcehut_routes())
        .merge(get_forgejo_redirect_routes())
}
