// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::NaiveDateTime;

use crate::Result;

#[derive(Debug)]
pub struct Logger {
    file: File,
}

fn now() -> NaiveDateTime {
    NaiveDateTime::from_timestamp_micros(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as _,
    )
    .unwrap()
}
impl Logger {
    pub fn try_new(dir: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("heartbeat.log"))?;
        Ok(Self { file })
    }

    pub fn log(&mut self, msg: &str) -> Result {
        let msg = format!("{} {msg}\n", now().format("%Y-%m-%d %H:%M:%S"));
        self.file.write_all(msg.as_bytes())?;
        Ok(())
    }
}
