// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use reqwest::{
    header::{ACCEPT, USER_AGENT},
    Url,
};

pub async fn is_gitlab(host: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    // Detecting GitLab is a bit tricky: while it has a /version and a /metadata
    // endpoint, both require authentication, so any data they may return is
    // unusable. Thankfully, every request that hits the GitLab API has an
    // `x-gitlab-meta` header, that's a strong enough indication that we're
    // dealing with GitLab.

    let uri = Url::parse(&format!("https://{}/api/v4/projects?per_page=1", &host))?;
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let res = client
        .get(uri)
        .header(ACCEPT, "application/json")
        .send()
        .await?;
    Ok(res.status() == 200 && res.headers().contains_key("x-gitlab-meta"))
}
