// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    UReq(Box<ureq::Error>), // this is big, hence the box
    Io(std::io::Error),
    Figment(figment::Error),
    Ffi(idle_time::Error),
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Error::UReq(Box::new(e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<figment::Error> for Error {
    fn from(e: figment::Error) -> Self {
        Error::Figment(e)
    }
}

impl From<idle_time::Error> for Error {
    fn from(e: idle_time::Error) -> Self {
        Error::Ffi(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UReq(ref e) => write!(f, "ureq error: {e}"),
            Self::Io(ref e) => write!(f, "io error: {e}"),
            Self::Figment(ref e) => write!(f, "figment error: {e}"),
            Self::Ffi(ref e) => write!(f, "ffi error: {e}"),
        }
    }
}
