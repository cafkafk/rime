// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::Router;

use super::{AutoDiscover, FlakeHub, Forge, Forgejo, GitHub, Gitlab, SourceHut};

pub fn get_routes() -> Router {
    let forgejo = Forgejo::new();
    let github = GitHub::new();
    let gitlab = Gitlab::new();
    let flakehub = FlakeHub::new();
    let sourcehut = SourceHut::new();

    Router::new()
        // --- Forgejo & Gitea
        .nest("/forgejo", forgejo.get_self_hosted_routes())
        .nest("/gitea", forgejo.get_self_hosted_routes())
        .nest("/codeberg", forgejo.get_routes())
        .nest("/codeberg.org", forgejo.get_routes())
        // --- Gitlab
        // For Gitlab, we do not provide a flagship instance, because gitlab.com
        // is behind a login wall. Thus, gitlab.com must be explicitly
        // specified, as if it was self-hosted.
        .nest("/gitlab", gitlab.get_self_hosted_routes())
        // --- SourceHut
        .nest("/sourcehut", sourcehut.get_self_hosted_routes())
        .nest("/sr.ht", sourcehut.get_routes())
        .nest("/git.sr.ht", sourcehut.get_routes())
        // --- GitHub
        .nest("/github", github.get_routes())
        .nest("/github.com", github.get_routes())
        // --- FlakeHub
        .nest("/flakehub", flakehub.get_routes())
        .nest("/flakehub.com", flakehub.get_routes())
        // --- Automatic discovery
        .nest("/:host", AutoDiscover::new().get_routes())
}
