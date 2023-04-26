mod config;
mod error;
mod log;
mod util;

use std::time::Duration;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use config::Config;
use error::Error;
use log::Logger;

type Result<T = ()> = std::result::Result<T, Error>;

const THRESHOLD: Duration = Duration::from_secs(120);

fn main() -> Result {
    let config = Config::try_new()?;
    let mut logger = Logger::try_new(config.log_dir())?;
    if idle_time::idle()? < THRESHOLD {
        let res = config.request().call()?;
        logger.log(&format!("{}: {}", res.status(), res.into_string()?))?;
    }
    Ok(())
}
