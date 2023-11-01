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

    Router::new()
        .merge(get_auto_discovery_routes())
        .nest("/forgejo", forgejo.get_routes())
        .nest("/gitea", forgejo.get_routes())
        .nest("/gitlab", Gitlab::new().get_routes())
        .nest("/sourcehut", SourceHut::new().get_routes())
        .nest("/github", github.get_flagship_routes())
        .nest("/github.com", github.get_flagship_routes())
        .nest("/flakehub", flakehub.get_flagship_routes())
        .nest("/flakehub.com", flakehub.get_flagship_routes())
        .merge(forgejo.get_redirect_routes())
}
