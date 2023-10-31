#![allow(clippy::type_complexity)]

// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use std::fs::{self, write};
use std::io::Error;

use log::*;

pub const CONFIG: &str = "config.yaml";

const ADDR: &str = "0.0.0.0";
const PORT: &str = "3000";
const DEFAULT_FORGE_API_PAGE_SIZE: u8 = 10;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// The address of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    addr: Option<String>,
    /// The port of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forges: Option<ConfigForges>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigForges {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_page_size: Option<u8>,
}

impl Config {
    /// Loads the configuration toml from a path into the Config struct.
    #[inline]
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("initializing new Config struct");
        let yaml = fs::read_to_string(path)?;

        debug!("deserialized yaml from config file");
        let config = serde_yaml::from_str(&yaml)?;

        Ok(config)
    }
    #[inline]
    pub fn load(path: &str) -> Self {
        trace!("path: {path:#?}");
        match Self::new(path) {
            Ok(config) => config,
            Err(_) => Config {
                addr: Some(ADDR.to_string()),
                port: Some(PORT.to_string()),
                forges: Some(ConfigForges {
                    api_page_size: Some(DEFAULT_FORGE_API_PAGE_SIZE),
                }),
            },
        }
    }
    pub fn bind_addr(&self) -> std::net::SocketAddr {
        let socket_addr: String = format!(
            "{}:{}",
            self.addr.clone().unwrap_or(ADDR.to_string()),
            self.port.clone().unwrap_or(PORT.to_string())
        );

        debug!("socket_addr: {socket_addr:#?}");

        socket_addr
            .parse()
            .expect("failed to parse the bind address")
    }

    pub fn _gen_example_config(&self) -> Result<(), Error> {
        let data = serde_yaml::to_string(&self).expect("failed to deserialize self to yaml");
        write(CONFIG, data)
    }

    pub fn get_forge_api_page_size(&self) -> u8 {
        match &self.forges {
            Some(cfg) => cfg.api_page_size.unwrap_or(DEFAULT_FORGE_API_PAGE_SIZE),
            _ => DEFAULT_FORGE_API_PAGE_SIZE,
        }
    }
}
