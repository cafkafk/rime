// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use super::endpoints::auto_discover;

use axum::{routing::get, Router};

pub fn get_routes() -> Router {
    Router::new().route("/:host/*req", get(auto_discover))
}
