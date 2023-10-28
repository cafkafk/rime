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

pub async fn get_repo_ref(
    Path((host, user, repo, git_ref)): Path<(String, String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if git_ref.ends_with(".tar.gz") {
        let uri = format!(
            "https://{}/{}/{}/archive/{}.tar.gz",
            host,
            user,
            repo,
            git_ref
                .strip_suffix(".tar.gz")
                .expect("couldn't strip .tar.gz suffix")
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
