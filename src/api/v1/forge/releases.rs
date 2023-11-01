// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use log::trace;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::slice::Iter;

use super::ForgeError;

#[derive(Clone, Debug)]
pub struct ForgeRelease {
    pub tag_name: String,
    pub prerelease: bool,
}

#[derive(Clone, Debug)]
pub struct ForgeReleases(Vec<ForgeRelease>);

impl ForgeReleases {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from(response: Iter<serde_json::Value>) -> Self {
        Self(
            response
                .map(|release| {
                    let release = release
                        .as_object()
                        .expect("failed to get the release as_object()");
                    ForgeRelease {
                        tag_name: release["tag_name"]
                            .as_str()
                            .expect("failed to get release tag name as_str()")
                            .to_string(),
                        prerelease: release
                            .get("prerelease")
                            .map(|v| {
                                v.as_bool()
                                    .expect("failed to get prerelease status as_bool()")
                            })
                            .unwrap_or(false),
                    }
                })
                .collect(),
        )
    }

    pub fn latest_release(self, include_prereleases: bool) -> Option<ForgeRelease> {
        trace!("include_prereleases: {include_prereleases:#?}");
        if include_prereleases {
            self.0.first().cloned()
        } else {
            self.0.clone().into_iter().find(|rel| !rel.prerelease)
        }
    }

    fn try_version_from_tag(&self, repo: &str, ver: &str) -> Result<semver::Version, ForgeError> {
        // v<ver> => <repo>- => <ver>
        let v = if ver.starts_with('v') {
            ver.strip_prefix('v')
                .expect("couldn't strip the `v` prefix")
        } else if ver.starts_with(&format!("{repo}-")) {
            ver.strip_prefix(&format!("{repo}-"))
                .expect("couldn't strip the `{repo}-` prefix")
        } else {
            ver
        };

        Ok(semver::Version::parse(v)?)
    }

    pub fn matching(self, repo: &str, version_req: semver::VersionReq) -> Option<ForgeRelease> {
        self.0.clone().into_iter().find(|v| {
            trace!("trying to match {} against {}", &v.tag_name, version_req);
            match self.try_version_from_tag(repo, &v.tag_name) {
                Ok(version) => version_req.matches(&version),
                _ => false,
            }
        })
    }

    pub async fn from_url(releases_url: String) -> Result<Self, ForgeError> {
        trace!("releases_url: {releases_url:#?}");

        let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        let res = client
            .get(releases_url)
            .header(ACCEPT, "application/vnd.github+json, application/json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let releases = if res.is_array() {
            Self::from(
                res.as_array()
                    .expect("Failed to unwrap releases API response as_array()")
                    .iter(),
            )
        } else {
            Self::new()
        };
        trace!("releases: {releases:#?}");

        Ok(releases)
    }
}
