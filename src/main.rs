// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

#![deny(clippy::unwrap_used)]

use axum::{extract::Extension, response::Redirect, routing::get, Router};
use tokio::net::TcpListener;

extern crate log;
extern crate pretty_env_logger;

mod api;
mod cli;
mod data;

use api::routes::get_routes as get_api_routes;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let matches = crate::cli::build_cli().get_matches();

    let config;

    if let Some(config_file) = matches.get_one::<String>("config") {
        config = crate::data::Config::load(config_file);
    } else {
        config = crate::data::Config::load(data::CONFIG);
    }

    trace!("{config:#?}");

    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::to("https://github.com/cafkafk/rime") }),
        )
        .merge(get_api_routes())
        .layer(Extension(config.clone()));

    let listener = TcpListener::bind(&config.bind_addr())
        .await
        .expect("failed to bind");
    axum::serve(listener, app)
        .await
        .expect("failed to serve app");
}
