// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::super::super::utils::ForgeReleases;

pub async fn github_api_get_releases(
    user: String,
    repo: String,
) -> Result<ForgeReleases, Box<dyn std::error::Error + Send + Sync>> {
    use reqwest::{
        header::{ACCEPT, USER_AGENT},
        Url,
    };
    let version_uri = Url::parse(&format!(
        "http://api.github.com/repos/{}/{}/releases?per_page=42",
        user, repo
    ))?;
    trace!("{:#?}", version_uri);
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let res = client
        .get(version_uri)
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let releases = if res.is_array() {
        ForgeReleases::from(
            res.as_array()
                .expect("Failed to unwrap releases API response as_array()")
                .iter(),
        )
    } else {
        ForgeReleases::new()
    };
    trace!("releases: {releases:#?}");

    Ok(releases)
}
