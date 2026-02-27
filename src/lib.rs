//! Raw FFI bindings to the HailoRT runtime library.
//!
//! These bindings mirror the C API declared in `<hailo/hailort.h>` from the
//! [hailo-ai/hailort](https://github.com/hailo-ai/hailort) open-source project.
//! They give Rust code direct access to the Hailo AI HAT+ on Raspberry Pi.
//!
//! # Safety
//! Every function in the [`extern "C"`] block is `unsafe`. Callers must
//! uphold all invariants described in the upstream C API documentation:
//! <https://hailo.ai/developer-zone/>
//!
//! # Linking
//! The `build.rs` script attempts `pkg-config` first, then falls back to
//! searching `/usr/lib` and `/usr/local/lib` for `libhailort.so`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_float, c_double, c_int, c_uchar, c_void};

// ---------------------------------------------------------------------------
// Primitive type aliases (mirror C typedef in hailort.h)
// ---------------------------------------------------------------------------

pub type float32_t = c_float;
pub type float64_t = c_double;
pub type nms_bbox_counter_t = u16;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const HAILO_MAX_NAME_SIZE: usize = 128;
pub const HAILO_MAX_STREAM_NAME_SIZE: usize = HAILO_MAX_NAME_SIZE;
pub const HAILO_MAX_NETWORK_GROUP_NAME_SIZE: usize = HAILO_MAX_NAME_SIZE;
/// `HAILO_MAX_NETWORK_GROUP_NAME_SIZE + 1 + HAILO_MAX_NAME_SIZE`
pub const HAILO_MAX_NETWORK_NAME_SIZE: usize =
    HAILO_MAX_NETWORK_GROUP_NAME_SIZE + 1 + HAILO_MAX_NAME_SIZE;

pub const HAILO_MAX_BOARD_NAME_LENGTH: usize = 32;
pub const HAILO_MAX_DEVICE_ID_LENGTH: usize = 32;
pub const HAILO_MAX_SERIAL_NUMBER_LENGTH: usize = 16;
pub const HAILO_MAX_PART_NUMBER_LENGTH: usize = 16;
pub const HAILO_MAX_PRODUCT_NAME_LENGTH: usize = 42;

pub const HAILO_MAX_STREAMS_COUNT: usize = 40;
pub const HAILO_MAX_NETWORK_GROUPS: usize = 8;
pub const HAILO_MAX_NETWORKS_IN_NETWORK_GROUP: usize = 8;

pub const HAILO_SOC_ID_LENGTH: usize = 32;
pub const HAILO_ETH_MAC_LENGTH: usize = 6;
pub const HAILO_UNIT_LEVEL_TRACKING_BYTES_LENGTH: usize = 12;
pub const HAILO_SOC_PM_VALUES_BYTES_LENGTH: usize = 24;
pub const HAILO_MAX_TEMPERATURE_THROTTLING_LEVELS_NUMBER: usize = 4;

pub const MAX_NUMBER_OF_PLANES: usize = 4;
pub const NUMBER_OF_PLANES_NV12_NV21: usize = 2;
pub const NUMBER_OF_PLANES_I420: usize = 3;

pub const HAILO_INFINITE: u32 = u32::MAX;
pub const HAILO_PCIE_ANY_DOMAIN: u32 = u32::MAX;
pub const HAILO_RANDOM_SEED: u32 = u32::MAX;

pub const HAILO_DEFAULT_BATCH_SIZE: u16 = 0;
pub const HAILO_DEFAULT_VSTREAM_QUEUE_SIZE: u32 = 2;
pub const HAILO_DEFAULT_VSTREAM_TIMEOUT_MS: u32 = 10_000;
pub const HAILO_DEFAULT_ASYNC_INFER_TIMEOUT_MS: u32 = 10_000;
pub const HAILO_DEFAULT_ASYNC_INFER_QUEUE_SIZE: u32 = 2;
pub const HAILO_DEFAULT_DEVICE_COUNT: u32 = 1;
pub const HAILO_DEFAULT_ETH_SCAN_TIMEOUT_MS: u32 = 10_000;
pub const HAILO_DEFAULT_ETH_DEVICE_PORT: u16 = 0;
pub const HAILO_DEFAULT_ETH_MAX_PAYLOAD_SIZE: u32 = 1456;
pub const HAILO_DEFAULT_ETH_MAX_NUMBER_OF_RETRIES: u32 = 3;

pub const HAILO_SCHEDULER_PRIORITY_NORMAL: u8 = 16;
pub const HAILO_SCHEDULER_PRIORITY_MAX: u8 = 31;
pub const HAILO_SCHEDULER_PRIORITY_MIN: u8 = 0;

// ---------------------------------------------------------------------------
// Opaque handle types
//
// In C these are `typedef struct _foo *foo;` — incomplete struct pointers.
// Represent them as uninhabited enums so Rust never constructs them directly.
// ---------------------------------------------------------------------------

pub enum hailo_device_opaque {}
/// Opaque handle to a physical Hailo device.
pub type hailo_device = *mut hailo_device_opaque;

pub enum hailo_vdevice_opaque {}
/// Opaque handle to a virtual device (may span multiple physical devices).
pub type hailo_vdevice = *mut hailo_vdevice_opaque;

pub enum hailo_hef_opaque {}
/// Opaque handle to a loaded HEF (Hailo Executable Format) model file.
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

/// `hailo_scan_devices_params_t` is opaque; pass a null pointer.
/// Only `NULL` is accepted by the current API.
pub type hailo_scan_devices_params_t = c_void;

// ---------------------------------------------------------------------------
// hailo_status — return code for every API call
// ---------------------------------------------------------------------------

pub type hailo_status = c_int;

