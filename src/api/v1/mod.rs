// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

pub mod routes;

mod forge;
pub use forge::{DynForge, Forge, ForgeError};
mod forges;
pub use forges::{AutoDiscover, FlakeHub, Forgejo, GitHub, Gitlab, SourceHut};

pub mod utils;
