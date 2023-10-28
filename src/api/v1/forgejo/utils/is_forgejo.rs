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

pub async fn is_forgejo(host: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    // I couldn't find a more reasonable way to detect Forgejo, so we'll check
    // an API endpoint that is specific to this forge, and if it exists, we
    // assume it's a Forgejo instance.
    let uri = Url::parse(&format!("https://{}/api/v1/settings/api", &host))?;
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let res = client
        .get(uri)
        .header(ACCEPT, "application/json")
        .send()
        .await?;
    Ok(res.status() == 200)
}
