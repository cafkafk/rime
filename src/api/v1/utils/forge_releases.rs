// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use log::trace;
use std::slice::Iter;

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
}
