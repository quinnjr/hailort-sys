//! Opaque C handle types.
//!
//! In C these are declared as `typedef struct _foo *foo;` — pointers to
//! incomplete (forward-declared) structs.  The idiomatic Rust FFI pattern is
//! an uninhabited enum: Rust can hold and pass the pointer but never
//! construct or dereference the pointee without an explicit `unsafe` block.

use std::os::raw::c_void;

/// Opaque handle to a physical Hailo device.
pub enum hailo_device_opaque {}
pub type hailo_device = *mut hailo_device_opaque;

/// Opaque handle to a virtual device (may aggregate multiple physical devices).
pub enum hailo_vdevice_opaque {}
pub type hailo_vdevice = *mut hailo_vdevice_opaque;

/// Opaque handle to a loaded HEF (Hailo Executable Format) model file.
pub enum hailo_hef_opaque {}
pub type hailo_hef = *mut hailo_hef_opaque;

pub enum hailo_input_stream_opaque {}
pub type hailo_input_stream = *mut hailo_input_stream_opaque;

pub enum hailo_output_stream_opaque {}
pub type hailo_output_stream = *mut hailo_output_stream_opaque;

pub enum hailo_configured_network_group_opaque {}
pub type hailo_configured_network_group = *mut hailo_configured_network_group_opaque;

pub enum hailo_activated_network_group_opaque {}
pub type hailo_activated_network_group = *mut hailo_activated_network_group_opaque;

pub enum hailo_input_transform_context_opaque {}
pub type hailo_input_transform_context = *mut hailo_input_transform_context_opaque;

pub enum hailo_output_transform_context_opaque {}
pub type hailo_output_transform_context = *mut hailo_output_transform_context_opaque;

pub enum hailo_output_demuxer_opaque {}
pub type hailo_output_demuxer = *mut hailo_output_demuxer_opaque;

pub enum hailo_input_vstream_opaque {}
pub type hailo_input_vstream = *mut hailo_input_vstream_opaque;

pub enum hailo_output_vstream_opaque {}
pub type hailo_output_vstream = *mut hailo_output_vstream_opaque;

/// Scan-devices parameter block.
///
/// This type is opaque in the upstream header; only a null pointer is accepted
/// by the current API.  Pass `std::ptr::null_mut()` to `hailo_scan_devices`.
pub type hailo_scan_devices_params_t = c_void;
