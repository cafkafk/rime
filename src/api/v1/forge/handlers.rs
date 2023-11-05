// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    extract::{Extension, OriginalUri, Path, Query},
    http::Uri,
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::collections::HashMap;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::{DynForge, ForgeError, ForgeRelease, ForgeReleases};
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

fn try_strip_targz_suffix(part: &str, uri: &Uri) -> Result<String, ForgeError> {
    if part.ends_with(".tar.gz") {
        Ok(part
            .strip_suffix(".tar.gz")
            .expect("couldn't strip .tar.gz suffix")
            .to_string())
    } else {
        Err(ForgeError::NoTarGz(uri.to_string()))
    }
}

async fn try_get_release_url<F>(
    redirect_url: Option<String>,
    forge: &DynForge,
    host: &str,
    user: &str,
    repo: &str,
    page_size: u8,
    reduce: F,
) -> Result<String, ForgeError>
where
    F: FnOnce(ForgeReleases) -> Option<ForgeRelease>,
{
    if let Some(url) = redirect_url {
        return Ok(url);
    }

    let api_releases_url = forge
        .get_api_releases_url(host, user, repo, page_size)
        .await?;
    trace!("api_releases_url: {}", api_releases_url);
    let releases = ForgeReleases::from_url(api_releases_url).await?;

    if let Some(release) = reduce(releases) {
        let latest_tag = release.tag_name;
        trace!("latest_tag: {latest_tag}");
        Ok(forge
            .get_tarball_url_for_version(host, user, repo, &latest_tag)
            .await?)
    } else {
        Err(ForgeError::NoReleaseFound(
            forge.get_repo_url(host, user, repo).await?,
        ))
    }
}

pub async fn get_tarball_url_for_latest_release(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    Extension(config): Extension<Config>,
    params: Query<ReleaseQueryParams>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;
    let repo = try_strip_targz_suffix(&repo, &original_uri)?;

    let redirect_url = try_get_release_url(
        forge
            .get_tarball_url_for_latest_release(&host, &user, &repo)
            .await?,
        &forge,
        &host,
        &user,
        &repo,
        config.get_forge_api_page_size(),
        |releases| releases.latest_release(params.include_prereleases()),
    )
    .await?;
    trace!("tarball_url_for_latest_release: {redirect_url:}");
    Ok(Redirect::to(&redirect_url).into_response())
}

#[derive(Deserialize, Debug)]
pub struct SemverQuery {
    pub version: Option<String>,
}

pub async fn get_tarball_url_for_semantic_version(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    Extension(config): Extension<Config>,
    params: Query<SemverQuery>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;
    let version = if let Some(version) = &params.version {
        format!("{version}.tar.gz")
    } else if let Some(version) = &paths.get("version") {
        version.to_string()
    } else {
        return Err(ForgeError::BadRequest(
            "No semantic version requirement specified".to_string(),
        ));
    };
    let version = try_strip_targz_suffix(&version, &original_uri)?;

    let v = semver::VersionReq::parse(&version)?;
    let redirect_url = try_get_release_url(
        forge
            .get_tarball_url_for_semantic_version(&host, &user, &repo, &version)
            .await?,
        &forge,
        &host,
        &user,
        &repo,
        config.get_forge_api_page_size(),
        |releases| releases.matching(v),
    )
    .await?;
    trace!("tarball_url_for_semantic_version: {redirect_url}");
    Ok(Redirect::to(&redirect_url).into_response())
}

pub async fn get_tarball_url_for_branch(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;
    let branch = try_strip_targz_suffix(&paths["branch"], &original_uri)?;
    let url = forge
        .get_tarball_url_for_branch(&host, &user, &repo, &branch)
        .await?;
    trace!("tarball_url_for_branch: {url:}");
    Ok(Redirect::to(&url).into_response())
}

pub async fn get_tarball_url_for_version(
    Path(paths): Path<HashMap<String, String>>,
    Extension(forge): Extension<DynForge>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<Response, ForgeError> {
    let (host, user, repo) = get_host_user_repo_triplet(&paths, &forge).await?;
    let version = try_strip_targz_suffix(&paths["version"], &original_uri)?;
    let url = forge
        .get_tarball_url_for_version(&host, &user, &repo, &version)
        .await?;
    trace!("tarball_url_for_version: {url:}");
    Ok(Redirect::to(&url).into_response())
}
