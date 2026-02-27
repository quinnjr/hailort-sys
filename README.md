# hailort-sys

Raw FFI bindings to the [HailoRT](https://github.com/hailo-ai/hailort) C runtime
library for the [Hailo AI HAT+](https://www.raspberrypi.com/products/ai-hat/) on
Raspberry Pi.

[![Crates.io](https://img.shields.io/crates/v/hailort-sys)](https://crates.io/crates/hailort-sys)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## Overview

This crate exposes the HailoRT C API as a thin, `unsafe` Rust interface following
the standard [`*-sys` crate convention](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages).
It is intended as the foundation for higher-level Rust wrappers; most users should
prefer such a wrapper rather than calling these bindings directly.

The bindings cover:

| Module | Contents |
|--------|----------|
| `constants` | Sizing limits, topology capacities, default parameter values |
| `handles` | Opaque C handle types (`hailo_device`, `hailo_vdevice`, `hailo_hef`, streams, vstreams, …) |
| `status` | `hailo_status` return type and all 98 named error codes |
| `enums` | All C enum type aliases (format, stream, power, health, notification, …) |
| `types` | All `#[repr(C)]` structs, unions, and callback type aliases |
| `ffi` | Raw `extern "C"` declarations for every public HailoRT function |

## Requirements

| Requirement | Notes |
|-------------|-------|
| Hailo AI HAT+ (Hailo-8, Hailo-8L, or Hailo-10H) | |
| HailoRT runtime package | `libhailort` must be installed |
| Raspberry Pi OS / Ubuntu (64-bit) | Other Linux distros may work |
| Rust 1.77+ | Required for stable `offset_of!` |

### Installing HailoRT

Download the HailoRT `.deb` package for your platform from the
[Hailo Developer Zone](https://hailo.ai/developer-zone/software-downloads/) and install it:

```sh
sudo dpkg -i hailort_<version>_arm64.deb
```

The package installs `libhailort.so` to `/usr/lib` and the C headers to
`/usr/include/hailo/`.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
hailort-sys = "0.1"
```

All public symbols are re-exported from the crate root via `pub use module::*`,
so you can use them without module prefixes:

```rust
use hailort_sys::*;

fn main() {
    let mut version = hailo_version_t { major: 0, minor: 0, revision: 0 };

    let status = unsafe { hailo_get_library_version(&mut version) };

    if status == HAILO_SUCCESS {
        println!(
            "HailoRT version: {}.{}.{}",
            version.major, version.minor, version.revision
        );
    } else {
        let msg = unsafe { std::ffi::CStr::from_ptr(hailo_get_status_message(status)) };
        eprintln!("error: {}", msg.to_string_lossy());
    }
}
```

## Build

`build.rs` first tries `pkg-config` to locate `libhailort`. If that fails it falls
back to the standard `.deb` installation paths (`/usr/lib`, `/usr/local/lib`).

To point the build at a custom install location set `PKG_CONFIG_PATH` or
`RUSTFLAGS`:

```sh
PKG_CONFIG_PATH=/opt/hailort/lib/pkgconfig cargo build
```

## Safety

Every function in the `ffi` module is `unsafe`. Callers are responsible for:

- Passing valid, non-null pointers where the C API requires them.
- Ensuring output-parameter buffers are large enough.
- Not sharing handles across threads without appropriate synchronisation.
- Checking the returned `hailo_status` before using any output parameters
  (`HAILO_SUCCESS == 0`).

## License

MIT — see [LICENSE](LICENSE).
