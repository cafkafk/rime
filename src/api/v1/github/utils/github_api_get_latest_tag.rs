// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub async fn github_api_get_latest_tag(
    user: String,
    repo: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use reqwest::{
        header::{ACCEPT, USER_AGENT},
        Url,
    };
    let version_uri = Url::parse(&format!(
        "http://api.github.com/repos/{}/{}/releases?per_page=1",
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

    trace!("got:\n {:#?}", res[0]["tag_name"]);

    Ok(res[0]["tag_name"]
        .as_str()
        .expect("failed to get release tag-name as_str()")
        .to_string())
}
