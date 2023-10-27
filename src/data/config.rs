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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The address of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// The port of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
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
            },
        }
    }
    pub fn _gen_example_config(&self) -> Result<(), Error> {
        let data = serde_yaml::to_string(&self).unwrap();
        write(CONFIG, data)
    }
}
