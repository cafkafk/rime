// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    body::Body,
    extract::{Extension, Path, Query},
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use std::collections::HashMap;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::{DynForge, ForgeError, ForgeReleases};
use crate::api::v1::utils::ReleaseQueryParams;
use crate::data::Config;

async fn get_host_user_repo_triplet(
    paths: &HashMap<String, String>,
    forge: &DynForge,
) -> Result<(String, String, String), ForgeError> {
    let (user, repo) = (paths["user"].clone(), paths["repo"].clone());
    let host = if let Some(host) = &paths.get("host") {
        host.to_string()
    } else {
        forge.get_flagship_host().await?
    };

    Ok((host, user, repo))
}

pub async fn get_tarball_url_for_latest_release(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    Extension(config): Extension<Config>,
    params: Query<ReleaseQueryParams>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;

    if repo.ends_with(".tar.gz") {
        let repo_name = repo
            .clone()
            .strip_suffix(".tar.gz")
            .expect("couldn't strip .tar.gz suffix")
            .to_string();
        let api_releases_url = forge
            .get_api_releases_url(&host, &user, &repo_name, config.get_forge_api_page_size())
            .await?;
        trace!("api_releases_url: {}", api_releases_url);
        let releases = ForgeReleases::from_url(api_releases_url).await?;

        if let Some(latest_release) = releases.latest_release(params.include_prereleases()) {
            let latest_tag = latest_release.tag_name;
            trace!("latest_tag: {latest_tag:}");

            let redirect_url = forge
                .get_tarball_url_for_version(&host, &user, &repo_name, &latest_tag)
                .await?;
            trace!("tarball_url_for_latest_release: {redirect_url:}");
            Ok(Redirect::to(&redirect_url).into_response())
        } else {
            let body = format!(
                "Hi friend, no releases found for {} :(",
                forge.get_repo_url(&host, &user, &repo_name).await?
            );
            Ok((StatusCode::NOT_FOUND, body).into_response())
        }
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        Ok((StatusCode::BAD_REQUEST, body).into_response())
    }
}

pub async fn get_tarball_url_for_branch(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;
    let branch = &paths["branch"];

    if branch.ends_with(".tar.gz") {
        let url = forge
            .get_tarball_url_for_branch(
                &host,
                &user,
                &repo,
                branch
                    .strip_suffix(".tar.gz")
                    .expect("couldn't strip .tar.gz suffix"),
            )
            .await?;
        trace!("tarball_url_for_branch: {url:}");
        Ok(Redirect::to(&url).into_response())
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        Ok((StatusCode::BAD_REQUEST, body).into_response())
    }
}

pub async fn get_tarball_url_for_version(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    request: Request<Body>,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;
    let version = &paths["version"];

    if version.ends_with(".tar.gz") {
        let url = forge
            .get_tarball_url_for_version(
                &host,
                &user,
                &repo,
                version
                    .strip_suffix(".tar.gz")
                    .expect("couldn't strip .tar.gz suffix"),
            )
            .await?;
        trace!("tarball_url_for_version: {url:}");
        Ok(Redirect::to(&url).into_response())
    } else {
        let body = format!(
            "Hi friend, you probably meant to request {:#?}{}.tar.gz, that should work <3",
            request.headers()["host"],
            request.uri()
        );
        Ok((StatusCode::BAD_REQUEST, body).into_response())
    }
}
