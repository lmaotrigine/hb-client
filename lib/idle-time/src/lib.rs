// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::idle;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::idle;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::idle;

#[no_mangle]
pub fn idle_millis() -> i64 {
    idle().map(|i| i.as_millis() as i64).unwrap_or(-1)
}

#[derive(Debug)]
pub struct Error {
    pub cause: String,
}

impl std::error::Error for Error {}

impl Error {
    pub fn new<C: Into<String>>(cause: C) -> Error {
        Error {
            cause: cause.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}
