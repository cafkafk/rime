// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::v1::routes::get_routes as get_v1_routes;
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature="ssr")] {
use axum::Router;

pub fn get_routes() -> Router {
    Router::new().nest("/v1", get_v1_routes())
}
}}
