// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fs,
    path::{Path, PathBuf},
};

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use ureq::Request;

use crate::{
    util::{cache_dir, config_dir},
    Result,
};

#[derive(Debug, Clone)]
pub struct Config {
    token: String,
    base_url: String,
    #[cfg(feature = "5ht2b")]
    device_name: String,
    log_dir: PathBuf,
}

impl Config {
    pub fn try_new() -> Result<Self> {
        let figment = Figment::from(Toml::file(
            config_dir()
                .unwrap_or_default()
                .join("heartbeat/config.toml"),
        ))
        .merge(Env::prefixed("HEARTBEAT_"));
        let log_dir = if let Ok(s) = figment.extract_inner::<String>("log_dir") {
            PathBuf::from(s)
        } else {
            cache_dir().unwrap_or_default().join("heartbeat")
        };
        fs::create_dir_all(&log_dir)?;
        Ok(Self {
            token: figment.extract_inner("token")?,
            base_url: figment.extract_inner("base_url")?,
            #[cfg(feature = "5ht2b")]
            device_name: figment.extract_inner("device_name")?,
            log_dir,
        })
    }

    pub fn log_dir(&self) -> &Path {
        self.log_dir.as_path()
    }

    #[cfg(feature = "5ht2b")]
    pub fn request(&self) -> Request {
        ureq::post(&format!("{}/api/beat", self.base_url))
            .set("Auth", &self.token)
            .set("Device", &self.device_name)
    }

    #[cfg(not(feature = "5ht2b"))]
    pub fn request(&self) -> Request {
        ureq::post(&format!("{}/api/beat", self.base_url)).set("Authorization", &self.token)
    }
}
