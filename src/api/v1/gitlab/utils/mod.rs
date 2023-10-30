// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

mod gitlab_api_get_releases;
pub use self::gitlab_api_get_releases::gitlab_api_get_releases;
mod is_gitlab;
pub use self::is_gitlab::is_gitlab;
