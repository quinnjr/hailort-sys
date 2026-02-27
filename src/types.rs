//! Structs, unions, and callback type aliases.
//!
//! These mirror the C struct and typedef declarations in `<hailo/hailort.h>`.
//! Unions require `unsafe` to read; see the individual field docs.

use std::os::raw::{c_char, c_float, c_double, c_int, c_uchar, c_void};

use crate::constants::{
    HAILO_ETH_MAC_LENGTH, HAILO_MAX_BOARD_NAME_LENGTH, HAILO_MAX_DEVICE_ID_LENGTH,
    HAILO_MAX_NETWORK_GROUP_NAME_SIZE, HAILO_MAX_NETWORK_NAME_SIZE,
    HAILO_MAX_NETWORKS_IN_NETWORK_GROUP, HAILO_MAX_NETWORK_GROUPS, HAILO_MAX_PART_NUMBER_LENGTH,
    HAILO_MAX_PRODUCT_NAME_LENGTH, HAILO_MAX_SERIAL_NUMBER_LENGTH, HAILO_MAX_STREAM_NAME_SIZE,
    HAILO_MAX_STREAMS_COUNT, HAILO_MAX_TEMPERATURE_THROTTLING_LEVELS_NUMBER,
    HAILO_SOC_ID_LENGTH, HAILO_SOC_PM_VALUES_BYTES_LENGTH,
    HAILO_UNIT_LEVEL_TRACKING_BYTES_LENGTH, MAX_NUMBER_OF_PLANES,
};
use crate::enums::{
    hailo_buffer_flags_t, hailo_device_architecture_t, hailo_device_boot_source_t,
    hailo_endianness_t, hailo_format_flags_t, hailo_format_order_t, hailo_format_type_t,
    hailo_latency_measurement_flags_t, hailo_nms_burst_type_t, hailo_notification_id_t,
    hailo_overcurrent_protection_overcurrent_zone_t, hailo_pipeline_elem_stats_flags_t,
    hailo_pix_buffer_memory_type_t, hailo_power_mode_t, hailo_scheduling_algorithm_t,
    hailo_stream_direction_t, hailo_stream_flags_t, hailo_stream_interface_t,
    hailo_stream_transform_mode_t, hailo_temperature_protection_temperature_zone_t,
    hailo_vstream_stats_flags_t,
};
use crate::handles::hailo_device;
use crate::status::hailo_status;

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

pub type float32_t = c_float;
pub type float64_t = c_double;
pub type nms_bbox_counter_t = u16;

