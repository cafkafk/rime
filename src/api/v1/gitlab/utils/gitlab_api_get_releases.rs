// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};
use reqwest::Url;

use super::super::super::utils::ForgeReleases;

pub async fn gitlab_api_get_releases(
    page_size: u8,
    host: String,
    user: String,
    repo: String,
) -> Result<ForgeReleases, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: The middle part, `{}%2F{}` is really `user/repo` URL-encoded. We
    // should do proper URL encoding.
    let releases_uri = Url::parse(&format!(
        "https://{}/api/v4/projects/{}%2F{}/releases?per_page={}",
        host, user, repo, page_size
    ))?;
    ForgeReleases::from_url(releases_uri).await
}
