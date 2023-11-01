// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};
use reqwest::Url;

use super::super::super::utils::ForgeReleases;

pub async fn forgejo_api_get_releases(
    page_size: u8,
    host: String,
    user: String,
    repo: String,
) -> Result<ForgeReleases, Box<dyn std::error::Error + Send + Sync>> {
    let releases_uri = Url::parse(&format!(
        "http://{}/api/v1/repos/{}/{}/releases?limit={}",
        host, user, repo, page_size
    ))?;
    ForgeReleases::from_url(releases_uri).await
}
