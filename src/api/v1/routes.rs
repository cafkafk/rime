// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::github::routes::get_routes as get_github_routes;
use axum::Router;

pub fn get_routes() -> Router {
    Router::new().merge(get_github_routes())
}
