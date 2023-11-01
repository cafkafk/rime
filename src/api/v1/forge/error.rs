// SPDX-FileCopyrightText: 2023 Gergely Nagy
// SPDX-FileContributor: Gergely Nagy
//
// SPDX-License-Identifier: AGPL-3.0-only

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub enum ForgeError {
    RequestError(reqwest::Error),
    EndpointUnavailable,
    NoFlagshipInstance,
    AutodetectFailed(String),
    SemverError(semver::Error),
}

impl IntoResponse for ForgeError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ForgeError::RequestError(error) => {
                error!("ForgeError::RequestError: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error communicating with the remote server".to_string(),
                )
            }
            ForgeError::EndpointUnavailable => {
                // No need to log this, this isn't technically an error.
                (
                    StatusCode::NOT_FOUND,
                    "endpoint not available for this forge".to_string(),
                )
            }
            ForgeError::NoFlagshipInstance => {
                error!("ForgeError::NoFlagshipInstanceError");
                (
                    StatusCode::NOT_FOUND,
                    "flagship instance unavailable for this forge".to_string(),
                )
            }
            ForgeError::AutodetectFailed(host) => {
                error!("ForgeError::AutodetectFailed: host: {host}");
                (
                    StatusCode::BAD_REQUEST,
                    format!("failed to auto-detect the forge on {}", host),
                )
            }
            ForgeError::SemverError(error) => {
                error!("ForgeError::SemverError: {error}");
                (StatusCode::BAD_REQUEST, error.to_string())
            }
        };
        (status, error_message).into_response()
    }
}

impl From<reqwest::Error> for ForgeError {
    fn from(error: reqwest::Error) -> ForgeError {
        ForgeError::RequestError(error)
    }
}

impl From<semver::Error> for ForgeError {
    fn from(error: semver::Error) -> ForgeError {
        ForgeError::SemverError(error)
    }
}
