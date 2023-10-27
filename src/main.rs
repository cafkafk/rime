// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

extern crate log;
extern crate pretty_env_logger;

mod cli;
mod data;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

async fn get_repo_branch_v1(
    Path((forge, user, repo, branch)): Path<(String, String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if branch.ends_with("tar.gz") {
        let uri = format!(
            "https://{}.com/{}/{}/archive/refs/heads/{}.tar.gz",
            forge,
            user,
            repo,
            branch.strip_suffix(".tar.gz").unwrap()
        );
        Redirect::to(&uri).into_response()
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

async fn github_api_get_latest_tag(
    user: String,
    repo: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use reqwest::{
        header::{ACCEPT, USER_AGENT},
        Url,
    };
    let version_uri = Url::parse(&format!(
        "http://api.github.com/repos/{}/{}/releases?per_page=1",
        user, repo
    ))?;
    trace!("{:#?}", version_uri);
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let res = client
        .get(version_uri)
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .json::<serde_json::Value>()
        // .text()
        .await?;

    trace!("got:\n {:#?}", res[0]["tag_name"]);

    Ok(res[0]["tag_name"].as_str().unwrap().to_string())
}

async fn get_repo_v1(
    Path((forge, user, repo)): Path<(String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if repo.ends_with("tar.gz") {
        let version = github_api_get_latest_tag(
            user.clone(),
            repo.clone().strip_suffix(".tar.gz").unwrap().to_string(),
        )
        .await
        .unwrap();
        let result_uri = format!(
            "http://{}.com/{}/{}/archive/refs/tags/{}.tar.gz",
            forge,
            user,
            repo.strip_suffix(".tar.gz").unwrap(),
            version,
        );
        trace!("{result_uri:#?}");
        Redirect::to(&result_uri).into_response()
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

async fn get_repo_version_v1(
    Path((forge, user, repo, version)): Path<(String, String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if version.ends_with("tar.gz") {
        let uri = format!(
            "https://{}.com/{}/{}/archive/refs/tags/{}.tar.gz",
            forge,
            user,
            repo,
            version.strip_suffix(".tar.gz").unwrap()
        );
        Redirect::to(&uri).into_response()
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

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

    let socket_addr: String = format!("{}:{}", config.addr.unwrap(), config.port.unwrap());

    debug!("{socket_addr:#?}");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/v1/:forge/:user/:repo/b/:branch", get(get_repo_branch_v1))
        .route(
            "/v1/:forge/:user/:repo/branch/:branch",
            get(get_repo_branch_v1),
        )
        .route(
            "/v1/:forge/:user/:repo/v/:version",
            get(get_repo_version_v1),
        )
        .route(
            "/v1/:forge/:user/:repo/version/:version",
            get(get_repo_version_v1),
        )
        .route(
            "/v1/:forge/:user/:repo/t/:version",
            get(get_repo_version_v1),
        )
        .route(
            "/v1/:forge/:user/:repo/tag/:version",
            get(get_repo_version_v1),
        )
        .route("/v1/:forge/:user/:repo", get(get_repo_v1));

    axum::Server::bind(&socket_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
