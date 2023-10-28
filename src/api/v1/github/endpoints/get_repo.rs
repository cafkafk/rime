// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect},
};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::super::utils::github_api_get_latest_tag;

pub async fn get_repo(
    Path((forge, user, repo)): Path<(String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if repo.ends_with(".tar.gz") {
        let version = github_api_get_latest_tag(
            user.clone(),
            repo.clone()
                .strip_suffix(".tar.gz")
                .expect("couldn't strip .tar.gz suffix")
                .to_string(),
        )
        .await
        .expect("failed to await github_api_get_latest_tag");
        let result_uri = format!(
            "http://{}.com/{}/{}/archive/refs/tags/{}.tar.gz",
            forge,
            user,
            repo.strip_suffix(".tar.gz")
                .expect("couldn't strip .tar.gz suffix"),
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