pub const HAILO_SUCCESS: hailo_status = 0;
pub const HAILO_UNINITIALIZED: hailo_status = 1;
pub const HAILO_INVALID_ARGUMENT: hailo_status = 2;
pub const HAILO_OUT_OF_HOST_MEMORY: hailo_status = 3;
pub const HAILO_TIMEOUT: hailo_status = 4;
pub const HAILO_INSUFFICIENT_BUFFER: hailo_status = 5;
pub const HAILO_INVALID_OPERATION: hailo_status = 6;
pub const HAILO_NOT_IMPLEMENTED: hailo_status = 7;
pub const HAILO_INTERNAL_FAILURE: hailo_status = 8;
pub const HAILO_DATA_ALIGNMENT_FAILURE: hailo_status = 9;
pub const HAILO_CHUNK_TOO_LARGE: hailo_status = 10;
pub const HAILO_INVALID_LOGGER_LEVEL: hailo_status = 11;
pub const HAILO_CLOSE_FAILURE: hailo_status = 12;
pub const HAILO_OPEN_FILE_FAILURE: hailo_status = 13;
pub const HAILO_FILE_OPERATION_FAILURE: hailo_status = 14;
pub const HAILO_UNSUPPORTED_CONTROL_PROTOCOL_VERSION: hailo_status = 15;
pub const HAILO_UNSUPPORTED_FW_VERSION: hailo_status = 16;
pub const HAILO_INVALID_CONTROL_RESPONSE: hailo_status = 17;
pub const HAILO_FW_CONTROL_FAILURE: hailo_status = 18;
pub const HAILO_ETH_FAILURE: hailo_status = 19;
pub const HAILO_ETH_INTERFACE_NOT_FOUND: hailo_status = 20;
pub const HAILO_ETH_RECV_FAILURE: hailo_status = 21;
pub const HAILO_ETH_SEND_FAILURE: hailo_status = 22;
pub const HAILO_INVALID_FIRMWARE: hailo_status = 23;
pub const HAILO_INVALID_CONTEXT_COUNT: hailo_status = 24;
pub const HAILO_INVALID_FRAME: hailo_status = 25;
pub const HAILO_INVALID_HEF: hailo_status = 26;
pub const HAILO_PCIE_NOT_SUPPORTED_ON_PLATFORM: hailo_status = 27;
pub const HAILO_INTERRUPTED_BY_SIGNAL: hailo_status = 28;
pub const HAILO_START_VDMA_CHANNEL_FAIL: hailo_status = 29;
pub const HAILO_SYNC_VDMA_BUFFER_FAIL: hailo_status = 30;
pub const HAILO_STOP_VDMA_CHANNEL_FAIL: hailo_status = 31;
pub const HAILO_CLOSE_VDMA_CHANNEL_FAIL: hailo_status = 32;
pub const HAILO_ATR_TABLES_CONF_VALIDATION_FAIL: hailo_status = 33;
pub const HAILO_EVENT_CREATE_FAIL: hailo_status = 34;
pub const HAILO_READ_EVENT_FAIL: hailo_status = 35;
pub const HAILO_DRIVER_OPERATION_FAILED: hailo_status = 36;
pub const HAILO_INVALID_FIRMWARE_MAGIC: hailo_status = 37;
pub const HAILO_INVALID_FIRMWARE_CODE_SIZE: hailo_status = 38;
pub const HAILO_INVALID_KEY_CERTIFICATE_SIZE: hailo_status = 39;
pub const HAILO_INVALID_CONTENT_CERTIFICATE_SIZE: hailo_status = 40;
pub const HAILO_MISMATCHING_FIRMWARE_BUFFER_SIZES: hailo_status = 41;
pub const HAILO_INVALID_FIRMWARE_CPU_ID: hailo_status = 42;
pub const HAILO_CONTROL_RESPONSE_MD5_MISMATCH: hailo_status = 43;
pub const HAILO_GET_CONTROL_RESPONSE_FAIL: hailo_status = 44;
pub const HAILO_GET_D2H_EVENT_MESSAGE_FAIL: hailo_status = 45;
pub const HAILO_MUTEX_INIT_FAIL: hailo_status = 46;
pub const HAILO_OUT_OF_DESCRIPTORS: hailo_status = 47;
pub const HAILO_UNSUPPORTED_OPCODE: hailo_status = 48;
pub const HAILO_USER_MODE_RATE_LIMITER_NOT_SUPPORTED: hailo_status = 49;
pub const HAILO_RATE_LIMIT_MAXIMUM_BANDWIDTH_EXCEEDED: hailo_status = 50;
pub const HAILO_ANSI_TO_UTF16_CONVERSION_FAILED: hailo_status = 51;
pub const HAILO_UTF16_TO_ANSI_CONVERSION_FAILED: hailo_status = 52;
pub const HAILO_UNEXPECTED_INTERFACE_INFO_FAILURE: hailo_status = 53;
pub const HAILO_UNEXPECTED_ARP_TABLE_FAILURE: hailo_status = 54;
pub const HAILO_MAC_ADDRESS_NOT_FOUND: hailo_status = 55;
pub const HAILO_NO_IPV4_INTERFACES_FOUND: hailo_status = 56;
pub const HAILO_SHUTDOWN_EVENT_SIGNALED: hailo_status = 57;
pub const HAILO_THREAD_ALREADY_ACTIVATED: hailo_status = 58;
pub const HAILO_THREAD_NOT_ACTIVATED: hailo_status = 59;
pub const HAILO_THREAD_NOT_JOINABLE: hailo_status = 60;
pub const HAILO_NOT_FOUND: hailo_status = 61;
pub const HAILO_COMMUNICATION_CLOSED: hailo_status = 62;
pub const HAILO_STREAM_ABORT: hailo_status = 63;
pub const HAILO_DRIVER_NOT_INSTALLED: hailo_status = 64;
pub const HAILO_NOT_AVAILABLE: hailo_status = 65;
pub const HAILO_TRAFFIC_CONTROL_FAILURE: hailo_status = 66;
pub const HAILO_INVALID_SECOND_STAGE: hailo_status = 67;
pub const HAILO_INVALID_PIPELINE: hailo_status = 68;
pub const HAILO_NETWORK_GROUP_NOT_ACTIVATED: hailo_status = 69;
pub const HAILO_VSTREAM_PIPELINE_NOT_ACTIVATED: hailo_status = 70;
pub const HAILO_OUT_OF_FW_MEMORY: hailo_status = 71;
pub const HAILO_STREAM_NOT_ACTIVATED: hailo_status = 72;
pub const HAILO_DEVICE_IN_USE: hailo_status = 73;
pub const HAILO_OUT_OF_PHYSICAL_DEVICES: hailo_status = 74;
pub const HAILO_INVALID_DEVICE_ARCHITECTURE: hailo_status = 75;
pub const HAILO_INVALID_DRIVER_VERSION: hailo_status = 76;
pub const HAILO_RPC_FAILED: hailo_status = 77;
pub const HAILO_INVALID_SERVICE_VERSION: hailo_status = 78;
pub const HAILO_NOT_SUPPORTED: hailo_status = 79;
pub const HAILO_NMS_BURST_INVALID_DATA: hailo_status = 80;
pub const HAILO_OUT_OF_HOST_CMA_MEMORY: hailo_status = 81;
pub const HAILO_QUEUE_IS_FULL: hailo_status = 82;
pub const HAILO_DMA_MAPPING_ALREADY_EXISTS: hailo_status = 83;
pub const HAILO_CANT_MEET_BUFFER_REQUIREMENTS: hailo_status = 84;
pub const HAILO_DRIVER_INVALID_RESPONSE: hailo_status = 85;
pub const HAILO_DRIVER_INVALID_IOCTL: hailo_status = 86;
pub const HAILO_DRIVER_TIMEOUT: hailo_status = 87;
pub const HAILO_DRIVER_INTERRUPTED: hailo_status = 88;
pub const HAILO_CONNECTION_REFUSED: hailo_status = 89;
pub const HAILO_DRIVER_WAIT_CANCELED: hailo_status = 90;
pub const HAILO_HEF_FILE_CORRUPTED: hailo_status = 91;
pub const HAILO_HEF_NOT_SUPPORTED: hailo_status = 92;
pub const HAILO_HEF_NOT_COMPATIBLE_WITH_DEVICE: hailo_status = 93;
pub const HAILO_INVALID_HEF_USE: hailo_status = 94;
pub const HAILO_OPERATION_ABORTED: hailo_status = 95;
pub const HAILO_DEVICE_NOT_CONNECTED: hailo_status = 96;
pub const HAILO_DEVICE_TEMPORARILY_UNAVAILABLE: hailo_status = 97;

