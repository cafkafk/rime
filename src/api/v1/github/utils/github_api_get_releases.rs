// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};
use reqwest::Url;

use super::super::super::utils::ForgeReleases;

pub async fn github_api_get_releases(
    page_size: u8,
    user: String,
    repo: String,
) -> Result<ForgeReleases, Box<dyn std::error::Error + Send + Sync>> {
    let releases_uri = Url::parse(&format!(
        "http://api.github.com/repos/{}/{}/releases?per_page={}",
        user, repo, page_size
    ))?;
    ForgeReleases::from_url(releases_uri).await
}
