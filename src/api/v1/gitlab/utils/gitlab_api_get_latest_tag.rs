// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub async fn gitlab_api_get_latest_tag(
    host: String,
    user: String,
    repo: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use reqwest::{
        header::{ACCEPT, USER_AGENT},
        Url,
    };
    // TODO: The middle part, `{}%2F{}` is really `user/repo` URL-encoded. We
    // should do proper URL encoding.
    let version_uri = Url::parse(&format!(
        "https://{}/api/v4/projects/{}%2F{}/releases",
        host, user, repo
    ))?;
    trace!("{:#?}", version_uri);
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let res = client
        .get(version_uri)
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    trace!("got:\n {:#?}", res[0]["tag_name"]);

    Ok(res[0]["tag_name"]
        .as_str()
        .expect("failed to get release name as_str()")
        .to_string())
}