// ---------------------------------------------------------------------------
// Enumerations
//
// All C enums are represented as `type = c_int` + named constants.  This is
// the safest FFI pattern: it never triggers UB on an unknown discriminant
// the way a Rust #[repr(C)] enum would.
// ---------------------------------------------------------------------------

pub type hailo_dvm_options_t = c_int;
pub const HAILO_DVM_OPTIONS_VDD_CORE: hailo_dvm_options_t = 0;
pub const HAILO_DVM_OPTIONS_VDD_IO: hailo_dvm_options_t = 1;
pub const HAILO_DVM_OPTIONS_MIPI_AVDD: hailo_dvm_options_t = 2;
pub const HAILO_DVM_OPTIONS_MIPI_AVDD_H: hailo_dvm_options_t = 3;
pub const HAILO_DVM_OPTIONS_USB_AVDD_IO: hailo_dvm_options_t = 4;
pub const HAILO_DVM_OPTIONS_VDD_TOP: hailo_dvm_options_t = 5;
pub const HAILO_DVM_OPTIONS_USB_AVDD_IO_HV: hailo_dvm_options_t = 6;
pub const HAILO_DVM_OPTIONS_AVDD_H: hailo_dvm_options_t = 7;
pub const HAILO_DVM_OPTIONS_SDIO_VDD_IO: hailo_dvm_options_t = 8;
pub const HAILO_DVM_OPTIONS_OVERCURRENT_PROTECTION: hailo_dvm_options_t = 9;
pub const HAILO_DVM_OPTIONS_COUNT: hailo_dvm_options_t = 10;
pub const HAILO_DVM_OPTIONS_AUTO: hailo_dvm_options_t = c_int::MAX;

pub type hailo_power_measurement_types_t = c_int;
pub const HAILO_POWER_MEASUREMENT_TYPES__SHUNT_VOLTAGE: hailo_power_measurement_types_t = 0;
pub const HAILO_POWER_MEASUREMENT_TYPES__BUS_VOLTAGE: hailo_power_measurement_types_t = 1;
pub const HAILO_POWER_MEASUREMENT_TYPES__POWER: hailo_power_measurement_types_t = 2;
pub const HAILO_POWER_MEASUREMENT_TYPES__CURRENT: hailo_power_measurement_types_t = 3;
pub const HAILO_POWER_MEASUREMENT_TYPES__COUNT: hailo_power_measurement_types_t = 4;
pub const HAILO_POWER_MEASUREMENT_TYPES__AUTO: hailo_power_measurement_types_t = c_int::MAX;

pub type hailo_sampling_period_t = c_int;
pub const HAILO_SAMPLING_PERIOD_140US: hailo_sampling_period_t = 0;
pub const HAILO_SAMPLING_PERIOD_204US: hailo_sampling_period_t = 1;
pub const HAILO_SAMPLING_PERIOD_332US: hailo_sampling_period_t = 2;
pub const HAILO_SAMPLING_PERIOD_588US: hailo_sampling_period_t = 3;
pub const HAILO_SAMPLING_PERIOD_1100US: hailo_sampling_period_t = 4;
pub const HAILO_SAMPLING_PERIOD_2116US: hailo_sampling_period_t = 5;
pub const HAILO_SAMPLING_PERIOD_4156US: hailo_sampling_period_t = 6;
pub const HAILO_SAMPLING_PERIOD_8244US: hailo_sampling_period_t = 7;

pub type hailo_averaging_factor_t = c_int;
pub const HAILO_AVERAGE_FACTOR_1: hailo_averaging_factor_t = 0;
pub const HAILO_AVERAGE_FACTOR_4: hailo_averaging_factor_t = 1;
pub const HAILO_AVERAGE_FACTOR_16: hailo_averaging_factor_t = 2;
pub const HAILO_AVERAGE_FACTOR_64: hailo_averaging_factor_t = 3;
pub const HAILO_AVERAGE_FACTOR_128: hailo_averaging_factor_t = 4;
pub const HAILO_AVERAGE_FACTOR_256: hailo_averaging_factor_t = 5;
pub const HAILO_AVERAGE_FACTOR_512: hailo_averaging_factor_t = 6;
pub const HAILO_AVERAGE_FACTOR_1024: hailo_averaging_factor_t = 7;

pub type hailo_measurement_buffer_index_t = c_int;
pub const HAILO_MEASUREMENT_BUFFER_INDEX_0: hailo_measurement_buffer_index_t = 0;
pub const HAILO_MEASUREMENT_BUFFER_INDEX_1: hailo_measurement_buffer_index_t = 1;
pub const HAILO_MEASUREMENT_BUFFER_INDEX_2: hailo_measurement_buffer_index_t = 2;
pub const HAILO_MEASUREMENT_BUFFER_INDEX_3: hailo_measurement_buffer_index_t = 3;

pub type hailo_device_type_t = c_int;
pub const HAILO_DEVICE_TYPE_PCIE: hailo_device_type_t = 0;
pub const HAILO_DEVICE_TYPE_ETH: hailo_device_type_t = 1;
pub const HAILO_DEVICE_TYPE_INTEGRATED: hailo_device_type_t = 2;

pub type hailo_scheduling_algorithm_t = c_int;
pub const HAILO_SCHEDULING_ALGORITHM_NONE: hailo_scheduling_algorithm_t = 0;
pub const HAILO_SCHEDULING_ALGORITHM_ROUND_ROBIN: hailo_scheduling_algorithm_t = 1;

pub type hailo_device_architecture_t = c_int;
pub const HAILO_ARCH_HAILO8_A0: hailo_device_architecture_t = 0;
pub const HAILO_ARCH_HAILO8: hailo_device_architecture_t = 1;
pub const HAILO_ARCH_HAILO8L: hailo_device_architecture_t = 2;
pub const HAILO_ARCH_HAILO15H: hailo_device_architecture_t = 3;
pub const HAILO_ARCH_HAILO15L: hailo_device_architecture_t = 4;
pub const HAILO_ARCH_HAILO15M: hailo_device_architecture_t = 5;
pub const HAILO_ARCH_HAILO10H: hailo_device_architecture_t = 6;
pub const HAILO_ARCH_HAILO12L: hailo_device_architecture_t = 7;

pub type hailo_cpu_id_t = c_int;
pub const HAILO_CPU_ID_0: hailo_cpu_id_t = 0;
pub const HAILO_CPU_ID_1: hailo_cpu_id_t = 1;

pub type hailo_device_boot_source_t = c_int;
pub const HAILO_DEVICE_BOOT_SOURCE_INVALID: hailo_device_boot_source_t = 0;
pub const HAILO_DEVICE_BOOT_SOURCE_PCIE: hailo_device_boot_source_t = 1;
pub const HAILO_DEVICE_BOOT_SOURCE_FLASH: hailo_device_boot_source_t = 2;

pub type hailo_endianness_t = c_int;
pub const HAILO_BIG_ENDIAN: hailo_endianness_t = 0;
pub const HAILO_LITTLE_ENDIAN: hailo_endianness_t = 1;

