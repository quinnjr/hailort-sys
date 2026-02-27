fn main() {
    // Try pkg-config first. If hailort provides a .pc file this handles
    // include paths, link flags, and rpath automatically.
    if pkg_config::probe_library("hailort").is_ok() {
        return;
    }

    // Fall back to the standard installation paths used by the Hailo .deb
    // package on Raspberry Pi OS / Ubuntu.
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=hailort");
}
