// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature="ssr")] {
mod auto_discover;
pub use auto_discover::AutoDiscover;
mod flakehub;
pub use flakehub::FlakeHub;
mod forgejo;
pub use forgejo::Forgejo;
mod github;
pub use github::GitHub;
mod gitlab;
pub use gitlab::Gitlab;
mod sourcehut;
pub use sourcehut::SourceHut;
}}
