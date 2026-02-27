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
