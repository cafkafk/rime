// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub async fn codeberg_redirect(Path(url): Path<String>) -> impl IntoResponse {
    let target = format!("/v1/forgejo/codeberg.org/{}", url);
    trace!("target: {target:#?}");
    Redirect::to(&target).into_response()
}
