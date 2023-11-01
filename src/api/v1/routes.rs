// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::Router;

use super::auto_discovery::routes::get_routes as get_auto_discovery_routes;

use super::{FlakeHub, Forge, Forgejo, GitHub, Gitlab, SourceHut};

pub fn get_routes() -> Router {
    let forgejo = Forgejo::new();
    let github = GitHub::new();
    let flakehub = FlakeHub::new();
    let sourcehut = SourceHut::new();

    Router::new()
        .merge(get_auto_discovery_routes())
        .nest("/forgejo", forgejo.get_routes())
        .nest("/gitea", forgejo.get_routes())
        .nest("/codeberg", forgejo.get_flagship_routes())
        .nest("/codeberg.org", forgejo.get_flagship_routes())
        .nest("/gitlab", Gitlab::new().get_routes())
        .nest("/sourcehut", sourcehut.get_routes())
        .nest("/sr.ht", sourcehut.get_flagship_routes())
        .nest("/git.sr.ht", sourcehut.get_flagship_routes())
        .nest("/github", github.get_flagship_routes())
        .nest("/github.com", github.get_flagship_routes())
        .nest("/flakehub", flakehub.get_flagship_routes())
        .nest("/flakehub.com", flakehub.get_flagship_routes())
}
