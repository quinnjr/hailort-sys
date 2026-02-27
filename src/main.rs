/// Minimal example: print the HailoRT library version.
///
/// Run on a Pi with the HailoRT package installed:
///
///   cargo run
fn main() {
    let mut version = hailort_sys::hailo_version_t {
        major: 0,
        minor: 0,
        revision: 0,
    };

    let status = unsafe { hailort_sys::hailo_get_library_version(&mut version) };

    if status == hailort_sys::HAILO_SUCCESS {
        println!(
            "HailoRT version: {}.{}.{}",
            version.major, version.minor, version.revision
        );
    } else {
        let msg = unsafe { hailort_sys::hailo_get_status_message(status) };
        let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
        eprintln!("hailo_get_library_version failed: {}", msg.to_string_lossy());
        std::process::exit(1);
    }
}
