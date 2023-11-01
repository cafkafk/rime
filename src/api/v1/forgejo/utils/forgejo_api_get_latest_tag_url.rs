// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub async fn forgejo_api_get_latest_tag_url(
    host: String,
    user: String,
    repo: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use reqwest::{
        header::{ACCEPT, USER_AGENT},
        Url,
    };
    let version_uri = Url::parse(&format!(
        "http://{}/api/v1/repos/{}/{}/releases?limit=1",
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

    trace!("got:\n {:#?}", res[0]["tarball_url"]);

    Ok(res[0]["tarball_url"]
        .as_str()
        .expect("failed to get release tarball_url as_str()")
        .to_string())
}
