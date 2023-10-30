// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use super::super::super::utils::ForgeReleases;

pub async fn gitlab_api_get_releases(
    host: String,
    user: String,
    repo: String,
) -> Result<ForgeReleases, Box<dyn std::error::Error + Send + Sync>> {
    use reqwest::{
        header::{ACCEPT, USER_AGENT},
        Url,
    };
    // TODO: The middle part, `{}%2F{}` is really `user/repo` URL-encoded. We
    // should do proper URL encoding.
    let version_uri = Url::parse(&format!(
        "https://{}/api/v4/projects/{}%2F{}/releases?per_page=42",
        host, user, repo
    ))?;
    trace!("version_uri: {version_uri:#?}");
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let res = client
        .get(version_uri)
        .header(ACCEPT, "application/json")
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
