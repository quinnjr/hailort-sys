//! C enum type aliases and their named variant constants.
//!
//! All C enums are represented as `type Foo = c_int` paired with `pub const`
//! values.  This is safer than `#[repr(C)] enum` because an unknown
//! discriminant received from the library never causes undefined behaviour.

use std::os::raw::c_int;

// --- Power and measurement --------------------------------------------------

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

pub type hailo_power_mode_t = c_int;
pub const HAILO_POWER_MODE_PERFORMANCE: hailo_power_mode_t = 0;
pub const HAILO_POWER_MODE_ULTRA_PERFORMANCE: hailo_power_mode_t = 1;

// --- Device identity --------------------------------------------------------

pub type hailo_device_type_t = c_int;
pub const HAILO_DEVICE_TYPE_PCIE: hailo_device_type_t = 0;
pub const HAILO_DEVICE_TYPE_ETH: hailo_device_type_t = 1;
pub const HAILO_DEVICE_TYPE_INTEGRATED: hailo_device_type_t = 2;

pub type hailo_device_architecture_t = c_int;
pub const HAILO_ARCH_HAILO8_A0: hailo_device_architecture_t = 0;
pub const HAILO_ARCH_HAILO8: hailo_device_architecture_t = 1;
pub const HAILO_ARCH_HAILO8L: hailo_device_architecture_t = 2;
pub const HAILO_ARCH_HAILO15H: hailo_device_architecture_t = 3;
pub const HAILO_ARCH_HAILO15L: hailo_device_architecture_t = 4;
pub const HAILO_ARCH_HAILO15M: hailo_device_architecture_t = 5;
pub const HAILO_ARCH_HAILO10H: hailo_device_architecture_t = 6;
pub const HAILO_ARCH_HAILO12L: hailo_device_architecture_t = 7;

pub type hailo_device_boot_source_t = c_int;
pub const HAILO_DEVICE_BOOT_SOURCE_INVALID: hailo_device_boot_source_t = 0;
pub const HAILO_DEVICE_BOOT_SOURCE_PCIE: hailo_device_boot_source_t = 1;
pub const HAILO_DEVICE_BOOT_SOURCE_FLASH: hailo_device_boot_source_t = 2;

pub type hailo_cpu_id_t = c_int;
pub const HAILO_CPU_ID_0: hailo_cpu_id_t = 0;
pub const HAILO_CPU_ID_1: hailo_cpu_id_t = 1;

pub type hailo_scheduling_algorithm_t = c_int;
pub const HAILO_SCHEDULING_ALGORITHM_NONE: hailo_scheduling_algorithm_t = 0;
pub const HAILO_SCHEDULING_ALGORITHM_ROUND_ROBIN: hailo_scheduling_algorithm_t = 1;

pub type hailo_reset_device_mode_t = c_int;
pub const HAILO_RESET_DEVICE_MODE_CHIP: hailo_reset_device_mode_t = 0;
pub const HAILO_RESET_DEVICE_MODE_NN_CORE: hailo_reset_device_mode_t = 1;
pub const HAILO_RESET_DEVICE_MODE_SOFT: hailo_reset_device_mode_t = 2;
pub const HAILO_RESET_DEVICE_MODE_FORCED_SOFT: hailo_reset_device_mode_t = 3;
pub const HAILO_RESET_DEVICE_MODE_REBOOT: hailo_reset_device_mode_t = 4;

pub type hailo_watchdog_mode_t = c_int;
pub const HAILO_WATCHDOG_MODE_HW_SW: hailo_watchdog_mode_t = 0;
pub const HAILO_WATCHDOG_MODE_HW_ONLY: hailo_watchdog_mode_t = 1;

// --- Data format ------------------------------------------------------------

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

// --- Streams ----------------------------------------------------------------

pub type hailo_stream_transform_mode_t = c_int;
pub const HAILO_STREAM_NO_TRANSFORM: hailo_stream_transform_mode_t = 0;
pub const HAILO_STREAM_TRANSFORM_COPY: hailo_stream_transform_mode_t = 1;

pub type hailo_stream_direction_t = c_int;
pub const HAILO_H2D_STREAM: hailo_stream_direction_t = 0;
pub const HAILO_D2H_STREAM: hailo_stream_direction_t = 1;

pub type hailo_stream_flags_t = c_int;
pub const HAILO_STREAM_FLAGS_NONE: hailo_stream_flags_t = 0;
pub const HAILO_STREAM_FLAGS_ASYNC: hailo_stream_flags_t = 1;

pub type hailo_stream_interface_t = c_int;
pub const HAILO_STREAM_INTERFACE_PCIE: hailo_stream_interface_t = 0;
pub const HAILO_STREAM_INTERFACE_ETH: hailo_stream_interface_t = 1;
pub const HAILO_STREAM_INTERFACE_MIPI: hailo_stream_interface_t = 2;
pub const HAILO_STREAM_INTERFACE_INTEGRATED: hailo_stream_interface_t = 3;

