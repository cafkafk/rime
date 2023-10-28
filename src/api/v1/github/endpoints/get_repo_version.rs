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

pub async fn get_repo_version(
    Path((forge, user, repo, version)): Path<(String, String, String, String)>,
    request: Request<Body>,
) -> impl IntoResponse {
    if version.ends_with(".tar.gz") {
        let uri = format!(
            "https://{}.com/{}/{}/archive/refs/tags/{}.tar.gz",
            forge,
            user,
            repo,
            version
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
