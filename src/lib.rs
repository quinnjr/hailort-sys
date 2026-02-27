//! Raw FFI bindings to the HailoRT runtime library.
//!
//! These bindings mirror the C API declared in `<hailo/hailort.h>` from the
//! [hailo-ai/hailort](https://github.com/hailo-ai/hailort) open-source project.
//! They give Rust code direct access to the Hailo AI HAT+ on Raspberry Pi.
//!
//! # Safety
//! Every function in the [`ffi`] module is `unsafe`. Callers must uphold all
//! invariants described in the upstream C API documentation:
//! <https://hailo.ai/developer-zone/>
//!
//! # Linking
//! The `build.rs` script attempts `pkg-config` first, then falls back to
//! searching `/usr/lib` and `/usr/local/lib` for `libhailort.so`.
//!
//! # Module layout
//! | Module | Contents |
//! |--------|----------|
//! | [`constants`] | Sizing limits and default parameter values |
//! | [`handles`] | Opaque C handle types |
//! | [`status`] | `hailo_status` return code and all error constants |
//! | [`enums`] | All other C enum type aliases and their variants |
//! | [`types`] | Structs, unions, and callback type aliases |
//! | [`ffi`] | `unsafe extern "C"` function declarations |

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub mod constants;
pub mod enums;
pub mod ffi;
pub mod handles;
pub mod status;
pub mod types;

pub use constants::*;
pub use enums::*;
pub use ffi::*;
pub use handles::*;
pub use status::*;
pub use types::*;
