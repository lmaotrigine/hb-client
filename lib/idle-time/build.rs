// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(target_os = "linux")]
fn ensure_x11_deps() {
    let deps = [("x11", "1.4.99.1"), ("xscrnsaver", "1.2")];
    for &(dep, version) in deps.iter() {
        pkg_config::Config::new()
            .atleast_version(version)
            .probe(dep)
            .unwrap();
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    ensure_x11_deps();

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=CoreFoundation\ncargo:rustc-link-lib=framework=IOKit");

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=user32");
}