pub type hailo_format_type_t = c_int;
pub const HAILO_FORMAT_TYPE_AUTO: hailo_format_type_t = 0;
pub const HAILO_FORMAT_TYPE_UINT8: hailo_format_type_t = 1;
pub const HAILO_FORMAT_TYPE_UINT16: hailo_format_type_t = 2;
pub const HAILO_FORMAT_TYPE_FLOAT32: hailo_format_type_t = 3;

pub type hailo_format_order_t = c_int;
pub const HAILO_FORMAT_ORDER_AUTO: hailo_format_order_t = 0;
pub const HAILO_FORMAT_ORDER_NHWC: hailo_format_order_t = 1;
pub const HAILO_FORMAT_ORDER_NHCW: hailo_format_order_t = 2;
pub const HAILO_FORMAT_ORDER_FCR: hailo_format_order_t = 3;
pub const HAILO_FORMAT_ORDER_F8CR: hailo_format_order_t = 4;
pub const HAILO_FORMAT_ORDER_NHW: hailo_format_order_t = 5;
pub const HAILO_FORMAT_ORDER_NC: hailo_format_order_t = 6;
pub const HAILO_FORMAT_ORDER_BAYER_RGB: hailo_format_order_t = 7;
pub const HAILO_FORMAT_ORDER_12_BIT_BAYER_RGB: hailo_format_order_t = 8;
pub const HAILO_FORMAT_ORDER_HAILO_NMS: hailo_format_order_t = 9;
pub const HAILO_FORMAT_ORDER_RGB888: hailo_format_order_t = 10;
pub const HAILO_FORMAT_ORDER_NCHW: hailo_format_order_t = 11;
pub const HAILO_FORMAT_ORDER_YUY2: hailo_format_order_t = 12;
pub const HAILO_FORMAT_ORDER_NV12: hailo_format_order_t = 13;
pub const HAILO_FORMAT_ORDER_NV21: hailo_format_order_t = 14;
pub const HAILO_FORMAT_ORDER_HAILO_YYUV: hailo_format_order_t = 15;
pub const HAILO_FORMAT_ORDER_HAILO_YYVU: hailo_format_order_t = 16;
pub const HAILO_FORMAT_ORDER_RGB4: hailo_format_order_t = 17;
pub const HAILO_FORMAT_ORDER_I420: hailo_format_order_t = 18;
pub const HAILO_FORMAT_ORDER_HAILO_YYYYUV: hailo_format_order_t = 19;
pub const HAILO_FORMAT_ORDER_HAILO_NMS_WITH_BYTE_MASK: hailo_format_order_t = 20;
pub const HAILO_FORMAT_ORDER_HAILO_NMS_ON_CHIP: hailo_format_order_t = 21;
pub const HAILO_FORMAT_ORDER_HAILO_NMS_BY_CLASS: hailo_format_order_t = 22;
pub const HAILO_FORMAT_ORDER_HAILO_NMS_BY_SCORE: hailo_format_order_t = 23;

pub type hailo_format_flags_t = c_int;
pub const HAILO_FORMAT_FLAGS_NONE: hailo_format_flags_t = 0;
pub const HAILO_FORMAT_FLAGS_QUANTIZED: hailo_format_flags_t = 1;
pub const HAILO_FORMAT_FLAGS_TRANSPOSED: hailo_format_flags_t = 2;

pub type hailo_stream_transform_mode_t = c_int;
pub const HAILO_STREAM_NO_TRANSFORM: hailo_stream_transform_mode_t = 0;
pub const HAILO_STREAM_TRANSFORM_COPY: hailo_stream_transform_mode_t = 1;

pub type hailo_stream_direction_t = c_int;
pub const HAILO_H2D_STREAM: hailo_stream_direction_t = 0;
pub const HAILO_D2H_STREAM: hailo_stream_direction_t = 1;

pub type hailo_stream_flags_t = c_int;
pub const HAILO_STREAM_FLAGS_NONE: hailo_stream_flags_t = 0;
pub const HAILO_STREAM_FLAGS_ASYNC: hailo_stream_flags_t = 1;

pub type hailo_dma_buffer_direction_t = c_int;
pub const HAILO_DMA_BUFFER_DIRECTION_H2D: hailo_dma_buffer_direction_t = 0;
pub const HAILO_DMA_BUFFER_DIRECTION_D2H: hailo_dma_buffer_direction_t = 1;
pub const HAILO_DMA_BUFFER_DIRECTION_BOTH: hailo_dma_buffer_direction_t = 2;

pub type hailo_buffer_flags_t = c_int;
pub const HAILO_BUFFER_FLAGS_NONE: hailo_buffer_flags_t = 0;
pub const HAILO_BUFFER_FLAGS_DMA: hailo_buffer_flags_t = 1;
pub const HAILO_BUFFER_FLAGS_CONTINUOUS: hailo_buffer_flags_t = 2;
pub const HAILO_BUFFER_FLAGS_SHARED_MEMORY: hailo_buffer_flags_t = 3;

pub type hailo_stream_interface_t = c_int;
pub const HAILO_STREAM_INTERFACE_PCIE: hailo_stream_interface_t = 0;
pub const HAILO_STREAM_INTERFACE_ETH: hailo_stream_interface_t = 1;
pub const HAILO_STREAM_INTERFACE_MIPI: hailo_stream_interface_t = 2;
pub const HAILO_STREAM_INTERFACE_INTEGRATED: hailo_stream_interface_t = 3;

pub type hailo_vstream_stats_flags_t = c_int;
pub const HAILO_VSTREAM_STATS_NONE: hailo_vstream_stats_flags_t = 0;
pub const HAILO_VSTREAM_STATS_MEASURE_FPS: hailo_vstream_stats_flags_t = 1;
pub const HAILO_VSTREAM_STATS_MEASURE_LATENCY: hailo_vstream_stats_flags_t = 2;

pub type hailo_pipeline_elem_stats_flags_t = c_int;
pub const HAILO_PIPELINE_ELEM_STATS_NONE: hailo_pipeline_elem_stats_flags_t = 0;
pub const HAILO_PIPELINE_ELEM_STATS_MEASURE_FPS: hailo_pipeline_elem_stats_flags_t = 1;
pub const HAILO_PIPELINE_ELEM_STATS_MEASURE_LATENCY: hailo_pipeline_elem_stats_flags_t = 2;
pub const HAILO_PIPELINE_ELEM_STATS_MEASURE_QUEUE_SIZE: hailo_pipeline_elem_stats_flags_t = 4;

pub type hailo_power_mode_t = c_int;
pub const HAILO_POWER_MODE_PERFORMANCE: hailo_power_mode_t = 0;
pub const HAILO_POWER_MODE_ULTRA_PERFORMANCE: hailo_power_mode_t = 1;

pub type hailo_latency_measurement_flags_t = c_int;
pub const HAILO_LATENCY_NONE: hailo_latency_measurement_flags_t = 0;
pub const HAILO_LATENCY_MEASURE: hailo_latency_measurement_flags_t = 1;
pub const HAILO_LATENCY_CLEAR_AFTER_GET: hailo_latency_measurement_flags_t = 2;