// ---------------------------------------------------------------------------
// Version and identity
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_version_t {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_firmware_version_t {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_pcie_device_info_t {
    pub domain: u32,
    pub bus: u32,
    pub device: u32,
    pub func: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_device_id_t {
    pub id: [c_char; HAILO_MAX_DEVICE_ID_LENGTH],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_device_identity_t {
    pub protocol_version: u32,
    pub fw_version: hailo_firmware_version_t,
    pub logger_version: u32,
    pub board_name_length: u8,
    pub board_name: [c_char; HAILO_MAX_BOARD_NAME_LENGTH],
    pub is_release: bool,
    pub extended_context_switch_buffer: bool,
    pub extended_fw_check: bool,
    pub device_architecture: hailo_device_architecture_t,
    pub serial_number_length: u8,
    pub serial_number: [c_char; HAILO_MAX_SERIAL_NUMBER_LENGTH],
    pub part_number_length: u8,
    pub part_number: [c_char; HAILO_MAX_PART_NUMBER_LENGTH],
    pub product_name_length: u8,
    pub product_name: [c_char; HAILO_MAX_PRODUCT_NAME_LENGTH],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_core_information_t {
    pub is_release: bool,
    pub extended_context_switch_buffer: bool,
    pub extended_fw_check: bool,
    pub fw_version: hailo_firmware_version_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_device_supported_features_t {
    pub ethernet: bool,
    pub mipi: bool,
    pub pcie: bool,
    pub current_monitoring: bool,
    pub mdio: bool,
    pub power_measurement: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_extended_device_information_t {
    pub neural_network_core_clock_rate: u32,
    pub supported_features: hailo_device_supported_features_t,
    pub boot_source: hailo_device_boot_source_t,
    pub soc_id: [c_uchar; HAILO_SOC_ID_LENGTH],
    pub lcs: c_uchar,
    pub eth_mac_address: [c_uchar; HAILO_ETH_MAC_LENGTH],
    pub unit_level_tracking_id: [c_uchar; HAILO_UNIT_LEVEL_TRACKING_BYTES_LENGTH],
    pub soc_pm_values: [c_uchar; HAILO_SOC_PM_VALUES_BYTES_LENGTH],
    pub gpio_mask: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_fw_user_config_information_t {
    pub version: u32,
    pub entry_count: u32,
    pub total_size: u32,
}

// ---------------------------------------------------------------------------
// VDevice parameters
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_vdevice_params_t {
    pub device_count: u32,
    /// Pointer to an array of `device_count` device IDs, or null for auto.
    pub device_ids: *mut hailo_device_id_t,
    pub scheduling_algorithm: hailo_scheduling_algorithm_t,
    /// Null-terminated group ID string, or null for the default unique group.
    pub group_id: *const c_char,
    pub multi_process_service: bool,
}

// ---------------------------------------------------------------------------
// Data format and quantisation
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_format_t {
    pub type_: hailo_format_type_t,
    pub order: hailo_format_order_t,
    pub flags: hailo_format_flags_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_quant_info_t {
    pub qp_zp: float32_t,
    pub qp_scale: float32_t,
    pub limvals_min: float32_t,
    pub limvals_max: float32_t,
}

// ---------------------------------------------------------------------------
// Stream parameters
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_transform_params_t {
    pub transform_mode: hailo_stream_transform_mode_t,
    pub user_buffer_format: hailo_format_t,
}

/// Empty params struct — reserved for future use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_demux_params_t {
    pub _reserved: u8,
}

/// Empty params struct — reserved for future use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_pcie_input_stream_params_t {
    pub _reserved: u8,
}

/// Empty params struct — reserved for future use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_pcie_output_stream_params_t {
    pub _reserved: u8,
}

/// Empty params struct — reserved for future use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_integrated_input_stream_params_t {
    pub _reserved: u8,
}

/// Empty params struct — reserved for future use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_integrated_output_stream_params_t {
    pub _reserved: u8,
}

/// Anonymous union inside `hailo_stream_parameters_t`.
#[repr(C)]
pub union hailo_stream_params_union_t {
    pub pcie_input_params: hailo_pcie_input_stream_params_t,
    pub integrated_input_params: hailo_integrated_input_stream_params_t,
    pub pcie_output_params: hailo_pcie_output_stream_params_t,
    pub integrated_output_params: hailo_integrated_output_stream_params_t,
}

#[repr(C)]
pub struct hailo_stream_parameters_t {
    pub stream_interface: hailo_stream_interface_t,
    pub direction: hailo_stream_direction_t,
    pub flags: hailo_stream_flags_t,
    pub params: hailo_stream_params_union_t,
}

#[repr(C)]
pub struct hailo_stream_parameters_by_name_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub stream_params: hailo_stream_parameters_t,
}

// ---------------------------------------------------------------------------
// VStream parameters
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_vstream_params_t {
    pub user_buffer_format: hailo_format_t,
    pub timeout_ms: u32,
    pub queue_size: u32,
    pub vstream_stats_flags: hailo_vstream_stats_flags_t,
    pub pipeline_elements_stats_flags: hailo_pipeline_elem_stats_flags_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_input_vstream_params_by_name_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub params: hailo_vstream_params_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_output_vstream_params_by_name_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub params: hailo_vstream_params_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_output_vstream_name_by_group_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub pipeline_group_index: u8,
}

// ---------------------------------------------------------------------------
// Image and pixel buffer types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_3d_image_shape_t {
    pub height: u32,
    pub width: u32,
    pub features: u32,
}

/// `user_ptr` / `fd` plane descriptor (anonymous union in C).
#[repr(C)]
pub union hailo_pix_buffer_plane_ptr_t {
    pub user_ptr: *mut c_void,
    pub fd: c_int,
}

#[repr(C)]
pub struct hailo_pix_buffer_plane_t {
    pub bytes_used: u32,
    pub plane_size: u32,
    pub ptr: hailo_pix_buffer_plane_ptr_t,
}

#[repr(C)]
pub struct hailo_pix_buffer_t {
    pub index: u32,
    pub planes: [hailo_pix_buffer_plane_t; MAX_NUMBER_OF_PLANES],
    pub number_of_planes: u32,
    pub memory_type: hailo_pix_buffer_memory_type_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_dma_buffer_t {
    pub fd: c_int,
    pub size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_buffer_parameters_t {
    pub flags: hailo_buffer_flags_t,
}

// ---------------------------------------------------------------------------
// NMS (Non-Maximum Suppression) types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_nms_defuse_info_t {
    pub class_group_index: u32,
    pub original_name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_nms_info_t {
    pub number_of_classes: u32,
    pub max_bboxes_per_class: u32,
    pub max_bboxes_total: u32,
    pub bbox_size: u32,
    pub chunks_per_frame: u32,
    pub burst_size: u32,
    pub is_defused: bool,
    pub defuse_info: hailo_nms_defuse_info_t,
    pub burst_type: hailo_nms_burst_type_t,
}

#[repr(C)]
pub struct hailo_nms_fuse_input_t {
    pub buffer: *mut c_void,
    pub size: usize,
    pub nms_info: hailo_nms_info_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_nms_shape_t {
    pub number_of_classes: u32,
    pub max_bboxes_per_class: u32,
    pub max_bboxes_total: u32,
    pub max_accumulated_mask_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_bbox_t {
    pub y_min: u16,
    pub x_min: u16,
    pub y_max: u16,
    pub x_max: u16,
    pub score: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_bbox_float32_t {
    pub y_min: float32_t,
    pub x_min: float32_t,
    pub y_max: float32_t,
    pub x_max: float32_t,
    pub score: float32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_rectangle_t {
    pub y_min: float32_t,
    pub x_min: float32_t,
    pub y_max: float32_t,
    pub x_max: float32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_detection_t {
    pub y_min: float32_t,
    pub x_min: float32_t,
    pub y_max: float32_t,
    pub x_max: float32_t,
    pub score: float32_t,
    pub class_id: u16,
}

/// Variable-length detection result header.
///
/// The `detections` field is a C flexible array member; access elements
/// beyond index 0 via raw pointer arithmetic on the parent pointer.
#[repr(C)]
pub struct hailo_detections_t {
    pub count: u16,
    pub detections: [hailo_detection_t; 0],
}

#[repr(C)]
pub struct hailo_detection_with_byte_mask_t {
    pub box_: hailo_rectangle_t,
    pub score: float32_t,
    pub class_id: u16,
    pub mask_size: usize,
    /// Deprecated: use `mask_offset` instead.
    pub mask: *mut c_uchar,
    pub mask_offset: usize,
}

// ---------------------------------------------------------------------------
// Async I/O completion and callbacks
// ---------------------------------------------------------------------------

/// Completion info passed to [`hailo_stream_write_async_callback_t`].
#[repr(C)]
pub struct hailo_stream_write_async_completion_info_t {
    pub status: hailo_status,
    pub buffer_addr: *const c_void,
    pub buffer_size: usize,
    pub opaque: *mut c_void,
}

/// Completion info passed to [`hailo_stream_read_async_callback_t`].
#[repr(C)]
pub struct hailo_stream_read_async_completion_info_t {
    pub status: hailo_status,
    pub buffer_addr: *mut c_void,
    pub buffer_size: usize,
    pub opaque: *mut c_void,
}

pub type hailo_stream_write_async_callback_t =
    unsafe extern "C" fn(info: *const hailo_stream_write_async_completion_info_t);

pub type hailo_stream_read_async_callback_t =
    unsafe extern "C" fn(info: *const hailo_stream_read_async_completion_info_t);

pub type hailo_notification_callback = unsafe extern "C" fn(
    device: hailo_device,
    notification: *const hailo_notification_t,
    opaque: *mut c_void,
);

// ---------------------------------------------------------------------------
// Notification message types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_rx_error_notification_message_t {
    pub error: u32,
    pub queue_number: u32,
    pub rx_errors_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_debug_notification_message_t {
    pub connection_status: u32,
    pub connection_type: u32,
    pub vdma_is_active: u32,
    pub host_port: u32,
    pub host_ip_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_health_monitor_dataflow_shutdown_notification_message_t {
    pub ts0_temperature: float32_t,
    pub ts1_temperature: float32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_health_monitor_temperature_alarm_notification_message_t {
    pub temperature_zone: hailo_temperature_protection_temperature_zone_t,
    pub alarm_ts_id: u32,
    pub ts0_temperature: float32_t,
    pub ts1_temperature: float32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_health_monitor_overcurrent_alert_notification_message_t {
    pub overcurrent_zone: hailo_overcurrent_protection_overcurrent_zone_t,
    pub exceeded_alert_threshold: float32_t,
    pub is_last_overcurrent_violation_reached: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_health_monitor_lcu_ecc_error_notification_message_t {
    pub cluster_error: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_health_monitor_cpu_ecc_notification_message_t {
    pub memory_bitmap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_context_switch_breakpoint_reached_message_t {
    pub network_group_index: u8,
    pub batch_index: u32,
    pub context_index: u16,
    pub action_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_health_monitor_clock_changed_notification_message_t {
    pub previous_clock: u32,
    pub current_clock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_hw_infer_manager_infer_done_notification_message_t {
    pub infer_cycles: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_start_update_cache_offset_notification_message_t {
    pub cache_id_bitmask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_context_switch_run_time_error_message_t {
    pub exit_status: u32,
    pub network_group_index: u8,
    pub batch_index: u16,
    pub context_index: u16,
    pub action_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hailo_throttling_state_change_message_t {
    pub new_state: u16,
}

#[repr(C)]
pub union hailo_notification_message_parameters_t {
    pub rx_error_notification: hailo_rx_error_notification_message_t,
    pub debug_notification: hailo_debug_notification_message_t,
    pub health_monitor_dataflow_shutdown_notification:
        hailo_health_monitor_dataflow_shutdown_notification_message_t,
    pub health_monitor_temperature_alarm_notification:
        hailo_health_monitor_temperature_alarm_notification_message_t,
    pub health_monitor_overcurrent_alert_notification:
        hailo_health_monitor_overcurrent_alert_notification_message_t,
    pub health_monitor_lcu_ecc_error_notification:
        hailo_health_monitor_lcu_ecc_error_notification_message_t,
    pub health_monitor_cpu_ecc_notification: hailo_health_monitor_cpu_ecc_notification_message_t,
    pub context_switch_breakpoint_reached_notification:
        hailo_context_switch_breakpoint_reached_message_t,
    pub health_monitor_clock_changed_notification:
        hailo_health_monitor_clock_changed_notification_message_t,
    pub hw_infer_manager_infer_done_notification:
        hailo_hw_infer_manager_infer_done_notification_message_t,
    pub context_switch_run_time_error: hailo_context_switch_run_time_error_message_t,
    pub start_update_cache_offset_notification:
        hailo_start_update_cache_offset_notification_message_t,
    pub throttling_state_change: hailo_throttling_state_change_message_t,
}

#[repr(C)]
pub struct hailo_notification_t {
    pub id: hailo_notification_id_t,
    pub sequence: u32,
    pub body: hailo_notification_message_parameters_t,
}

// ---------------------------------------------------------------------------
// Stream and VStream info (contain anonymous unions)
// ---------------------------------------------------------------------------

/// Unnamed inner struct for the tensor-shape branch of `hailo_stream_info_t`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_stream_info_shape_pair_t {
    pub shape: hailo_3d_image_shape_t,
    pub hw_shape: hailo_3d_image_shape_t,
}

/// Anonymous union inside `hailo_stream_info_t`: tensor shape vs NMS info.
#[repr(C)]
pub union hailo_stream_info_shape_t {
    pub shapes: hailo_stream_info_shape_pair_t,
    pub nms_info: hailo_nms_info_t,
}

#[repr(C)]
pub struct hailo_stream_info_t {
    pub shape: hailo_stream_info_shape_t,
    pub hw_data_bytes: u32,
    pub hw_frame_size: u32,
    pub format: hailo_format_t,
    pub direction: hailo_stream_direction_t,
    pub index: u8,
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub quant_info: hailo_quant_info_t,
    pub is_mux: bool,
}

/// Anonymous union inside `hailo_vstream_info_t`: tensor shape vs NMS shape.
#[repr(C)]
pub union hailo_vstream_info_shape_t {
    pub shape: hailo_3d_image_shape_t,
    pub nms_shape: hailo_nms_shape_t,
}

#[repr(C)]
pub struct hailo_vstream_info_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub network_name: [c_char; HAILO_MAX_NETWORK_NAME_SIZE],
    pub direction: hailo_stream_direction_t,
    pub format: hailo_format_t,
    pub shape: hailo_vstream_info_shape_t,
    pub quant_info: hailo_quant_info_t,
}

// ---------------------------------------------------------------------------
// Power, temperature, and health monitoring
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_power_measurement_data_t {
    pub average_value: float32_t,
    pub average_time_value_milliseconds: float32_t,
    pub min_value: float32_t,
    pub max_value: float32_t,
    pub total_number_of_samples: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_chip_temperature_info_t {
    pub ts0_temperature: float32_t,
    pub ts1_temperature: float32_t,
    pub sample_count: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_throttling_level_t {
    pub temperature_threshold: float32_t,
    pub hysteresis_temperature_threshold: float32_t,
    pub throttling_nn_clock_freq: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_health_info_t {
    pub overcurrent_protection_active: bool,
    pub current_overcurrent_zone: u8,
    pub red_overcurrent_threshold: float32_t,
    pub overcurrent_throttling_active: bool,
    pub temperature_throttling_active: bool,
    pub current_temperature_zone: u8,
    pub current_temperature_throttling_level: i8,
    pub temperature_throttling_levels:
        [hailo_throttling_level_t; HAILO_MAX_TEMPERATURE_THROTTLING_LEVELS_NUMBER],
    pub orange_temperature_threshold: i32,
    pub orange_hysteresis_temperature_threshold: i32,
    pub red_temperature_threshold: i32,
    pub red_hysteresis_temperature_threshold: i32,
    pub requested_overcurrent_clock_freq: u32,
    pub requested_temperature_clock_freq: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_performance_stats_t {
    pub cpu_utilization: float32_t,
    pub ram_size_total: i64,
    pub ram_size_used: i64,
    pub nnc_utilization: float32_t,
    pub ddr_noc_total_transactions: i32,
    pub dsp_utilization: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_health_stats_t {
    pub on_die_temperature: float32_t,
    pub on_die_voltage: i32,
    pub bist_failure_mask: i32,
}

// ---------------------------------------------------------------------------
// Network and configure parameter structures
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_network_parameters_t {
    pub batch_size: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_network_parameters_by_name_t {
    pub name: [c_char; HAILO_MAX_NETWORK_NAME_SIZE],
    pub network_params: hailo_network_parameters_t,
}

#[repr(C)]
pub struct hailo_configure_network_group_params_t {
    pub name: [c_char; HAILO_MAX_NETWORK_GROUP_NAME_SIZE],
    pub batch_size: u16,
    pub power_mode: hailo_power_mode_t,
    pub latency: hailo_latency_measurement_flags_t,
    pub enable_kv_cache: bool,
    pub stream_params_by_name_count: usize,
    pub stream_params_by_name: [hailo_stream_parameters_by_name_t; HAILO_MAX_STREAMS_COUNT],
    pub network_params_by_name_count: usize,
    pub network_params_by_name:
        [hailo_network_parameters_by_name_t; HAILO_MAX_NETWORKS_IN_NETWORK_GROUP],
}

#[repr(C)]
pub struct hailo_configure_params_t {
    pub network_group_params_count: usize,
    pub network_group_params: [hailo_configure_network_group_params_t; HAILO_MAX_NETWORK_GROUPS],
}

/// Empty params struct — reserved for future use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_activate_network_group_params_t {
    pub _reserved: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_network_group_info_t {
    pub name: [c_char; HAILO_MAX_NETWORK_GROUP_NAME_SIZE],
    pub is_multi_context: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_layer_name_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_network_info_t {
    pub name: [c_char; HAILO_MAX_NETWORK_NAME_SIZE],
}

#[repr(C)]
pub struct hailo_stream_raw_buffer_t {
    pub buffer: *mut c_void,
    pub size: usize,
}

#[repr(C)]
pub struct hailo_stream_raw_buffer_by_name_t {
    pub name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub raw_buffer: hailo_stream_raw_buffer_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_latency_measurement_result_t {
    pub avg_hw_latency_ms: float64_t,
}

#[repr(C)]
pub struct hailo_rate_limit_t {
    pub stream_name: [c_char; HAILO_MAX_STREAM_NAME_SIZE],
    pub rate: u32,
}

// ---------------------------------------------------------------------------
// I²C
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_i2c_slave_config_t {
    pub endianness: hailo_endianness_t,
    pub slave_address: u16,
    pub register_address_size: u8,
    pub bus_index: u8,
    pub should_hold_bus: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
//
// All size/alignment/offset assertions target 64-bit Linux (the only supported
// platform for the Hailo AI HAT+).  Tests that depend on pointer width are
// guarded with `#[cfg(target_pointer_width = "64")]`.

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, offset_of, size_of};

    // --- Primitive type aliases ----------------------------------------------

    #[test]
    fn primitive_aliases_match_underlying_type() {
        assert_eq!(size_of::<float32_t>(), size_of::<f32>());
        assert_eq!(size_of::<float64_t>(), size_of::<f64>());
        assert_eq!(size_of::<nms_bbox_counter_t>(), size_of::<u16>());
        assert_eq!(align_of::<nms_bbox_counter_t>(), align_of::<u16>());
    }

    // --- Version / identity structs -----------------------------------------

    #[test]
    fn hailo_version_t_layout() {
        assert_eq!(size_of::<hailo_version_t>(), 12);
        assert_eq!(align_of::<hailo_version_t>(), 4);
        assert_eq!(offset_of!(hailo_version_t, major), 0);
        assert_eq!(offset_of!(hailo_version_t, minor), 4);
        assert_eq!(offset_of!(hailo_version_t, revision), 8);
    }

    #[test]
    fn hailo_firmware_version_t_layout() {
        assert_eq!(size_of::<hailo_firmware_version_t>(), 12);
        assert_eq!(align_of::<hailo_firmware_version_t>(), 4);
        assert_eq!(offset_of!(hailo_firmware_version_t, major), 0);
        assert_eq!(offset_of!(hailo_firmware_version_t, minor), 4);
        assert_eq!(offset_of!(hailo_firmware_version_t, revision), 8);
    }

    #[test]
    fn hailo_pcie_device_info_t_layout() {
        assert_eq!(size_of::<hailo_pcie_device_info_t>(), 16);
        assert_eq!(align_of::<hailo_pcie_device_info_t>(), 4);
        assert_eq!(offset_of!(hailo_pcie_device_info_t, domain), 0);
        assert_eq!(offset_of!(hailo_pcie_device_info_t, bus), 4);
        assert_eq!(offset_of!(hailo_pcie_device_info_t, device), 8);
        assert_eq!(offset_of!(hailo_pcie_device_info_t, func), 12);
    }

    #[test]
    fn hailo_device_id_t_layout() {
        use crate::constants::HAILO_MAX_DEVICE_ID_LENGTH;
        assert_eq!(size_of::<hailo_device_id_t>(), HAILO_MAX_DEVICE_ID_LENGTH);
        assert_eq!(offset_of!(hailo_device_id_t, id), 0);
    }

    #[test]
    fn hailo_device_identity_t_layout() {
        // Computed layout (64-bit):
        //   0: protocol_version (u32)
        //   4: fw_version (12)
        //  16: logger_version (u32)
        //  20: board_name_length (u8)
        //  21: board_name ([i8; 32])
        //  53: is_release (bool)
        //  54: extended_context_switch_buffer (bool)
        //  55: extended_fw_check (bool)
        //  56: device_architecture (c_int, align 4)
        //  60: serial_number_length (u8)
        //  61: serial_number ([i8; 16])
        //  77: part_number_length (u8)
        //  78: part_number ([i8; 16])
        //  94: product_name_length (u8)
        //  95: product_name ([i8; 42])  → end 137 → padded to 140
        assert_eq!(size_of::<hailo_device_identity_t>(), 140);
        assert_eq!(align_of::<hailo_device_identity_t>(), 4);
        assert_eq!(offset_of!(hailo_device_identity_t, protocol_version), 0);
        assert_eq!(offset_of!(hailo_device_identity_t, fw_version), 4);
        assert_eq!(offset_of!(hailo_device_identity_t, logger_version), 16);
        assert_eq!(offset_of!(hailo_device_identity_t, board_name_length), 20);
        assert_eq!(offset_of!(hailo_device_identity_t, board_name), 21);
        assert_eq!(offset_of!(hailo_device_identity_t, is_release), 53);
        assert_eq!(offset_of!(hailo_device_identity_t, device_architecture), 56);
        assert_eq!(offset_of!(hailo_device_identity_t, serial_number_length), 60);
        assert_eq!(offset_of!(hailo_device_identity_t, serial_number), 61);
        assert_eq!(offset_of!(hailo_device_identity_t, part_number_length), 77);
        assert_eq!(offset_of!(hailo_device_identity_t, part_number), 78);
        assert_eq!(offset_of!(hailo_device_identity_t, product_name_length), 94);
        assert_eq!(offset_of!(hailo_device_identity_t, product_name), 95);
    }

    #[test]
    fn hailo_core_information_t_layout() {
        // is_release(1) ext_ctx(1) ext_fw(1) pad(1) fw_version(12) = 16
        assert_eq!(size_of::<hailo_core_information_t>(), 16);
        assert_eq!(align_of::<hailo_core_information_t>(), 4);
        assert_eq!(offset_of!(hailo_core_information_t, is_release), 0);
        assert_eq!(offset_of!(hailo_core_information_t, fw_version), 4);
    }

    #[test]
    fn hailo_device_supported_features_t_layout() {
        // 6 bools = 6 bytes, align 1
        assert_eq!(size_of::<hailo_device_supported_features_t>(), 6);
        assert_eq!(align_of::<hailo_device_supported_features_t>(), 1);
        assert_eq!(offset_of!(hailo_device_supported_features_t, ethernet), 0);
        assert_eq!(offset_of!(hailo_device_supported_features_t, mipi), 1);
        assert_eq!(offset_of!(hailo_device_supported_features_t, pcie), 2);
        assert_eq!(
            offset_of!(hailo_device_supported_features_t, current_monitoring),
            3
        );
        assert_eq!(offset_of!(hailo_device_supported_features_t, mdio), 4);
        assert_eq!(
            offset_of!(hailo_device_supported_features_t, power_measurement),
            5
        );
    }

    #[test]
    fn hailo_extended_device_information_t_layout() {
        // 0: nn_core_clock(4)  4: features(6)  [pad 2]  12: boot_source(4)
        // 16: soc_id(32)  48: lcs(1)  49: eth_mac(6)  55: unit_track(12)
        // 67: soc_pm(24)  [pad 1]  92: gpio_mask(2)  → end 94 → padded to 96
        assert_eq!(size_of::<hailo_extended_device_information_t>(), 96);
        assert_eq!(align_of::<hailo_extended_device_information_t>(), 4);
        assert_eq!(
            offset_of!(
                hailo_extended_device_information_t,
                neural_network_core_clock_rate
            ),
            0
        );
        assert_eq!(
            offset_of!(hailo_extended_device_information_t, supported_features),
            4
        );
        assert_eq!(
            offset_of!(hailo_extended_device_information_t, boot_source),
            12
        );
        assert_eq!(offset_of!(hailo_extended_device_information_t, soc_id), 16);
        assert_eq!(offset_of!(hailo_extended_device_information_t, lcs), 48);
        assert_eq!(
            offset_of!(hailo_extended_device_information_t, eth_mac_address),
            49
        );
        assert_eq!(
            offset_of!(
                hailo_extended_device_information_t,
                unit_level_tracking_id
            ),
            55
        );
        assert_eq!(
            offset_of!(hailo_extended_device_information_t, soc_pm_values),
            67
        );
        assert_eq!(
            offset_of!(hailo_extended_device_information_t, gpio_mask),
            92
        );
    }

    #[test]
    fn hailo_fw_user_config_information_t_layout() {
        assert_eq!(size_of::<hailo_fw_user_config_information_t>(), 12);
        assert_eq!(align_of::<hailo_fw_user_config_information_t>(), 4);
        assert_eq!(offset_of!(hailo_fw_user_config_information_t, version), 0);
        assert_eq!(
            offset_of!(hailo_fw_user_config_information_t, entry_count),
            4
        );
        assert_eq!(
            offset_of!(hailo_fw_user_config_information_t, total_size),
            8
        );
    }

    // --- VDevice params ------------------------------------------------------

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_vdevice_params_t_layout() {
        // 0: device_count(4) [pad 4] 8: device_ids(ptr,8)
        // 16: scheduling_algorithm(4) [pad 4] 24: group_id(ptr,8)
        // 32: multi_process_service(1) → end 33 → padded to 40
        assert_eq!(size_of::<hailo_vdevice_params_t>(), 40);
        assert_eq!(align_of::<hailo_vdevice_params_t>(), 8);
        assert_eq!(offset_of!(hailo_vdevice_params_t, device_count), 0);
        assert_eq!(offset_of!(hailo_vdevice_params_t, device_ids), 8);
        assert_eq!(
            offset_of!(hailo_vdevice_params_t, scheduling_algorithm),
            16
        );
        assert_eq!(offset_of!(hailo_vdevice_params_t, group_id), 24);
        assert_eq!(
            offset_of!(hailo_vdevice_params_t, multi_process_service),
            32
        );
    }

    // --- Format and quantisation --------------------------------------------

    #[test]
    fn hailo_format_t_layout() {
        assert_eq!(size_of::<hailo_format_t>(), 12);
        assert_eq!(align_of::<hailo_format_t>(), 4);
        assert_eq!(offset_of!(hailo_format_t, type_), 0);
        assert_eq!(offset_of!(hailo_format_t, order), 4);
        assert_eq!(offset_of!(hailo_format_t, flags), 8);
    }

    #[test]
    fn hailo_quant_info_t_layout() {
        assert_eq!(size_of::<hailo_quant_info_t>(), 16);
        assert_eq!(align_of::<hailo_quant_info_t>(), 4);
        assert_eq!(offset_of!(hailo_quant_info_t, qp_zp), 0);
        assert_eq!(offset_of!(hailo_quant_info_t, qp_scale), 4);
        assert_eq!(offset_of!(hailo_quant_info_t, limvals_min), 8);
        assert_eq!(offset_of!(hailo_quant_info_t, limvals_max), 12);
    }

    // --- Stream params -------------------------------------------------------

    #[test]
    fn hailo_transform_params_t_layout() {
        assert_eq!(size_of::<hailo_transform_params_t>(), 16);
        assert_eq!(align_of::<hailo_transform_params_t>(), 4);
        assert_eq!(offset_of!(hailo_transform_params_t, transform_mode), 0);
        assert_eq!(offset_of!(hailo_transform_params_t, user_buffer_format), 4);
    }

    #[test]
    fn empty_stream_param_structs_are_one_byte() {
        assert_eq!(size_of::<hailo_demux_params_t>(), 1);
        assert_eq!(size_of::<hailo_pcie_input_stream_params_t>(), 1);
        assert_eq!(size_of::<hailo_pcie_output_stream_params_t>(), 1);
        assert_eq!(size_of::<hailo_integrated_input_stream_params_t>(), 1);
        assert_eq!(size_of::<hailo_integrated_output_stream_params_t>(), 1);
    }

    #[test]
    fn hailo_stream_params_union_t_layout() {
        assert_eq!(size_of::<hailo_stream_params_union_t>(), 1);
        assert_eq!(align_of::<hailo_stream_params_union_t>(), 1);
    }

    #[test]
    fn hailo_stream_parameters_t_layout() {
        // stream_interface(4) direction(4) flags(4) params(1) → end 13 → pad to 16
        assert_eq!(size_of::<hailo_stream_parameters_t>(), 16);
        assert_eq!(align_of::<hailo_stream_parameters_t>(), 4);
        assert_eq!(offset_of!(hailo_stream_parameters_t, stream_interface), 0);
        assert_eq!(offset_of!(hailo_stream_parameters_t, direction), 4);
        assert_eq!(offset_of!(hailo_stream_parameters_t, flags), 8);
        assert_eq!(offset_of!(hailo_stream_parameters_t, params), 12);
    }

    #[test]
    fn hailo_stream_parameters_by_name_t_layout() {
        // name([i8;128]) + stream_params(16, align 4) = 144
        assert_eq!(size_of::<hailo_stream_parameters_by_name_t>(), 144);
        assert_eq!(align_of::<hailo_stream_parameters_by_name_t>(), 4);
        assert_eq!(offset_of!(hailo_stream_parameters_by_name_t, name), 0);
        assert_eq!(
            offset_of!(hailo_stream_parameters_by_name_t, stream_params),
            128
        );
    }

    // --- VStream params -------------------------------------------------------

    #[test]
    fn hailo_vstream_params_t_layout() {
        // format(12) timeout_ms(4) queue_size(4) vstream_stats(4) pipeline_stats(4) = 28
        assert_eq!(size_of::<hailo_vstream_params_t>(), 28);
        assert_eq!(align_of::<hailo_vstream_params_t>(), 4);
        assert_eq!(offset_of!(hailo_vstream_params_t, user_buffer_format), 0);
        assert_eq!(offset_of!(hailo_vstream_params_t, timeout_ms), 12);
        assert_eq!(offset_of!(hailo_vstream_params_t, queue_size), 16);
        assert_eq!(offset_of!(hailo_vstream_params_t, vstream_stats_flags), 20);
        assert_eq!(
            offset_of!(hailo_vstream_params_t, pipeline_elements_stats_flags),
            24
        );
    }

    #[test]
    fn hailo_input_vstream_params_by_name_t_layout() {
        assert_eq!(size_of::<hailo_input_vstream_params_by_name_t>(), 156);
        assert_eq!(align_of::<hailo_input_vstream_params_by_name_t>(), 4);
        assert_eq!(offset_of!(hailo_input_vstream_params_by_name_t, name), 0);
        assert_eq!(
            offset_of!(hailo_input_vstream_params_by_name_t, params),
            128
        );
    }

    #[test]
    fn hailo_output_vstream_params_by_name_t_layout() {
        assert_eq!(size_of::<hailo_output_vstream_params_by_name_t>(), 156);
        assert_eq!(align_of::<hailo_output_vstream_params_by_name_t>(), 4);
        assert_eq!(offset_of!(hailo_output_vstream_params_by_name_t, name), 0);
        assert_eq!(
            offset_of!(hailo_output_vstream_params_by_name_t, params),
            128
        );
    }

    #[test]
    fn hailo_output_vstream_name_by_group_t_layout() {
        assert_eq!(size_of::<hailo_output_vstream_name_by_group_t>(), 129);
        assert_eq!(align_of::<hailo_output_vstream_name_by_group_t>(), 1);
        assert_eq!(offset_of!(hailo_output_vstream_name_by_group_t, name), 0);
        assert_eq!(
            offset_of!(hailo_output_vstream_name_by_group_t, pipeline_group_index),
            128
        );
    }

    // --- Image / pixel buffer ------------------------------------------------

    #[test]
    fn hailo_3d_image_shape_t_layout() {
        assert_eq!(size_of::<hailo_3d_image_shape_t>(), 12);
        assert_eq!(align_of::<hailo_3d_image_shape_t>(), 4);
        assert_eq!(offset_of!(hailo_3d_image_shape_t, height), 0);
        assert_eq!(offset_of!(hailo_3d_image_shape_t, width), 4);
        assert_eq!(offset_of!(hailo_3d_image_shape_t, features), 8);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_pix_buffer_plane_ptr_t_layout() {
        // union { *mut c_void (8, align 8), c_int (4, align 4) } → size 8, align 8
        assert_eq!(size_of::<hailo_pix_buffer_plane_ptr_t>(), 8);
        assert_eq!(align_of::<hailo_pix_buffer_plane_ptr_t>(), 8);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_pix_buffer_plane_t_layout() {
        // bytes_used(4) plane_size(4) ptr(8, align 8) = 16
        assert_eq!(size_of::<hailo_pix_buffer_plane_t>(), 16);
        assert_eq!(align_of::<hailo_pix_buffer_plane_t>(), 8);
        assert_eq!(offset_of!(hailo_pix_buffer_plane_t, bytes_used), 0);
        assert_eq!(offset_of!(hailo_pix_buffer_plane_t, plane_size), 4);
        assert_eq!(offset_of!(hailo_pix_buffer_plane_t, ptr), 8);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_pix_buffer_t_layout() {
        // index(4) [pad 4] planes(4×16=64, align 8) number_of_planes(4) memory_type(4) = 80
        assert_eq!(size_of::<hailo_pix_buffer_t>(), 80);
        assert_eq!(align_of::<hailo_pix_buffer_t>(), 8);
        assert_eq!(offset_of!(hailo_pix_buffer_t, index), 0);
        assert_eq!(offset_of!(hailo_pix_buffer_t, planes), 8);
        assert_eq!(offset_of!(hailo_pix_buffer_t, number_of_planes), 72);
        assert_eq!(offset_of!(hailo_pix_buffer_t, memory_type), 76);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_dma_buffer_t_layout() {
        // fd(4) [pad 4] size(usize=8) = 16
        assert_eq!(size_of::<hailo_dma_buffer_t>(), 16);
        assert_eq!(align_of::<hailo_dma_buffer_t>(), 8);
        assert_eq!(offset_of!(hailo_dma_buffer_t, fd), 0);
        assert_eq!(offset_of!(hailo_dma_buffer_t, size), 8);
    }

    #[test]
    fn hailo_buffer_parameters_t_layout() {
        assert_eq!(size_of::<hailo_buffer_parameters_t>(), 4);
        assert_eq!(align_of::<hailo_buffer_parameters_t>(), 4);
        assert_eq!(offset_of!(hailo_buffer_parameters_t, flags), 0);
    }

    // --- NMS types -----------------------------------------------------------

    #[test]
    fn hailo_nms_defuse_info_t_layout() {
        // class_group_index(4) original_name([i8;128]) = 132
        assert_eq!(size_of::<hailo_nms_defuse_info_t>(), 132);
        assert_eq!(align_of::<hailo_nms_defuse_info_t>(), 4);
        assert_eq!(offset_of!(hailo_nms_defuse_info_t, class_group_index), 0);
        assert_eq!(offset_of!(hailo_nms_defuse_info_t, original_name), 4);
    }

    #[test]
    fn hailo_nms_info_t_layout() {
        // 6×u32(0-24) is_defused(24) [pad 3] defuse_info(28, size 132) burst_type(160) = 164
        assert_eq!(size_of::<hailo_nms_info_t>(), 164);
        assert_eq!(align_of::<hailo_nms_info_t>(), 4);
        assert_eq!(offset_of!(hailo_nms_info_t, number_of_classes), 0);
        assert_eq!(offset_of!(hailo_nms_info_t, max_bboxes_per_class), 4);
        assert_eq!(offset_of!(hailo_nms_info_t, max_bboxes_total), 8);
        assert_eq!(offset_of!(hailo_nms_info_t, bbox_size), 12);
        assert_eq!(offset_of!(hailo_nms_info_t, chunks_per_frame), 16);
        assert_eq!(offset_of!(hailo_nms_info_t, burst_size), 20);
        assert_eq!(offset_of!(hailo_nms_info_t, is_defused), 24);
        assert_eq!(offset_of!(hailo_nms_info_t, defuse_info), 28);
        assert_eq!(offset_of!(hailo_nms_info_t, burst_type), 160);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_nms_fuse_input_t_layout() {
        // buffer(ptr=8) size(8) nms_info(164, align 4) → end 180 → pad to 184
        assert_eq!(size_of::<hailo_nms_fuse_input_t>(), 184);
        assert_eq!(align_of::<hailo_nms_fuse_input_t>(), 8);
        assert_eq!(offset_of!(hailo_nms_fuse_input_t, buffer), 0);
        assert_eq!(offset_of!(hailo_nms_fuse_input_t, size), 8);
        assert_eq!(offset_of!(hailo_nms_fuse_input_t, nms_info), 16);
    }

    #[test]
    fn hailo_nms_shape_t_layout() {
        assert_eq!(size_of::<hailo_nms_shape_t>(), 16);
        assert_eq!(align_of::<hailo_nms_shape_t>(), 4);
        assert_eq!(offset_of!(hailo_nms_shape_t, number_of_classes), 0);
        assert_eq!(offset_of!(hailo_nms_shape_t, max_bboxes_per_class), 4);
        assert_eq!(offset_of!(hailo_nms_shape_t, max_bboxes_total), 8);
        assert_eq!(offset_of!(hailo_nms_shape_t, max_accumulated_mask_size), 12);
    }

    #[test]
    fn hailo_bbox_t_layout() {
        assert_eq!(size_of::<hailo_bbox_t>(), 10);
        assert_eq!(align_of::<hailo_bbox_t>(), 2);
        assert_eq!(offset_of!(hailo_bbox_t, y_min), 0);
        assert_eq!(offset_of!(hailo_bbox_t, x_min), 2);
        assert_eq!(offset_of!(hailo_bbox_t, y_max), 4);
        assert_eq!(offset_of!(hailo_bbox_t, x_max), 6);
        assert_eq!(offset_of!(hailo_bbox_t, score), 8);
    }

    #[test]
    fn hailo_bbox_float32_t_layout() {
        assert_eq!(size_of::<hailo_bbox_float32_t>(), 20);
        assert_eq!(align_of::<hailo_bbox_float32_t>(), 4);
        assert_eq!(offset_of!(hailo_bbox_float32_t, y_min), 0);
        assert_eq!(offset_of!(hailo_bbox_float32_t, x_min), 4);
        assert_eq!(offset_of!(hailo_bbox_float32_t, y_max), 8);
        assert_eq!(offset_of!(hailo_bbox_float32_t, x_max), 12);
        assert_eq!(offset_of!(hailo_bbox_float32_t, score), 16);
    }

    #[test]
    fn hailo_rectangle_t_layout() {
        assert_eq!(size_of::<hailo_rectangle_t>(), 16);
        assert_eq!(align_of::<hailo_rectangle_t>(), 4);
        assert_eq!(offset_of!(hailo_rectangle_t, y_min), 0);
        assert_eq!(offset_of!(hailo_rectangle_t, x_min), 4);
        assert_eq!(offset_of!(hailo_rectangle_t, y_max), 8);
        assert_eq!(offset_of!(hailo_rectangle_t, x_max), 12);
    }

    #[test]
    fn hailo_detection_t_layout() {
        // 5×f32(0-20) class_id(u16, at 20) → end 22 → pad to 24
        assert_eq!(size_of::<hailo_detection_t>(), 24);
        assert_eq!(align_of::<hailo_detection_t>(), 4);
        assert_eq!(offset_of!(hailo_detection_t, y_min), 0);
        assert_eq!(offset_of!(hailo_detection_t, x_min), 4);
        assert_eq!(offset_of!(hailo_detection_t, y_max), 8);
        assert_eq!(offset_of!(hailo_detection_t, x_max), 12);
        assert_eq!(offset_of!(hailo_detection_t, score), 16);
        assert_eq!(offset_of!(hailo_detection_t, class_id), 20);
    }

    #[test]
    fn hailo_detections_t_layout() {
        // count(u16, 0) [pad 2] detections([T;0], align 4) = 4
        assert_eq!(size_of::<hailo_detections_t>(), 4);
        assert_eq!(align_of::<hailo_detections_t>(), 4);
        assert_eq!(offset_of!(hailo_detections_t, count), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_detection_with_byte_mask_t_layout() {
        // box_(16) score(4) class_id(2) [pad 2] mask_size(8) mask(ptr=8) mask_offset(8) = 48
        assert_eq!(size_of::<hailo_detection_with_byte_mask_t>(), 48);
        assert_eq!(align_of::<hailo_detection_with_byte_mask_t>(), 8);
        assert_eq!(offset_of!(hailo_detection_with_byte_mask_t, box_), 0);
        assert_eq!(offset_of!(hailo_detection_with_byte_mask_t, score), 16);
        assert_eq!(offset_of!(hailo_detection_with_byte_mask_t, class_id), 20);
        assert_eq!(offset_of!(hailo_detection_with_byte_mask_t, mask_size), 24);
        assert_eq!(offset_of!(hailo_detection_with_byte_mask_t, mask), 32);
        assert_eq!(offset_of!(hailo_detection_with_byte_mask_t, mask_offset), 40);
    }

    // --- Async completion info -----------------------------------------------

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_stream_write_async_completion_info_t_layout() {
        // status(4) [pad 4] buffer_addr(ptr=8) buffer_size(8) opaque(ptr=8) = 32
        assert_eq!(size_of::<hailo_stream_write_async_completion_info_t>(), 32);
        assert_eq!(
            align_of::<hailo_stream_write_async_completion_info_t>(),
            8
        );
        assert_eq!(
            offset_of!(hailo_stream_write_async_completion_info_t, status),
            0
        );
        assert_eq!(
            offset_of!(hailo_stream_write_async_completion_info_t, buffer_addr),
            8
        );
        assert_eq!(
            offset_of!(hailo_stream_write_async_completion_info_t, buffer_size),
            16
        );
        assert_eq!(
            offset_of!(hailo_stream_write_async_completion_info_t, opaque),
            24
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_stream_read_async_completion_info_t_layout() {
        assert_eq!(size_of::<hailo_stream_read_async_completion_info_t>(), 32);
        assert_eq!(align_of::<hailo_stream_read_async_completion_info_t>(), 8);
        assert_eq!(
            offset_of!(hailo_stream_read_async_completion_info_t, status),
            0
        );
        assert_eq!(
            offset_of!(hailo_stream_read_async_completion_info_t, buffer_addr),
            8
        );
        assert_eq!(
            offset_of!(hailo_stream_read_async_completion_info_t, buffer_size),
            16
        );
        assert_eq!(
            offset_of!(hailo_stream_read_async_completion_info_t, opaque),
            24
        );
    }

    // --- Notification message structs ----------------------------------------

    #[test]
    fn notification_message_struct_sizes() {
        assert_eq!(
            size_of::<hailo_rx_error_notification_message_t>(),
            12
        );
        assert_eq!(
            size_of::<hailo_debug_notification_message_t>(),
            20
        );
        assert_eq!(
            size_of::<hailo_health_monitor_dataflow_shutdown_notification_message_t>(),
            8
        );
        assert_eq!(
            size_of::<hailo_health_monitor_temperature_alarm_notification_message_t>(),
            16
        );
        assert_eq!(
            size_of::<hailo_health_monitor_overcurrent_alert_notification_message_t>(),
            12
        );
        assert_eq!(
            size_of::<hailo_health_monitor_lcu_ecc_error_notification_message_t>(),
            2
        );
        assert_eq!(
            size_of::<hailo_health_monitor_cpu_ecc_notification_message_t>(),
            4
        );
        assert_eq!(
            size_of::<hailo_context_switch_breakpoint_reached_message_t>(),
            12
        );
        assert_eq!(
            size_of::<hailo_health_monitor_clock_changed_notification_message_t>(),
            8
        );
        assert_eq!(
            size_of::<hailo_hw_infer_manager_infer_done_notification_message_t>(),
            4
        );
        assert_eq!(
            size_of::<hailo_start_update_cache_offset_notification_message_t>(),
            8
        );
        assert_eq!(
            size_of::<hailo_context_switch_run_time_error_message_t>(),
            12
        );
        assert_eq!(
            size_of::<hailo_throttling_state_change_message_t>(),
            2
        );
    }

    #[test]
    fn hailo_notification_message_parameters_t_layout() {
        // Largest variant: hailo_debug_notification_message_t (20 bytes).
        // Highest-aligned variant: hailo_start_update_cache_offset_notification_message_t
        //   contains u64 (align 8).  Union size = round_up(20, 8) = 24.
        assert_eq!(size_of::<hailo_notification_message_parameters_t>(), 24);
        assert_eq!(align_of::<hailo_notification_message_parameters_t>(), 8);
    }

    #[test]
    fn hailo_notification_t_layout() {
        // id(4) sequence(4) [body aligns to 8 → offset 8] body(24) = 32
        assert_eq!(size_of::<hailo_notification_t>(), 32);
        assert_eq!(align_of::<hailo_notification_t>(), 8);
        assert_eq!(offset_of!(hailo_notification_t, id), 0);
        assert_eq!(offset_of!(hailo_notification_t, sequence), 4);
        assert_eq!(offset_of!(hailo_notification_t, body), 8);
    }

    // --- Stream / VStream info unions ----------------------------------------

    #[test]
    fn hailo_stream_info_shape_pair_t_layout() {
        assert_eq!(size_of::<hailo_stream_info_shape_pair_t>(), 24);
        assert_eq!(align_of::<hailo_stream_info_shape_pair_t>(), 4);
        assert_eq!(offset_of!(hailo_stream_info_shape_pair_t, shape), 0);
        assert_eq!(offset_of!(hailo_stream_info_shape_pair_t, hw_shape), 12);
    }

    #[test]
    fn hailo_stream_info_shape_t_layout() {
        // union { pair(24, align 4), nms_info(164, align 4) } → size 164, align 4
        assert_eq!(size_of::<hailo_stream_info_shape_t>(), 164);
        assert_eq!(align_of::<hailo_stream_info_shape_t>(), 4);
    }

    #[test]
    fn hailo_stream_info_t_layout() {
        // shape(164) hw_data_bytes(4) hw_frame_size(4) format(12) direction(4)
        // index(1) name([i8;128]) [pad 3] quant_info(16, align 4 at 320) is_mux(1)
        // → end 337 → pad to 340
        assert_eq!(size_of::<hailo_stream_info_t>(), 340);
        assert_eq!(align_of::<hailo_stream_info_t>(), 4);
        assert_eq!(offset_of!(hailo_stream_info_t, shape), 0);
        assert_eq!(offset_of!(hailo_stream_info_t, hw_data_bytes), 164);
        assert_eq!(offset_of!(hailo_stream_info_t, hw_frame_size), 168);
        assert_eq!(offset_of!(hailo_stream_info_t, format), 172);
        assert_eq!(offset_of!(hailo_stream_info_t, direction), 184);
        assert_eq!(offset_of!(hailo_stream_info_t, index), 188);
        assert_eq!(offset_of!(hailo_stream_info_t, name), 189);
        assert_eq!(offset_of!(hailo_stream_info_t, quant_info), 320);
        assert_eq!(offset_of!(hailo_stream_info_t, is_mux), 336);
    }

    #[test]
    fn hailo_vstream_info_shape_t_layout() {
        // union { 3d_shape(12, align 4), nms_shape(16, align 4) } → size 16, align 4
        assert_eq!(size_of::<hailo_vstream_info_shape_t>(), 16);
        assert_eq!(align_of::<hailo_vstream_info_shape_t>(), 4);
    }

    #[test]
    fn hailo_vstream_info_t_layout() {
        // name([i8;128]) network_name([i8;257]) [pad 3 to align direction to 4]
        // direction(4 at 388) format(12 at 392) shape(16 at 404) quant_info(16 at 420) = 436
        assert_eq!(size_of::<hailo_vstream_info_t>(), 436);
        assert_eq!(align_of::<hailo_vstream_info_t>(), 4);
        assert_eq!(offset_of!(hailo_vstream_info_t, name), 0);
        assert_eq!(offset_of!(hailo_vstream_info_t, network_name), 128);
        assert_eq!(offset_of!(hailo_vstream_info_t, direction), 388);
        assert_eq!(offset_of!(hailo_vstream_info_t, format), 392);
        assert_eq!(offset_of!(hailo_vstream_info_t, shape), 404);
        assert_eq!(offset_of!(hailo_vstream_info_t, quant_info), 420);
    }

    // --- Power / temperature / health ----------------------------------------

    #[test]
    fn hailo_power_measurement_data_t_layout() {
        assert_eq!(size_of::<hailo_power_measurement_data_t>(), 20);
        assert_eq!(align_of::<hailo_power_measurement_data_t>(), 4);
        assert_eq!(
            offset_of!(hailo_power_measurement_data_t, average_value),
            0
        );
        assert_eq!(
            offset_of!(
                hailo_power_measurement_data_t,
                average_time_value_milliseconds
            ),
            4
        );
        assert_eq!(offset_of!(hailo_power_measurement_data_t, min_value), 8);
        assert_eq!(offset_of!(hailo_power_measurement_data_t, max_value), 12);
        assert_eq!(
            offset_of!(hailo_power_measurement_data_t, total_number_of_samples),
            16
        );
    }

    #[test]
    fn hailo_chip_temperature_info_t_layout() {
        // ts0(4) ts1(4) sample_count(2) → end 10 → pad to 12
        assert_eq!(size_of::<hailo_chip_temperature_info_t>(), 12);
        assert_eq!(align_of::<hailo_chip_temperature_info_t>(), 4);
        assert_eq!(
            offset_of!(hailo_chip_temperature_info_t, ts0_temperature),
            0
        );
        assert_eq!(
            offset_of!(hailo_chip_temperature_info_t, ts1_temperature),
            4
        );
        assert_eq!(offset_of!(hailo_chip_temperature_info_t, sample_count), 8);
    }

    #[test]
    fn hailo_throttling_level_t_layout() {
        assert_eq!(size_of::<hailo_throttling_level_t>(), 12);
        assert_eq!(align_of::<hailo_throttling_level_t>(), 4);
        assert_eq!(
            offset_of!(hailo_throttling_level_t, temperature_threshold),
            0
        );
        assert_eq!(
            offset_of!(
                hailo_throttling_level_t,
                hysteresis_temperature_threshold
            ),
            4
        );
        assert_eq!(
            offset_of!(hailo_throttling_level_t, throttling_nn_clock_freq),
            8
        );
    }

    #[test]
    fn hailo_health_info_t_layout() {
        // overcurrent_protection_active(1) current_overcurrent_zone(1)
        // [pad 2] red_overcurrent_threshold(f32 at 4)
        // overcurrent_throttling_active(1 at 8) temperature_throttling_active(1 at 9)
        // current_temperature_zone(1 at 10) current_temperature_throttling_level(i8 at 11)
        // temperature_throttling_levels(48, align 4 at 12)
        // orange_temperature_threshold(i32 at 60) ... → 84, align 4
        assert_eq!(size_of::<hailo_health_info_t>(), 84);
        assert_eq!(align_of::<hailo_health_info_t>(), 4);
        assert_eq!(
            offset_of!(hailo_health_info_t, overcurrent_protection_active),
            0
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, current_overcurrent_zone),
            1
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, red_overcurrent_threshold),
            4
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, overcurrent_throttling_active),
            8
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, temperature_throttling_active),
            9
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, current_temperature_zone),
            10
        );
        assert_eq!(
            offset_of!(
                hailo_health_info_t,
                current_temperature_throttling_level
            ),
            11
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, temperature_throttling_levels),
            12
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, orange_temperature_threshold),
            60
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, requested_overcurrent_clock_freq),
            76
        );
        assert_eq!(
            offset_of!(hailo_health_info_t, requested_temperature_clock_freq),
            80
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_performance_stats_t_layout() {
        // cpu_utilization(4) [pad 4] ram_size_total(i64=8) ram_size_used(8)
        // nnc_utilization(4) ddr_noc(4) dsp(4) → end 36 → pad to 40
        assert_eq!(size_of::<hailo_performance_stats_t>(), 40);
        assert_eq!(align_of::<hailo_performance_stats_t>(), 8);
        assert_eq!(offset_of!(hailo_performance_stats_t, cpu_utilization), 0);
        assert_eq!(offset_of!(hailo_performance_stats_t, ram_size_total), 8);
        assert_eq!(offset_of!(hailo_performance_stats_t, ram_size_used), 16);
        assert_eq!(offset_of!(hailo_performance_stats_t, nnc_utilization), 24);
        assert_eq!(
            offset_of!(hailo_performance_stats_t, ddr_noc_total_transactions),
            28
        );
        assert_eq!(offset_of!(hailo_performance_stats_t, dsp_utilization), 32);
    }

    #[test]
    fn hailo_health_stats_t_layout() {
        assert_eq!(size_of::<hailo_health_stats_t>(), 12);
        assert_eq!(align_of::<hailo_health_stats_t>(), 4);
        assert_eq!(offset_of!(hailo_health_stats_t, on_die_temperature), 0);
        assert_eq!(offset_of!(hailo_health_stats_t, on_die_voltage), 4);
        assert_eq!(offset_of!(hailo_health_stats_t, bist_failure_mask), 8);
    }

    // --- Network / configure params ------------------------------------------

    #[test]
    fn hailo_network_parameters_t_layout() {
        assert_eq!(size_of::<hailo_network_parameters_t>(), 2);
        assert_eq!(align_of::<hailo_network_parameters_t>(), 2);
        assert_eq!(offset_of!(hailo_network_parameters_t, batch_size), 0);
    }

    #[test]
    fn hailo_network_parameters_by_name_t_layout() {
        // name([i8;257]) [pad 1] network_params(u16 at 258) → end 260, align 2
        assert_eq!(size_of::<hailo_network_parameters_by_name_t>(), 260);
        assert_eq!(align_of::<hailo_network_parameters_by_name_t>(), 2);
        assert_eq!(offset_of!(hailo_network_parameters_by_name_t, name), 0);
        assert_eq!(
            offset_of!(hailo_network_parameters_by_name_t, network_params),
            258
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_configure_network_group_params_t_layout() {
        // name(128) batch_size(2 at 128) [pad 2] power_mode(4 at 132) latency(4 at 136)
        // enable_kv_cache(1 at 140) [pad 3] stream_params_by_name_count(usize=8 at 144)
        // stream_params_by_name(40×144=5760, align 4 at 152) → end 5912
        // network_params_by_name_count(usize=8 at 5912) → end 5920
        // network_params_by_name(8×260=2080, align 2 at 5920) → end 8000
        assert_eq!(
            size_of::<hailo_configure_network_group_params_t>(),
            8000
        );
        assert_eq!(
            align_of::<hailo_configure_network_group_params_t>(),
            8
        );
        assert_eq!(
            offset_of!(hailo_configure_network_group_params_t, name),
            0
        );
        assert_eq!(
            offset_of!(hailo_configure_network_group_params_t, batch_size),
            128
        );
        assert_eq!(
            offset_of!(hailo_configure_network_group_params_t, power_mode),
            132
        );
        assert_eq!(
            offset_of!(hailo_configure_network_group_params_t, latency),
            136
        );
        assert_eq!(
            offset_of!(hailo_configure_network_group_params_t, enable_kv_cache),
            140
        );
        assert_eq!(
            offset_of!(
                hailo_configure_network_group_params_t,
                stream_params_by_name_count
            ),
            144
        );
        assert_eq!(
            offset_of!(
                hailo_configure_network_group_params_t,
                stream_params_by_name
            ),
            152
        );
        assert_eq!(
            offset_of!(
                hailo_configure_network_group_params_t,
                network_params_by_name_count
            ),
            5912
        );
        assert_eq!(
            offset_of!(
                hailo_configure_network_group_params_t,
                network_params_by_name
            ),
            5920
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_configure_params_t_layout() {
        // network_group_params_count(usize=8) network_group_params(8×8000=64000, align 8) = 64008
        assert_eq!(size_of::<hailo_configure_params_t>(), 64008);
        assert_eq!(align_of::<hailo_configure_params_t>(), 8);
        assert_eq!(
            offset_of!(hailo_configure_params_t, network_group_params_count),
            0
        );
        assert_eq!(
            offset_of!(hailo_configure_params_t, network_group_params),
            8
        );
    }

    #[test]
    fn hailo_activate_network_group_params_t_layout() {
        assert_eq!(size_of::<hailo_activate_network_group_params_t>(), 1);
    }

    #[test]
    fn hailo_network_group_info_t_layout() {
        assert_eq!(size_of::<hailo_network_group_info_t>(), 129);
        assert_eq!(align_of::<hailo_network_group_info_t>(), 1);
        assert_eq!(offset_of!(hailo_network_group_info_t, name), 0);
        assert_eq!(
            offset_of!(hailo_network_group_info_t, is_multi_context),
            128
        );
    }

    #[test]
    fn hailo_layer_name_t_layout() {
        assert_eq!(size_of::<hailo_layer_name_t>(), 128);
        assert_eq!(offset_of!(hailo_layer_name_t, name), 0);
    }

    #[test]
    fn hailo_network_info_t_layout() {
        assert_eq!(size_of::<hailo_network_info_t>(), 257);
        assert_eq!(offset_of!(hailo_network_info_t, name), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_stream_raw_buffer_t_layout() {
        // buffer(ptr=8) size(usize=8) = 16
        assert_eq!(size_of::<hailo_stream_raw_buffer_t>(), 16);
        assert_eq!(align_of::<hailo_stream_raw_buffer_t>(), 8);
        assert_eq!(offset_of!(hailo_stream_raw_buffer_t, buffer), 0);
        assert_eq!(offset_of!(hailo_stream_raw_buffer_t, size), 8);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn hailo_stream_raw_buffer_by_name_t_layout() {
        // name([i8;128]) raw_buffer(16, align 8, at 128) = 144
        assert_eq!(size_of::<hailo_stream_raw_buffer_by_name_t>(), 144);
        assert_eq!(align_of::<hailo_stream_raw_buffer_by_name_t>(), 8);
        assert_eq!(offset_of!(hailo_stream_raw_buffer_by_name_t, name), 0);
        assert_eq!(
            offset_of!(hailo_stream_raw_buffer_by_name_t, raw_buffer),
            128
        );
    }

    #[test]
    fn hailo_latency_measurement_result_t_layout() {
        assert_eq!(size_of::<hailo_latency_measurement_result_t>(), 8);
        assert_eq!(align_of::<hailo_latency_measurement_result_t>(), 8);
        assert_eq!(
            offset_of!(hailo_latency_measurement_result_t, avg_hw_latency_ms),
            0
        );
    }

    #[test]
    fn hailo_rate_limit_t_layout() {
        // stream_name([i8;128]) rate(u32, align 4, at 128) = 132
        assert_eq!(size_of::<hailo_rate_limit_t>(), 132);
        assert_eq!(align_of::<hailo_rate_limit_t>(), 4);
        assert_eq!(offset_of!(hailo_rate_limit_t, stream_name), 0);
        assert_eq!(offset_of!(hailo_rate_limit_t, rate), 128);
    }

    #[test]
    fn hailo_i2c_slave_config_t_layout() {
        // endianness(c_int=4) slave_address(u16=2 at 4) register_address_size(u8=1 at 6)
        // bus_index(u8=1 at 7) should_hold_bus(bool=1 at 8) → end 9 → pad to 12
        assert_eq!(size_of::<hailo_i2c_slave_config_t>(), 12);
        assert_eq!(align_of::<hailo_i2c_slave_config_t>(), 4);
        assert_eq!(offset_of!(hailo_i2c_slave_config_t, endianness), 0);
        assert_eq!(offset_of!(hailo_i2c_slave_config_t, slave_address), 4);
        assert_eq!(
            offset_of!(hailo_i2c_slave_config_t, register_address_size),
            6
        );
        assert_eq!(offset_of!(hailo_i2c_slave_config_t, bus_index), 7);
        assert_eq!(offset_of!(hailo_i2c_slave_config_t, should_hold_bus), 8);
    }
}
