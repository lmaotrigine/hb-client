use std::path::PathBuf;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{absolute_path, home_dir};

pub fn cache_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_CACHE_HOME")
        .and_then(absolute_path)
        .or_else(|| home_dir().map(|h| h.join(".cache")))
}

pub fn config_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .and_then(absolute_path)
        .or_else(|| home_dir().map(|h| h.join(".config")))
}
