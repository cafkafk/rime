// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature="ssr")] {
use super::forge;
use super::{AutoDiscover, DynForge, FlakeHub, Forge, Forgejo, GitHub, Gitlab, SourceHut};
use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};

fn get_forge_routes(forge: impl Forge + Send + Sync + 'static) -> Router {
    let forge = Arc::new(forge) as DynForge;
    Router::new()
        .route(
            "/:user/:repo",
            get(forge::handlers::get_tarball_url_for_latest_release),
        )
        .route(
            "/:user/:repo/branch/*branch",
            get(forge::handlers::get_tarball_url_for_branch),
        )
        .route(
            "/:user/:repo/b/*branch",
            get(forge::handlers::get_tarball_url_for_branch),
        )
        .route(
            "/:user/:repo/version/:version",
            get(forge::handlers::get_tarball_url_for_version),
        )
        .route(
            "/:user/:repo/v/:version",
            get(forge::handlers::get_tarball_url_for_version),
        )
        .route(
            "/:user/:repo/tag/:version",
            get(forge::handlers::get_tarball_url_for_version),
        )
        .route(
            "/:user/:repo/t/:version",
            get(forge::handlers::get_tarball_url_for_version),
        )
        .route(
            "/:user/:repo/semver/:version",
            get(forge::handlers::get_tarball_url_for_semantic_version),
        )
        .route(
            "/:user/:repo/s/:version",
            get(forge::handlers::get_tarball_url_for_semantic_version),
        )
        .layer(Extension(forge))
}

fn get_self_hosted_forge_routes(forge: impl Forge + Send + Sync + 'static) -> Router {
    Router::new().nest("/:host", get_forge_routes(forge))
}

pub fn get_routes() -> Router {
    Router::new()
        // --- Forgejo & Gitea
        .nest("/forgejo", get_self_hosted_forge_routes(Forgejo))
        .nest("/gitea", get_self_hosted_forge_routes(Forgejo))
        .nest("/codeberg", get_forge_routes(Forgejo))
        .nest("/codeberg.org", get_forge_routes(Forgejo))
        // --- Gitlab
        // For Gitlab, we do not provide a flagship instance, because gitlab.com
        // is behind a login wall. Thus, gitlab.com must be explicitly
        // specified, as if it was self-hosted.
        .nest("/gitlab", get_self_hosted_forge_routes(Gitlab))
        // --- SourceHut
        .nest("/sourcehut", get_self_hosted_forge_routes(SourceHut))
        .nest("/sr.ht", get_forge_routes(SourceHut))
        .nest("/git.sr.ht", get_forge_routes(SourceHut))
        // --- GitHub
        .nest("/github", get_forge_routes(GitHub))
        .nest("/github.com", get_forge_routes(GitHub))
        // --- FlakeHub
        .nest("/flakehub", get_forge_routes(FlakeHub))
        .nest("/flakehub.com", get_forge_routes(FlakeHub))
        // --- Automatic discovery
        .nest("/:host", get_forge_routes(AutoDiscover))
}

}}