pub type hailo_latency_measurement_flags_t = c_int;
pub const HAILO_LATENCY_NONE: hailo_latency_measurement_flags_t = 0;
pub const HAILO_LATENCY_MEASURE: hailo_latency_measurement_flags_t = 1;
pub const HAILO_LATENCY_CLEAR_AFTER_GET: hailo_latency_measurement_flags_t = 2;

// --- Virtual streams --------------------------------------------------------

pub type hailo_vstream_stats_flags_t = c_int;
pub const HAILO_VSTREAM_STATS_NONE: hailo_vstream_stats_flags_t = 0;
pub const HAILO_VSTREAM_STATS_MEASURE_FPS: hailo_vstream_stats_flags_t = 1;
pub const HAILO_VSTREAM_STATS_MEASURE_LATENCY: hailo_vstream_stats_flags_t = 2;

pub type hailo_pipeline_elem_stats_flags_t = c_int;
pub const HAILO_PIPELINE_ELEM_STATS_NONE: hailo_pipeline_elem_stats_flags_t = 0;
pub const HAILO_PIPELINE_ELEM_STATS_MEASURE_FPS: hailo_pipeline_elem_stats_flags_t = 1;
pub const HAILO_PIPELINE_ELEM_STATS_MEASURE_LATENCY: hailo_pipeline_elem_stats_flags_t = 2;
pub const HAILO_PIPELINE_ELEM_STATS_MEASURE_QUEUE_SIZE: hailo_pipeline_elem_stats_flags_t = 4;

// --- Buffers and DMA --------------------------------------------------------

pub type hailo_dma_buffer_direction_t = c_int;
pub const HAILO_DMA_BUFFER_DIRECTION_H2D: hailo_dma_buffer_direction_t = 0;
pub const HAILO_DMA_BUFFER_DIRECTION_D2H: hailo_dma_buffer_direction_t = 1;
pub const HAILO_DMA_BUFFER_DIRECTION_BOTH: hailo_dma_buffer_direction_t = 2;

pub type hailo_buffer_flags_t = c_int;
pub const HAILO_BUFFER_FLAGS_NONE: hailo_buffer_flags_t = 0;
pub const HAILO_BUFFER_FLAGS_DMA: hailo_buffer_flags_t = 1;
pub const HAILO_BUFFER_FLAGS_CONTINUOUS: hailo_buffer_flags_t = 2;
pub const HAILO_BUFFER_FLAGS_SHARED_MEMORY: hailo_buffer_flags_t = 3;

pub type hailo_pix_buffer_memory_type_t = c_int;
pub const HAILO_PIX_BUFFER_MEMORY_TYPE_USERPTR: hailo_pix_buffer_memory_type_t = 0;
pub const HAILO_PIX_BUFFER_MEMORY_TYPE_DMABUF: hailo_pix_buffer_memory_type_t = 1;

// --- NMS detection ----------------------------------------------------------

pub type hailo_nms_burst_type_t = c_int;
pub const HAILO_BURST_TYPE_H8_BBOX: hailo_nms_burst_type_t = 0;
pub const HAILO_BURST_TYPE_H15_BBOX: hailo_nms_burst_type_t = 1;
pub const HAILO_BURST_TYPE_H8_PER_CLASS: hailo_nms_burst_type_t = 2;
pub const HAILO_BURST_TYPE_H15_PER_CLASS: hailo_nms_burst_type_t = 3;
pub const HAILO_BURST_TYPE_H15_PER_FRAME: hailo_nms_burst_type_t = 4;
pub const HAILO_BURST_TYPE_COUNT: hailo_nms_burst_type_t = 5;

// --- Health monitoring ------------------------------------------------------

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

pub type hailo_hw_throttling_state_t = c_int;
pub const HAILO_THROTTLING_STATE_NONE: hailo_hw_throttling_state_t = 0;
pub const HAILO_THROTTLING_STATE_0_LIGHT: hailo_hw_throttling_state_t = 1;
pub const HAILO_THROTTLING_STATE_1_MEDIUM: hailo_hw_throttling_state_t = 2;
pub const HAILO_THROTTLING_STATE_2_HEAVY: hailo_hw_throttling_state_t = 3;
pub const HAILO_THROTTLING_STATE_3_SEVERE: hailo_hw_throttling_state_t = 4;
pub const HAILO_THROTTLING_STATE_4_STREAMS_OFF: hailo_hw_throttling_state_t = 5;
pub const HAILO_THROTTLING_STATE_OVERHEAT: hailo_hw_throttling_state_t = 6;
pub const HAILO_THROTTLING_STATE_COUNT: hailo_hw_throttling_state_t = 7;

// --- Notifications ----------------------------------------------------------

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

// --- Sensors and firmware logging -------------------------------------------

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