pub type hailo_reset_device_mode_t = c_int;
pub const HAILO_RESET_DEVICE_MODE_CHIP: hailo_reset_device_mode_t = 0;
pub const HAILO_RESET_DEVICE_MODE_NN_CORE: hailo_reset_device_mode_t = 1;
pub const HAILO_RESET_DEVICE_MODE_SOFT: hailo_reset_device_mode_t = 2;
pub const HAILO_RESET_DEVICE_MODE_FORCED_SOFT: hailo_reset_device_mode_t = 3;
pub const HAILO_RESET_DEVICE_MODE_REBOOT: hailo_reset_device_mode_t = 4;

pub type hailo_watchdog_mode_t = c_int;
pub const HAILO_WATCHDOG_MODE_HW_SW: hailo_watchdog_mode_t = 0;
pub const HAILO_WATCHDOG_MODE_HW_ONLY: hailo_watchdog_mode_t = 1;

pub type hailo_notification_id_t = c_int;
pub const HAILO_NOTIFICATION_ID_ETHERNET_RX_ERROR: hailo_notification_id_t = 0;
pub const HAILO_NOTIFICATION_ID_HEALTH_MONITOR_TEMPERATURE_ALARM: hailo_notification_id_t = 1;
pub const HAILO_NOTIFICATION_ID_HEALTH_MONITOR_DATAFLOW_SHUTDOWN: hailo_notification_id_t = 2;
pub const HAILO_NOTIFICATION_ID_HEALTH_MONITOR_OVERCURRENT_ALARM: hailo_notification_id_t = 3;
pub const HAILO_NOTIFICATION_ID_LCU_ECC_CORRECTABLE_ERROR: hailo_notification_id_t = 4;
pub const HAILO_NOTIFICATION_ID_LCU_ECC_UNCORRECTABLE_ERROR: hailo_notification_id_t = 5;
pub const HAILO_NOTIFICATION_ID_CPU_ECC_ERROR: hailo_notification_id_t = 6;
pub const HAILO_NOTIFICATION_ID_CPU_ECC_FATAL: hailo_notification_id_t = 7;
pub const HAILO_NOTIFICATION_ID_DEBUG: hailo_notification_id_t = 8;
pub const HAILO_NOTIFICATION_ID_CONTEXT_SWITCH_BREAKPOINT_REACHED: hailo_notification_id_t = 9;
pub const HAILO_NOTIFICATION_ID_HEALTH_MONITOR_CLOCK_CHANGED_EVENT: hailo_notification_id_t = 10;
pub const HAILO_NOTIFICATION_ID_HW_INFER_MANAGER_INFER_DONE: hailo_notification_id_t = 11;
pub const HAILO_NOTIFICATION_ID_CONTEXT_SWITCH_RUN_TIME_ERROR_EVENT: hailo_notification_id_t = 12;
pub const HAILO_NOTIFICATION_ID_NN_CORE_CRC_ERROR_EVENT: hailo_notification_id_t = 13;
pub const HAILO_NOTIFICATION_ID_THROTTLING_STATE_CHANGE_EVENT: hailo_notification_id_t = 14;
pub const HAILO_NOTIFICATION_ID_COUNT: hailo_notification_id_t = 15;

pub type hailo_temperature_protection_temperature_zone_t = c_int;
pub const HAILO_TEMPERATURE_PROTECTION_TEMPERATURE_ZONE__GREEN:
    hailo_temperature_protection_temperature_zone_t = 0;
pub const HAILO_TEMPERATURE_PROTECTION_TEMPERATURE_ZONE__ORANGE:
    hailo_temperature_protection_temperature_zone_t = 1;
pub const HAILO_TEMPERATURE_PROTECTION_TEMPERATURE_ZONE__RED:
    hailo_temperature_protection_temperature_zone_t = 2;

pub type hailo_overcurrent_protection_overcurrent_zone_t = c_int;
pub const HAILO_OVERCURRENT_PROTECTION_OVERCURRENT_ZONE__GREEN:
    hailo_overcurrent_protection_overcurrent_zone_t = 0;
pub const HAILO_OVERCURRENT_PROTECTION_OVERCURRENT_ZONE__RED:
    hailo_overcurrent_protection_overcurrent_zone_t = 1;

pub type hailo_nms_burst_type_t = c_int;
pub const HAILO_BURST_TYPE_H8_BBOX: hailo_nms_burst_type_t = 0;
pub const HAILO_BURST_TYPE_H15_BBOX: hailo_nms_burst_type_t = 1;
pub const HAILO_BURST_TYPE_H8_PER_CLASS: hailo_nms_burst_type_t = 2;
pub const HAILO_BURST_TYPE_H15_PER_CLASS: hailo_nms_burst_type_t = 3;
pub const HAILO_BURST_TYPE_H15_PER_FRAME: hailo_nms_burst_type_t = 4;
pub const HAILO_BURST_TYPE_COUNT: hailo_nms_burst_type_t = 5;

pub type hailo_pix_buffer_memory_type_t = c_int;
pub const HAILO_PIX_BUFFER_MEMORY_TYPE_USERPTR: hailo_pix_buffer_memory_type_t = 0;
pub const HAILO_PIX_BUFFER_MEMORY_TYPE_DMABUF: hailo_pix_buffer_memory_type_t = 1;

pub type hailo_sensor_types_t = c_int;
pub const HAILO_SENSOR_TYPES_GENERIC: hailo_sensor_types_t = 0;
pub const HAILO_SENSOR_TYPES_ONSEMI_AR0220AT: hailo_sensor_types_t = 1;
pub const HAILO_SENSOR_TYPES_RASPICAM: hailo_sensor_types_t = 2;
pub const HAILO_SENSOR_TYPES_ONSEMI_AS0149AT: hailo_sensor_types_t = 3;
pub const HAILO_SENSOR_TYPES_HAILO8_ISP: hailo_sensor_types_t = 4;

pub type hailo_fw_logger_interface_t = c_int;
pub const HAILO_FW_LOGGER_INTERFACE_PCIE: hailo_fw_logger_interface_t = 0;
pub const HAILO_FW_LOGGER_INTERFACE_UART: hailo_fw_logger_interface_t = 1;

pub type hailo_fw_logger_level_t = c_int;
pub const HAILO_FW_LOGGER_LEVEL_TRACE: hailo_fw_logger_level_t = 0;
pub const HAILO_FW_LOGGER_LEVEL_DEBUG: hailo_fw_logger_level_t = 1;
pub const HAILO_FW_LOGGER_LEVEL_INFO: hailo_fw_logger_level_t = 2;
pub const HAILO_FW_LOGGER_LEVEL_WARN: hailo_fw_logger_level_t = 3;
pub const HAILO_FW_LOGGER_LEVEL_ERROR: hailo_fw_logger_level_t = 4;
pub const HAILO_FW_LOGGER_LEVEL_FATAL: hailo_fw_logger_level_t = 5;

