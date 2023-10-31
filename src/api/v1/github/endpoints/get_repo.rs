// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    body::Body,
    extract::{Extension, Path},
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect},
};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::super::utils::github_api_get_releases;
use crate::data::Config;

pub async fn get_repo(
    Path((user, repo)): Path<(String, String)>,
    Extension(config): Extension<Config>,
    request: Request<Body>,
) -> impl IntoResponse {
    if repo.ends_with(".tar.gz") {
        let repo_name = repo
            .clone()
            .strip_suffix(".tar.gz")
            .expect("couldn't strip .tar.gz suffix")
            .to_string();
        let releases = github_api_get_releases(
            config.get_forge_api_page_size(),
            user.clone(),
            repo_name.clone(),
        )
        .await
        .expect("failed to await github_api_get_releases");
        if let Some(latest_release) = releases.latest_release(true) {
            let latest_tag = latest_release.tag_name;
            trace!("latest_tag: {latest_tag:#?}");

            let redirect_url =
                format!("/v1/github/{}/{}/v/{}.tar.gz", user, repo_name, latest_tag,);
            Redirect::to(&redirect_url).into_response()
        } else {
            let body = format!(
                "Hi friend, no releases found for github.com/{}/{} :(",
                user, repo_name,
            );
            (StatusCode::NOT_FOUND, body).into_response()
        }
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
