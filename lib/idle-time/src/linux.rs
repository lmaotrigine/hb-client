// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Error;
use std::{
    ffi::{c_char, c_int, c_ulong, c_void},
    time::Duration,
};

const SCREENSAVERS: &[&[&str; 3]; 3] = &[
    &[
        "org.freedesktop.ScreenSaver",
        "/ScreenSaver",
        "org.freedesktop.ScreenSaver",
    ],
    &[
        "org.gnome.ScreenSaver",
        "/ScreenSaver",
        "org.gnome.ScreenSaver",
    ],
    &[
        "org.kde.ScreenSaver",
        "/ScreenSaver",
        "org.freedesktop.ScreenSaver",
    ],
];

pub fn is_locked() -> bool {
    for scrnsaver in SCREENSAVERS {
        let Ok(conn) = dbus::blocking::Connection::new_session() else {
            continue;
        };
        let proxy = conn.with_proxy(scrnsaver[0], scrnsaver[1], Duration::from_millis(5000));
        let (res,): (bool,) = match proxy.method_call(scrnsaver[2], "GetActive", ()) {
            Ok(r) => r,
            Err(_) => continue,
        };
        return res;
    }
    false // boo don't judge me
}

pub enum Display {}
pub type Window = c_ulong;
pub type Drawable = c_ulong;
pub type Status = c_int;

extern "C" {
    pub fn XOpenDisplay(_1: *const c_char) -> *mut Display;
    pub fn XDefaultScreen(_1: *mut Display) -> c_int;
    pub fn XRootWindow(_2: *mut Display, _1: c_int) -> c_ulong;
    pub fn XFree(_1: *mut c_void) -> c_int;
    pub fn XCloseDisplay(_1: *mut Display) -> c_int;
    pub fn XScreenSaverAllocInfo() -> *mut XScreenSaverInfo;
    pub fn XScreenSaverQueryInfo(
        _1: *mut Display,
        _2: Drawable,
        _3: *mut XScreenSaverInfo,
    ) -> Status;
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XScreenSaverInfo {
    pub window: Window,
    pub state: c_int,
    pub kind: c_int,
    pub til_or_since: c_ulong,
    pub idle: c_ulong,
    pub eventMask: c_ulong,
}

pub fn idle() -> Result<Duration, Error> {
    if is_locked() {
        return Ok(Duration::MAX);
    }
    unsafe {
        let info = XScreenSaverAllocInfo();
        let display = XOpenDisplay(0 as _);
        let screen = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen);
        let status = XScreenSaverQueryInfo(display, root_window, info);
        let time = (*info).idle;
        XFree(info as _);
        XCloseDisplay(display);
        if status == 1 {
            Ok(Duration::from_millis(time as _))
        } else {
            Err(Error::new("Status not OK"))
        }
    }
}
