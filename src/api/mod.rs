// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature="ssr")] {
pub mod routes;

mod v1;
}}
