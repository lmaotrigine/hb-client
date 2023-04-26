// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;

use crate::Error;

extern "stdcall" {
    fn GetTickCount() -> u32;
    fn GetLastInputInfo(plii: *mut LastInputInfo) -> i32;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct LastInputInfo {
    cb_size: u32,
    dw_time: u32,
}

impl Default for LastInputInfo {
    fn default() -> Self {
        Self {
            cb_size: std::mem::size_of::<Self>() as u32,
            dw_time: 0,
        }
    }
}

pub fn idle() -> Result<Duration, Error> {
    let mut last_input_info = LastInputInfo::default();

    let res = unsafe { GetLastInputInfo(&mut last_input_info) };
    match res {
        0 => Err(Error::new("failed to get last input info")),
        _ => {
            let now = unsafe { GetTickCount() };
            Ok(Duration::from_millis((now - last_input_info.dw_time) as _))
        }
    }
}