pub type hailo_hw_throttling_state_t = c_int;
pub const HAILO_THROTTLING_STATE_NONE: hailo_hw_throttling_state_t = 0;
pub const HAILO_THROTTLING_STATE_0_LIGHT: hailo_hw_throttling_state_t = 1;
pub const HAILO_THROTTLING_STATE_1_MEDIUM: hailo_hw_throttling_state_t = 2;
pub const HAILO_THROTTLING_STATE_2_HEAVY: hailo_hw_throttling_state_t = 3;
pub const HAILO_THROTTLING_STATE_3_SEVERE: hailo_hw_throttling_state_t = 4;
pub const HAILO_THROTTLING_STATE_4_STREAMS_OFF: hailo_hw_throttling_state_t = 5;
pub const HAILO_THROTTLING_STATE_OVERHEAT: hailo_hw_throttling_state_t = 6;
pub const HAILO_THROTTLING_STATE_COUNT: hailo_hw_throttling_state_t = 7;

// ---------------------------------------------------------------------------
// Structures
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
pub struct hailo_vdevice_params_t {
    pub device_count: u32,
    /// Pointer to an array of `device_count` device IDs, or null for auto.
    pub device_ids: *mut hailo_device_id_t,
    pub scheduling_algorithm: hailo_scheduling_algorithm_t,
    /// Null-terminated group ID string, or null for the default unique group.
    pub group_id: *const c_char,
    pub multi_process_service: bool,
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
pub struct hailo_i2c_slave_config_t {
    pub endianness: hailo_endianness_t,
    pub slave_address: u16,
    pub register_address_size: u8,
    pub bus_index: u8,
    pub should_hold_bus: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_fw_user_config_information_t {
    pub version: u32,
    pub entry_count: u32,
    pub total_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_format_t {
    pub type_: hailo_format_type_t,
    pub order: hailo_format_order_t,
    pub flags: hailo_format_flags_t,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_quant_info_t {
    pub qp_zp: float32_t,
    pub qp_scale: float32_t,
    pub limvals_min: float32_t,
    pub limvals_max: float32_t,
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
/// beyond index 0 via raw pointer arithmetic.
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

// ---------------------------------------------------------------------------
// Callback type aliases
// ---------------------------------------------------------------------------

pub type hailo_stream_write_async_callback_t = unsafe extern "C" fn(
    info: *const hailo_stream_write_async_completion_info_t,
);

pub type hailo_stream_read_async_callback_t = unsafe extern "C" fn(
    info: *const hailo_stream_read_async_completion_info_t,
);

pub type hailo_notification_callback = unsafe extern "C" fn(
    device: hailo_device,
    notification: *const hailo_notification_t,
    opaque: *mut c_void,
);

// ---------------------------------------------------------------------------
// Notification structures (needed before hailo_notification_t)
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
    pub health_monitor_cpu_ecc_notification:
        hailo_health_monitor_cpu_ecc_notification_message_t,
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
// Stream and VStream info structures
// ---------------------------------------------------------------------------

/// Unnamed inner struct for the shape branch of `hailo_stream_info_t`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_stream_info_shape_pair_t {
    pub shape: hailo_3d_image_shape_t,
    pub hw_shape: hailo_3d_image_shape_t,
}

/// Anonymous union inside `hailo_stream_info_t`.
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

/// Anonymous union inside `hailo_vstream_info_t`.
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
// Power, temperature, and health structures
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
    pub stream_params_by_name:
        [hailo_stream_parameters_by_name_t; HAILO_MAX_STREAMS_COUNT],
    pub network_params_by_name_count: usize,
    pub network_params_by_name:
        [hailo_network_parameters_by_name_t; HAILO_MAX_NETWORKS_IN_NETWORK_GROUP],
}

#[repr(C)]
pub struct hailo_configure_params_t {
    pub network_group_params_count: usize,
    pub network_group_params:
        [hailo_configure_network_group_params_t; HAILO_MAX_NETWORK_GROUPS],
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hailo_buffer_parameters_t {
    pub flags: hailo_buffer_flags_t,
}

// ---------------------------------------------------------------------------
// extern "C" — the actual library functions
// ---------------------------------------------------------------------------

#[link(name = "hailort")]
unsafe extern "C" {
    // --- Library version and status ----------------------------------------

    pub fn hailo_get_library_version(version: *mut hailo_version_t) -> hailo_status;
    pub fn hailo_get_status_message(status: hailo_status) -> *const c_char;

    // --- Device discovery and creation -------------------------------------

    pub fn hailo_scan_devices(
        params: *mut hailo_scan_devices_params_t,
        device_ids: *mut hailo_device_id_t,
        device_ids_length: *mut usize,
    ) -> hailo_status;

    pub fn hailo_create_device_by_id(
        device_id: *const hailo_device_id_t,
        device: *mut hailo_device,
    ) -> hailo_status;

    pub fn hailo_scan_pcie_devices(
        pcie_device_infos: *mut hailo_pcie_device_info_t,
        pcie_device_infos_length: usize,
        number_of_devices: *mut usize,
    ) -> hailo_status;

    pub fn hailo_parse_pcie_device_info(
        device_info_str: *const c_char,
        device_info: *mut hailo_pcie_device_info_t,
    ) -> hailo_status;

    pub fn hailo_create_pcie_device(
        device_info: *mut hailo_pcie_device_info_t,
        device: *mut hailo_device,
    ) -> hailo_status;

    pub fn hailo_release_device(device: hailo_device) -> hailo_status;

    pub fn hailo_device_get_type_by_device_id(
        device_id: *const hailo_device_id_t,
        device_type: *mut hailo_device_type_t,
    ) -> hailo_status;

    // --- Device identification ---------------------------------------------

    pub fn hailo_identify(
        device: hailo_device,
        device_identity: *mut hailo_device_identity_t,
    ) -> hailo_status;

    pub fn hailo_core_identify(
        device: hailo_device,
        core_information: *mut hailo_core_information_t,
    ) -> hailo_status;

    pub fn hailo_get_extended_device_information(
        device: hailo_device,
        extended_device_information: *mut hailo_extended_device_information_t,
    ) -> hailo_status;

    pub fn hailo_get_device_id(
        device: hailo_device,
        id: *mut hailo_device_id_t,
    ) -> hailo_status;

    pub fn hailo_get_driver_version(
        device: hailo_device,
        version: *mut hailo_version_t,
    ) -> hailo_status;

    // --- Firmware and system control ---------------------------------------

    pub fn hailo_set_fw_logger(
        device: hailo_device,
        level: hailo_fw_logger_level_t,
        interface_mask: u32,
    ) -> hailo_status;

    pub fn hailo_set_throttling_state(
        device: hailo_device,
        should_activate: bool,
    ) -> hailo_status;

    pub fn hailo_get_throttling_state(
        device: hailo_device,
        is_active: *mut bool,
    ) -> hailo_status;

    pub fn hailo_wd_enable(
        device: hailo_device,
        cpu_id: hailo_cpu_id_t,
    ) -> hailo_status;

    pub fn hailo_wd_disable(
        device: hailo_device,
        cpu_id: hailo_cpu_id_t,
    ) -> hailo_status;

    pub fn hailo_wd_config(
        device: hailo_device,
        cpu_id: hailo_cpu_id_t,
        wd_cycles: u32,
        wd_mode: hailo_watchdog_mode_t,
    ) -> hailo_status;

    pub fn hailo_get_previous_system_state(
        device: hailo_device,
        cpu_id: hailo_cpu_id_t,
        previous_system_state: *mut u32,
    ) -> hailo_status;

    pub fn hailo_set_pause_frames(
        device: hailo_device,
        rx_pause_frames_enable: bool,
    ) -> hailo_status;

    // --- Temperature and reset ---------------------------------------------

    pub fn hailo_get_chip_temperature(
        device: hailo_device,
        temp_info: *mut hailo_chip_temperature_info_t,
    ) -> hailo_status;

    pub fn hailo_reset_device(
        device: hailo_device,
        mode: hailo_reset_device_mode_t,
    ) -> hailo_status;

    // --- Firmware update ---------------------------------------------------

    pub fn hailo_update_firmware(
        device: hailo_device,
        firmware_buffer: *mut c_void,
        firmware_buffer_size: u32,
    ) -> hailo_status;

    pub fn hailo_update_second_stage(
        device: hailo_device,
        second_stage_buffer: *mut c_void,
        second_stage_buffer_size: u32,
    ) -> hailo_status;

    // --- Notification management -------------------------------------------

    pub fn hailo_set_notification_callback(
        device: hailo_device,
        callback: hailo_notification_callback,
        notification_id: hailo_notification_id_t,
        opaque: *mut c_void,
    ) -> hailo_status;

    pub fn hailo_remove_notification_callback(
        device: hailo_device,
        notification_id: hailo_notification_id_t,
    ) -> hailo_status;

    // --- Sensor management -------------------------------------------------

    pub fn hailo_reset_sensor(device: hailo_device, section_index: u8) -> hailo_status;

    pub fn hailo_set_sensor_i2c_bus_index(
        device: hailo_device,
        sensor_type: hailo_sensor_types_t,
        bus_index: u8,
    ) -> hailo_status;

    pub fn hailo_load_and_start_sensor(
        device: hailo_device,
        section_index: u8,
    ) -> hailo_status;

    pub fn hailo_dump_sensor_config(
        device: hailo_device,
        section_index: u8,
        config_file_path: *const c_char,
    ) -> hailo_status;

    pub fn hailo_store_sensor_config(
        device: hailo_device,
        section_index: u32,
        sensor_type: hailo_sensor_types_t,
        reset_config_size: u32,
        config_height: u16,
        config_width: u16,
        config_fps: u16,
        config_file_path: *const c_char,
        config_name: *const c_char,
    ) -> hailo_status;

    pub fn hailo_store_isp_config(
        device: hailo_device,
        reset_config_size: u32,
        config_height: u16,
        config_width: u16,
        config_fps: u16,
        isp_static_config_file_path: *const c_char,
        isp_runtime_config_file_path: *const c_char,
        config_name: *const c_char,
    ) -> hailo_status;

    // --- I2C ---------------------------------------------------------------

    pub fn hailo_i2c_read(
        device: hailo_device,
        slave_config: *const hailo_i2c_slave_config_t,
        register_address: u32,
        data: *mut u8,
        length: u32,
    ) -> hailo_status;

    pub fn hailo_i2c_write(
        device: hailo_device,
        slave_config: *const hailo_i2c_slave_config_t,
        register_address: u32,
        data: *const u8,
        length: u32,
    ) -> hailo_status;

    // --- Diagnostics -------------------------------------------------------

    pub fn hailo_test_chip_memories(device: hailo_device) -> hailo_status;

    // --- Power measurement -------------------------------------------------

    pub fn hailo_power_measurement(
        device: hailo_device,
        dvm: hailo_dvm_options_t,
        measurement_type: hailo_power_measurement_types_t,
        measurement: *mut float32_t,
    ) -> hailo_status;

    pub fn hailo_start_power_measurement(
        device: hailo_device,
        averaging_factor: hailo_averaging_factor_t,
        sampling_period: hailo_sampling_period_t,
    ) -> hailo_status;

    pub fn hailo_set_power_measurement(
        device: hailo_device,
        buffer_index: hailo_measurement_buffer_index_t,
        dvm: hailo_dvm_options_t,
        measurement_type: hailo_power_measurement_types_t,
    ) -> hailo_status;

    pub fn hailo_get_power_measurement(
        device: hailo_device,
        buffer_index: hailo_measurement_buffer_index_t,
        should_clear: bool,
        measurement_data: *mut hailo_power_measurement_data_t,
    ) -> hailo_status;

    pub fn hailo_stop_power_measurement(device: hailo_device) -> hailo_status;

    // --- Health monitoring -------------------------------------------------

    pub fn hailo_get_health_information(
        device: hailo_device,
        health_info: *mut hailo_health_info_t,
    ) -> hailo_status;

    pub fn hailo_get_performance_stats(
        device: hailo_device,
        perf_stats: *mut hailo_performance_stats_t,
    ) -> hailo_status;

    pub fn hailo_get_health_stats(
        device: hailo_device,
        health_stats: *mut hailo_health_stats_t,
    ) -> hailo_status;

    // --- VDevice -----------------------------------------------------------

    pub fn hailo_init_vdevice_params(params: *mut hailo_vdevice_params_t) -> hailo_status;

    pub fn hailo_create_vdevice(
        params: *mut hailo_vdevice_params_t,
        vdevice: *mut hailo_vdevice,
    ) -> hailo_status;

    pub fn hailo_configure_vdevice(
        vdevice: hailo_vdevice,
        hef: hailo_hef,
        params: *mut hailo_configure_params_t,
        network_groups: *mut hailo_configured_network_group,
        number_of_network_groups: *mut usize,
    ) -> hailo_status;

    pub fn hailo_get_physical_devices(
        vdevice: hailo_vdevice,
        devices: *mut hailo_device,
        number_of_devices: *mut usize,
    ) -> hailo_status;

    pub fn hailo_vdevice_get_physical_devices_ids(
        vdevice: hailo_vdevice,
        devices_ids: *mut hailo_device_id_t,
        number_of_devices: *mut usize,
    ) -> hailo_status;

    pub fn hailo_release_vdevice(vdevice: hailo_vdevice) -> hailo_status;

    // --- HEF ---------------------------------------------------------------

    pub fn hailo_create_hef_file(hef: *mut hailo_hef, file_name: *const c_char) -> hailo_status;

    pub fn hailo_create_hef_buffer(
        hef: *mut hailo_hef,
        buffer: *const c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_release_hef(hef: hailo_hef) -> hailo_status;

    pub fn hailo_hef_get_stream_infos(
        hef: hailo_hef,
        name: *const c_char,
        stream_infos: *mut hailo_stream_info_t,
        number_of_streams: *mut usize,
    ) -> hailo_status;

    pub fn hailo_hef_get_vstream_infos(
        hef: hailo_hef,
        name: *const c_char,
        vstream_infos: *mut hailo_vstream_info_t,
        number_of_vstreams: *mut usize,
    ) -> hailo_status;

    pub fn hailo_hef_get_network_group_infos(
        hef: hailo_hef,
        network_group_infos: *mut hailo_network_group_info_t,
        number_of_network_groups: *mut usize,
    ) -> hailo_status;

    pub fn hailo_hef_get_network_infos(
        hef: hailo_hef,
        network_group_name: *const c_char,
        network_infos: *mut hailo_network_info_t,
        number_of_networks: *mut usize,
    ) -> hailo_status;

    // --- Configure device --------------------------------------------------

    pub fn hailo_init_configure_params_by_device(
        device: hailo_device,
        hef: hailo_hef,
        params: *mut hailo_configure_params_t,
    ) -> hailo_status;

    pub fn hailo_init_configure_params_by_vdevice(
        vdevice: hailo_vdevice,
        hef: hailo_hef,
        params: *mut hailo_configure_params_t,
    ) -> hailo_status;

    pub fn hailo_configure_device(
        device: hailo_device,
        hef: hailo_hef,
        params: *mut hailo_configure_params_t,
        network_groups: *mut hailo_configured_network_group,
        number_of_network_groups: *mut usize,
    ) -> hailo_status;

    // --- Network group activation ------------------------------------------

    pub fn hailo_activate_network_group(
        network_group: hailo_configured_network_group,
        params: *mut hailo_activate_network_group_params_t,
        activated_network_group: *mut hailo_activated_network_group,
    ) -> hailo_status;

    pub fn hailo_deactivate_network_group(
        network_group: hailo_activated_network_group,
    ) -> hailo_status;

    pub fn hailo_release_network_group(
        network_group: hailo_configured_network_group,
    ) -> hailo_status;

    pub fn hailo_get_network_group_info(
        network_group: hailo_configured_network_group,
        info: *mut hailo_network_group_info_t,
    ) -> hailo_status;

    // --- Synchronous stream I/O --------------------------------------------

    pub fn hailo_get_input_streams_by_network(
        network_group: hailo_activated_network_group,
        network_name: *const c_char,
        inputs: *mut hailo_input_stream,
        number_of_inputs: *mut usize,
    ) -> hailo_status;

    pub fn hailo_get_output_streams_by_network(
        network_group: hailo_activated_network_group,
        network_name: *const c_char,
        outputs: *mut hailo_output_stream,
        number_of_outputs: *mut usize,
    ) -> hailo_status;

    pub fn hailo_input_stream_write(
        stream: hailo_input_stream,
        buffer: *const c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_output_stream_read(
        stream: hailo_output_stream,
        buffer: *mut c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_stream_get_info(
        stream: hailo_input_stream,
        info: *mut hailo_stream_info_t,
    ) -> hailo_status;

    pub fn hailo_output_stream_get_info(
        stream: hailo_output_stream,
        info: *mut hailo_stream_info_t,
    ) -> hailo_status;

    // --- Asynchronous stream I/O -------------------------------------------

    pub fn hailo_input_stream_write_async(
        stream: hailo_input_stream,
        buffer: *const c_void,
        size: usize,
        callback: hailo_stream_write_async_callback_t,
        opaque: *mut c_void,
    ) -> hailo_status;

    pub fn hailo_output_stream_read_async(
        stream: hailo_output_stream,
        buffer: *mut c_void,
        size: usize,
        callback: hailo_stream_read_async_callback_t,
        opaque: *mut c_void,
    ) -> hailo_status;

    // --- Quantisation info -------------------------------------------------

    pub fn hailo_get_output_stream_quant_infos(
        stream: hailo_output_stream,
        quant_infos: *mut hailo_quant_info_t,
        number_of_quant_infos: *mut usize,
    ) -> hailo_status;

    pub fn hailo_get_output_vstream_quant_infos(
        vstream: hailo_output_vstream,
        quant_infos: *mut hailo_quant_info_t,
        number_of_quant_infos: *mut usize,
    ) -> hailo_status;

    // --- Transform contexts ------------------------------------------------

    pub fn hailo_create_input_transform_context(
        stream: hailo_input_stream,
        transform_params: *const hailo_transform_params_t,
        context: *mut hailo_input_transform_context,
    ) -> hailo_status;

    pub fn hailo_create_output_transform_context(
        stream: hailo_output_stream,
        transform_params: *const hailo_transform_params_t,
        context: *mut hailo_output_transform_context,
    ) -> hailo_status;

    pub fn hailo_input_transform_context_write(
        context: hailo_input_transform_context,
        buffer: *const c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_output_transform_context_read(
        context: hailo_output_transform_context,
        buffer: *mut c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_release_input_transform_context(
        context: hailo_input_transform_context,
    ) -> hailo_status;

    pub fn hailo_release_output_transform_context(
        context: hailo_output_transform_context,
    ) -> hailo_status;

    // --- Output demuxer ----------------------------------------------------

    pub fn hailo_create_output_demuxer(
        stream: hailo_output_stream,
        demux_params: *const hailo_demux_params_t,
        demuxer: *mut hailo_output_demuxer,
    ) -> hailo_status;

    pub fn hailo_output_demuxer_read(
        demuxer: hailo_output_demuxer,
        buffer: *mut c_void,
        size: usize,
        actual_size: *mut usize,
    ) -> hailo_status;

    pub fn hailo_release_output_demuxer(demuxer: hailo_output_demuxer) -> hailo_status;

    // --- Virtual streams (vstreams) ----------------------------------------

    pub fn hailo_create_input_vstreams(
        network_group: hailo_configured_network_group,
        inputs_params: *const hailo_input_vstream_params_by_name_t,
        inputs_count: usize,
        input_vstreams: *mut hailo_input_vstream,
    ) -> hailo_status;

    pub fn hailo_create_output_vstreams(
        network_group: hailo_configured_network_group,
        outputs_params: *const hailo_output_vstream_params_by_name_t,
        outputs_count: usize,
        output_vstreams: *mut hailo_output_vstream,
    ) -> hailo_status;

    pub fn hailo_release_input_vstream(vstream: hailo_input_vstream) -> hailo_status;
    pub fn hailo_release_output_vstream(vstream: hailo_output_vstream) -> hailo_status;

    pub fn hailo_input_vstream_write(
        vstream: hailo_input_vstream,
        buffer: *const c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_output_vstream_read(
        vstream: hailo_output_vstream,
        buffer: *mut c_void,
        size: usize,
    ) -> hailo_status;

    pub fn hailo_input_vstream_get_info(
        vstream: hailo_input_vstream,
        info: *mut hailo_vstream_info_t,
    ) -> hailo_status;

    pub fn hailo_output_vstream_get_info(
        vstream: hailo_output_vstream,
        info: *mut hailo_vstream_info_t,
    ) -> hailo_status;

    pub fn hailo_input_vstream_flush(vstream: hailo_input_vstream) -> hailo_status;

    pub fn hailo_input_vstream_clear(vstream: hailo_input_vstream) -> hailo_status;

    // --- Default parameter initialisation ----------------------------------

    pub fn hailo_get_default_vstream_params(
        vstream_info: *const hailo_vstream_info_t,
        user_buffer_format: hailo_format_t,
        direction: hailo_stream_direction_t,
        params: *mut hailo_vstream_params_t,
    ) -> hailo_status;

    // --- Latency measurement -----------------------------------------------

    pub fn hailo_input_vstream_get_latency_measurement(
        vstream: hailo_input_vstream,
        result: *mut hailo_latency_measurement_result_t,
    ) -> hailo_status;
}
