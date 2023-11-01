// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReleaseQueryParams {
    include_prereleases: Option<bool>,
}

impl ReleaseQueryParams {
    pub fn include_prereleases(&self) -> bool {
        self.include_prereleases.unwrap_or(false)
    }
}
