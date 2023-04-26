use std::path::PathBuf;

use super::home_dir;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub fn cache_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join("Library/Caches"))
}

pub fn config_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join("Library/Application Support"))
}
