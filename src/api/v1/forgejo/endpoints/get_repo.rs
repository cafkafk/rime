// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
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

use super::super::utils::forgejo_api_get_latest_tag_url;

pub async fn get_repo(
    Path((host, user, repo)): Path<(String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if repo.ends_with(".tar.gz") {
        let latest_tag_url = forgejo_api_get_latest_tag_url(
            host.clone(),
            user.clone(),
            repo.clone()
                .strip_suffix(".tar.gz")
                .expect("couldn't strip .tar.gz suffix")
                .to_string(),
        )
        .await
        .expect("failed to await forgejo_api_get_latest_tag_url");
        trace!("latest_tag_url: {latest_tag_url:#?}");
        Redirect::to(&latest_tag_url).into_response()
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
