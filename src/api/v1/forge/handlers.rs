// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    body::Body,
    extract::{Extension, Path, Query},
    http::Request,
    response::Response,
};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::{DynForge, ForgeError, ForgeReleases};
use crate::api::v1::utils::ReleaseQueryParams;
use crate::data::Config;

mod internal;

// -- get_tarball_url_for_latest_release

pub async fn get_tarball_url_for_latest_release(
    Path((host, user, repo)): Path<(String, String, String)>,
    Extension(forge): Extension<DynForge>,
    Extension(config): Extension<Config>,
    params: Query<ReleaseQueryParams>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    internal::get_tarball_url_for_latest_release((host, user, repo), forge, config, params, request)
        .await
}

pub async fn get_tarball_url_for_latest_release_from_flagship(
    Path((user, repo)): Path<(String, String)>,
    Extension(forge): Extension<DynForge>,
    Extension(config): Extension<Config>,
    params: Query<ReleaseQueryParams>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    internal::get_tarball_url_for_latest_release(
        (forge.get_flagship_host()?, user, repo),
        forge,
        config,
        params,
        request,
    )
    .await
}

// get_tarball_url_for_branch

pub async fn get_tarball_url_for_branch(
    Path((host, user, repo, branch)): Path<(String, String, String, String)>,
    Extension(forge): Extension<DynForge>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    internal::get_tarball_url_for_branch((host, user, repo, branch), forge, request).await
}

pub async fn get_tarball_url_for_branch_from_flagship(
    Path((user, repo, branch)): Path<(String, String, String)>,
    Extension(forge): Extension<DynForge>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    internal::get_tarball_url_for_branch(
        (forge.get_flagship_host()?, user, repo, branch),
        forge,
        request,
    )
    .await
}

// get_tarball_url_for_version

pub async fn get_tarball_url_for_version(
    Path((host, user, repo, version)): Path<(String, String, String, String)>,
    Extension(forge): Extension<DynForge>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    internal::get_tarball_url_for_version((host, user, repo, version), forge, request).await
}

pub async fn get_tarball_url_for_version_from_flagship(
    Path((user, repo, version)): Path<(String, String, String)>,
    Extension(forge): Extension<DynForge>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    internal::get_tarball_url_for_version(
        (forge.get_flagship_host()?, user, repo, version),
        forge,
        request,
    )
    .await
}
