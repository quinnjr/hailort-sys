//! Raw `extern "C"` declarations for every public HailoRT function.
//!
//! All functions here are `unsafe`.  The `#[link(name = "hailort")]`
//! attribute instructs the linker to pull in `libhailort.so`; `build.rs`
//! tells Cargo where to find it.

use std::os::raw::{c_char, c_void};

use crate::enums::{
    hailo_averaging_factor_t, hailo_cpu_id_t, hailo_device_type_t, hailo_dvm_options_t,
    hailo_fw_logger_level_t,
    hailo_measurement_buffer_index_t, hailo_notification_id_t,
    hailo_power_measurement_types_t, hailo_reset_device_mode_t, hailo_sampling_period_t,
    hailo_sensor_types_t, hailo_stream_direction_t, hailo_watchdog_mode_t,
};
use crate::handles::{
    hailo_activated_network_group, hailo_configured_network_group, hailo_device,
    hailo_hef, hailo_input_stream, hailo_input_transform_context, hailo_input_vstream,
    hailo_output_demuxer, hailo_output_stream, hailo_output_transform_context,
    hailo_output_vstream, hailo_scan_devices_params_t, hailo_vdevice,
};
use crate::status::hailo_status;
use crate::types::{
    float32_t, hailo_activate_network_group_params_t, hailo_chip_temperature_info_t,
    hailo_configure_params_t, hailo_core_information_t, hailo_demux_params_t,
    hailo_device_id_t, hailo_device_identity_t, hailo_extended_device_information_t,
    hailo_format_t, hailo_health_info_t, hailo_health_stats_t,
    hailo_i2c_slave_config_t, hailo_input_vstream_params_by_name_t,
    hailo_latency_measurement_result_t, hailo_network_group_info_t, hailo_network_info_t,
    hailo_notification_callback, hailo_output_vstream_params_by_name_t,
    hailo_pcie_device_info_t, hailo_performance_stats_t, hailo_power_measurement_data_t,
    hailo_quant_info_t, hailo_stream_info_t, hailo_stream_read_async_callback_t,
    hailo_stream_write_async_callback_t, hailo_transform_params_t, hailo_version_t,
    hailo_vdevice_params_t, hailo_vstream_info_t, hailo_vstream_params_t,
};

#[link(name = "hailort")]
unsafe extern "C" {
    // --- Library version and status -----------------------------------------

    pub fn hailo_get_library_version(version: *mut hailo_version_t) -> hailo_status;
    pub fn hailo_get_status_message(status: hailo_status) -> *const c_char;

    // --- Device discovery and creation --------------------------------------

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

    // --- Device identification ----------------------------------------------

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

    // --- Firmware and system control ----------------------------------------

    pub fn hailo_set_fw_logger(
        device: hailo_device,
        level: hailo_fw_logger_level_t,
        interface_mask: u32,
    ) -> hailo_status;

    pub fn hailo_set_throttling_state(
        device: hailo_device,
        should_activate: bool,
    ) -> hailo_status;

    pub fn hailo_get_throttling_state(device: hailo_device, is_active: *mut bool) -> hailo_status;

    pub fn hailo_wd_enable(device: hailo_device, cpu_id: hailo_cpu_id_t) -> hailo_status;

    pub fn hailo_wd_disable(device: hailo_device, cpu_id: hailo_cpu_id_t) -> hailo_status;

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

    // --- Temperature and reset ----------------------------------------------

    pub fn hailo_get_chip_temperature(
        device: hailo_device,
        temp_info: *mut hailo_chip_temperature_info_t,
    ) -> hailo_status;

    pub fn hailo_reset_device(
        device: hailo_device,
        mode: hailo_reset_device_mode_t,
    ) -> hailo_status;

    // --- Firmware update ----------------------------------------------------

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

    // --- Notification management --------------------------------------------

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

    // --- Sensor management --------------------------------------------------

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

    // --- I²C ----------------------------------------------------------------

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

    // --- Diagnostics --------------------------------------------------------

    pub fn hailo_test_chip_memories(device: hailo_device) -> hailo_status;

    // --- Power measurement --------------------------------------------------

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

    // --- Health monitoring --------------------------------------------------

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

    // --- VDevice ------------------------------------------------------------

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

    // --- HEF ----------------------------------------------------------------

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

    // --- Configure device ---------------------------------------------------

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

    // --- Network group activation -------------------------------------------

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

    // --- Synchronous stream I/O ---------------------------------------------

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

    // --- Asynchronous stream I/O --------------------------------------------

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

    // --- Quantisation info --------------------------------------------------

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

    // --- Transform contexts -------------------------------------------------

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

    // --- Output demuxer -----------------------------------------------------

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

    // --- Virtual streams (vstreams) -----------------------------------------

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

    // --- Default parameter initialisation -----------------------------------

    pub fn hailo_get_default_vstream_params(
        vstream_info: *const hailo_vstream_info_t,
        user_buffer_format: hailo_format_t,
        direction: hailo_stream_direction_t,
        params: *mut hailo_vstream_params_t,
    ) -> hailo_status;

    // --- Latency measurement ------------------------------------------------

    pub fn hailo_input_vstream_get_latency_measurement(
        vstream: hailo_input_vstream,
        result: *mut hailo_latency_measurement_result_t,
    ) -> hailo_status;
}
