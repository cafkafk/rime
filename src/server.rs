use std::convert::Infallible;

use crate::api::routes::get_routes as get_api_routes;
use crate::app::App;
use crate::data;
use crate::thing::{ReadThings, Thing};

use axum::response::Response as AxumResponse;
use axum::{body::Body, http::Request, response::IntoResponse};
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::services::ServeDir;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub async fn main() {
    let matches = crate::cli::build_cli().get_matches();

    let config;

    if let Some(config_file) = matches.get_one::<String>("config") {
        config = crate::data::Config::load(config_file);
    } else {
        config = crate::data::Config::load(data::CONFIG);
    }

    trace!("{config:#?}");

    let conf = get_configuration(None).await.unwrap();
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    let client_dist = ServeDir::new(conf.leptos_options.site_root.clone());
    let leptos_options = conf.leptos_options.clone(); // A copy to move to the closure below.
    let not_found_service =
        tower::service_fn(move |req| not_found_handler(leptos_options.to_owned(), req));
    let leptos_routes = Router::new()
        // Good old rime
        // custom routes
        .route("/hello", get(root))
        // server functions API routes
        .route("/aapi/*fn_name", post(leptos_axum::handle_server_fns))
        // application routes
        .leptos_routes(&conf.leptos_options, routes, |cx| view! { cx, <App/> })
        // static files are served as fallback (but *before* falling back to
        // error handler)
        .fallback_service(client_dist.clone().not_found_service(not_found_service))
        .with_state(conf.leptos_options.clone());
    let api_routes = Router::new()
        .merge(get_api_routes())
        .layer(Extension(config.clone()));
    let app = Router::new().merge(leptos_routes).merge(api_routes);
    println!("Launching http://{}", &conf.leptos_options.site_addr);
    println!("fn_url: {}", ReadThings::url());
    axum::Server::bind(&conf.leptos_options.site_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// On missing routes, just delegate to the leptos app, which has a route
// fallback rendering 404 response.
pub async fn not_found_handler(
    options: LeptosOptions,
    req: Request<Body>,
) -> Result<AxumResponse, Infallible> {
    let handler =
        leptos_axum::render_app_to_stream(options.to_owned(), move |cx| view! { cx, <App/> });
    Ok(handler(req).await.into_response())
}

#[debug_handler]
async fn root() -> String {
    let thing = Thing::new(42, "Hello from backend".to_string());
    serde_json::to_string(&thing).unwrap()
}
