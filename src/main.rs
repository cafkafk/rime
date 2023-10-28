// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

#![deny(clippy::unwrap_used)]

use axum::{response::Redirect, routing::get, Router};

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

    let socket_addr: String = format!(
        "{}:{}",
        config.addr.expect("couldn't unwrap config.addr"),
        config.port.expect("couldn't unwrap config.addr")
    );

    debug!("{socket_addr:#?}");

    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::to("https://github.com/cafkafk/rime") }),
        )
        .merge(get_api_routes());

    axum::Server::bind(&socket_addr.parse().expect("failed to parse socket_addr"))
        .serve(app.into_make_service())
        .await
        .expect("failed to await on bind().serve()");
}
