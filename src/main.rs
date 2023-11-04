//TODO:  #![deny(clippy::unwrap_used)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    rime::server::main().await
}

// #[tokio::main]
// async fn main() {

//     axum::Server::bind(&config.bind_addr())
//         .serve(app.into_make_service())
//         .await
//         .expect("failed to await on bind().serve()");
// }
