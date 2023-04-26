// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::OsString, path::PathBuf};

mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{cache_dir, config_dir};
#[cfg(target_os = "macos")]
pub use macos::{cache_dir, config_dir};
#[cfg(target_os = "windows")]
pub use windows::{cache_dir, config_dir};

fn absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

#[cfg(not(target_os = "windows"))]
fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from)
}

#[cfg(target_os = "windows")]
fn home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from)
}
