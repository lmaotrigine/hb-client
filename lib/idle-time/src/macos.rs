// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Error;
use std::{
    ffi::{c_char, c_void, CString},
    ptr::NonNull,
    time::Duration,
};
extern "C" {
    fn IOMasterPort(bootstrap_port: u32, master_port: *mut u32) -> i32;
    fn IOServiceGetMatchingServices(
        master_port: u32,
        matching: *mut Dictionary,
        existing: *mut u32,
    ) -> i32;
    fn IOServiceMatching(name: *const c_char) -> *mut Dictionary;
    fn IOIteratorNext(iterator: u32) -> u32;
    fn IORegistryEntryCreateCFProperties(
        entry: u32,
        properties: *mut *mut Dictionary,
        allocator: *const Allocator,
        options: u32,
    ) -> i32;
    fn IOObjectRelease(object: u32) -> i32;
}
// core foundation

type Allocator = c_void;
type Data = c_void;
type Dictionary = c_void;
type Number = c_void;
type String_ = c_void;

#[repr(C)]
struct Range {
    location: u64,
    length: u64,
}

const CF_NUMBER_S_INT64_TYPE: i64 = 4;
const CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

extern "C" {
    fn CFDataGetBytes(data: *mut Data, range: Range, buffer: *mut u8);
    fn CFDataGetTypeID() -> u64;
    fn CFDictionaryGetValueIfPresent(
        dict: NonNull<Dictionary>,
        key: *const c_void,
        value: *mut *const c_void,
    ) -> u8;
    fn CFGetTypeID(cf: *const c_void) -> u64;
    fn CFNumberGetTypeID() -> u64;
    fn CFNumberGetValue(number: *const Number, type_: i64, value: *mut c_void) -> bool;
    fn CFRelease(cf: *const c_void);
    fn CFStringCreateWithCString(
        alloc: *const Allocator,
        c_str: *const c_char,
        encoding: u32,
    ) -> *const String_;
    static kCFAllocatorDefault: *const Allocator;
}

fn get_master_port() -> Result<u32, Error> {
    let mut master_port = 0u32;
    let res = unsafe { IOMasterPort(0, &mut master_port as _) };
    if res == 0 {
        Ok(master_port)
    } else {
        let last_os = std::io::Error::last_os_error();
        Err(Error::new(format!("unable to open mach port: {last_os}")))
    }
}

fn io_service_get_matching_services(master_port: u32, name: &str) -> Result<u32, Error> {
    let mut existing = 0u32;
    let res = unsafe {
        let name = CString::new(name).expect("invalid service name");
        IOServiceGetMatchingServices(
            master_port,
            IOServiceMatching(name.as_ptr()),
            &mut existing as _,
        )
    };
    if res == 0 {
        Ok(existing)
    } else {
        let last_os = std::io::Error::last_os_error();
        Err(Error::new(format!("unable to lookup {name}: {last_os}")))
    }
}

fn io_registry_entry_create_cf_properties(entry: u32) -> Result<NonNull<c_void>, Error> {
    let mut properties = std::ptr::null_mut();
    let res = unsafe {
        IORegistryEntryCreateCFProperties(entry, &mut properties, kCFAllocatorDefault, 0)
    };
    if res == 0 {
        Ok(NonNull::new(properties).unwrap())
    } else {
        let last_os = std::io::Error::last_os_error();
        Err(Error::new(format!("unable to get properties: {last_os}")))
    }
}

unsafe fn _idle(iter: u32) -> Result<u64, Error> {
    let ns;
    let entry = IOIteratorNext(iter);
    if entry == 0 {
        return Err(Error::new(format!(
            "IOIteratorNext failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    let mut value: *const c_void = std::ptr::null_mut();
    let properties = io_registry_entry_create_cf_properties(entry)?;
    let prop_name = CString::new("HIDIdleTime").unwrap();
    let prop_name = CFStringCreateWithCString(
        kCFAllocatorDefault,
        prop_name.as_ptr(),
        CF_STRING_ENCODING_UTF8,
    );
    let present = CFDictionaryGetValueIfPresent(properties, prop_name, &mut value);
    if present != 1 {
        return Err(Error::new(format!(
            "CFDictionaryGetValueIfPresent failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    IOObjectRelease(iter);
    IOObjectRelease(entry);
    CFRelease(properties.as_ptr());
    if CFGetTypeID(value) == CFDataGetTypeID() {
        let mut buf = [0u8; std::mem::size_of::<i64>()];
        let range = Range {
            location: buf.as_ptr() as _,
            length: std::mem::size_of::<i64>() as _,
        };
        CFDataGetBytes(value as _, range, buf.as_mut_ptr());
        ns = i64::from_ne_bytes(buf) as u64;
    } else if CFGetTypeID(value) == CFNumberGetTypeID() {
        let mut buf = [0i64, 1];
        CFNumberGetValue(value as _, CF_NUMBER_S_INT64_TYPE, buf.as_mut_ptr() as _);
        ns = buf[0] as u64;
    } else {
        return Err(Error::new(
            "unexpected type of value returned from CFDictionaryGetValueIfPresent",
        ));
    }
    IOObjectRelease(entry);
    Ok(ns)
}

pub fn idle() -> Result<Duration, Error> {
    let port = get_master_port()?;
    let iter = io_service_get_matching_services(port, "IOHIDSystem")?;
    if iter == 0 {
        return Err(Error::new("unable to get IOHIDSystem service"));
    }
    let ns = unsafe {
        let ns = _idle(iter)?;
        IOObjectRelease(iter);
        ns
    };
    Ok(Duration::from_nanos(ns))
}
