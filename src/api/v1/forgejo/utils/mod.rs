// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

mod forgejo_api_get_releases;
pub use self::forgejo_api_get_releases::forgejo_api_get_releases;
mod is_forgejo;
pub use self::is_forgejo::is_forgejo;
