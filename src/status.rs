//! `hailo_status` return-code type and every named status constant.
//!
//! Every public HailoRT C function returns `hailo_status`.  Check the result
//! against [`HAILO_SUCCESS`] before using any output parameters.

use std::os::raw::c_int;

/// Return code for every HailoRT API call.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_is_zero() {
        assert_eq!(HAILO_SUCCESS, 0);
    }

    #[test]
    fn error_codes_1_through_50() {
        assert_eq!(HAILO_UNINITIALIZED, 1);
        assert_eq!(HAILO_INVALID_ARGUMENT, 2);
        assert_eq!(HAILO_OUT_OF_HOST_MEMORY, 3);
        assert_eq!(HAILO_TIMEOUT, 4);
        assert_eq!(HAILO_INSUFFICIENT_BUFFER, 5);
        assert_eq!(HAILO_INVALID_OPERATION, 6);
        assert_eq!(HAILO_NOT_IMPLEMENTED, 7);
        assert_eq!(HAILO_INTERNAL_FAILURE, 8);
        assert_eq!(HAILO_DATA_ALIGNMENT_FAILURE, 9);
        assert_eq!(HAILO_CHUNK_TOO_LARGE, 10);
        assert_eq!(HAILO_INVALID_LOGGER_LEVEL, 11);
        assert_eq!(HAILO_CLOSE_FAILURE, 12);
        assert_eq!(HAILO_OPEN_FILE_FAILURE, 13);
        assert_eq!(HAILO_FILE_OPERATION_FAILURE, 14);
        assert_eq!(HAILO_UNSUPPORTED_CONTROL_PROTOCOL_VERSION, 15);
        assert_eq!(HAILO_UNSUPPORTED_FW_VERSION, 16);
        assert_eq!(HAILO_INVALID_CONTROL_RESPONSE, 17);
        assert_eq!(HAILO_FW_CONTROL_FAILURE, 18);
        assert_eq!(HAILO_ETH_FAILURE, 19);
        assert_eq!(HAILO_ETH_INTERFACE_NOT_FOUND, 20);
        assert_eq!(HAILO_ETH_RECV_FAILURE, 21);
        assert_eq!(HAILO_ETH_SEND_FAILURE, 22);
        assert_eq!(HAILO_INVALID_FIRMWARE, 23);
        assert_eq!(HAILO_INVALID_CONTEXT_COUNT, 24);
        assert_eq!(HAILO_INVALID_FRAME, 25);
        assert_eq!(HAILO_INVALID_HEF, 26);
        assert_eq!(HAILO_PCIE_NOT_SUPPORTED_ON_PLATFORM, 27);
        assert_eq!(HAILO_INTERRUPTED_BY_SIGNAL, 28);
        assert_eq!(HAILO_START_VDMA_CHANNEL_FAIL, 29);
        assert_eq!(HAILO_SYNC_VDMA_BUFFER_FAIL, 30);
        assert_eq!(HAILO_STOP_VDMA_CHANNEL_FAIL, 31);
        assert_eq!(HAILO_CLOSE_VDMA_CHANNEL_FAIL, 32);
        assert_eq!(HAILO_ATR_TABLES_CONF_VALIDATION_FAIL, 33);
        assert_eq!(HAILO_EVENT_CREATE_FAIL, 34);
        assert_eq!(HAILO_READ_EVENT_FAIL, 35);
        assert_eq!(HAILO_DRIVER_OPERATION_FAILED, 36);
        assert_eq!(HAILO_INVALID_FIRMWARE_MAGIC, 37);
        assert_eq!(HAILO_INVALID_FIRMWARE_CODE_SIZE, 38);
        assert_eq!(HAILO_INVALID_KEY_CERTIFICATE_SIZE, 39);
        assert_eq!(HAILO_INVALID_CONTENT_CERTIFICATE_SIZE, 40);
        assert_eq!(HAILO_MISMATCHING_FIRMWARE_BUFFER_SIZES, 41);
        assert_eq!(HAILO_INVALID_FIRMWARE_CPU_ID, 42);
        assert_eq!(HAILO_CONTROL_RESPONSE_MD5_MISMATCH, 43);
        assert_eq!(HAILO_GET_CONTROL_RESPONSE_FAIL, 44);
        assert_eq!(HAILO_GET_D2H_EVENT_MESSAGE_FAIL, 45);
        assert_eq!(HAILO_MUTEX_INIT_FAIL, 46);
        assert_eq!(HAILO_OUT_OF_DESCRIPTORS, 47);
        assert_eq!(HAILO_UNSUPPORTED_OPCODE, 48);
        assert_eq!(HAILO_USER_MODE_RATE_LIMITER_NOT_SUPPORTED, 49);
        assert_eq!(HAILO_RATE_LIMIT_MAXIMUM_BANDWIDTH_EXCEEDED, 50);
    }

    #[test]
    fn error_codes_51_through_97() {
        assert_eq!(HAILO_ANSI_TO_UTF16_CONVERSION_FAILED, 51);
        assert_eq!(HAILO_UTF16_TO_ANSI_CONVERSION_FAILED, 52);
        assert_eq!(HAILO_UNEXPECTED_INTERFACE_INFO_FAILURE, 53);
        assert_eq!(HAILO_UNEXPECTED_ARP_TABLE_FAILURE, 54);
        assert_eq!(HAILO_MAC_ADDRESS_NOT_FOUND, 55);
        assert_eq!(HAILO_NO_IPV4_INTERFACES_FOUND, 56);
        assert_eq!(HAILO_SHUTDOWN_EVENT_SIGNALED, 57);
        assert_eq!(HAILO_THREAD_ALREADY_ACTIVATED, 58);
        assert_eq!(HAILO_THREAD_NOT_ACTIVATED, 59);
        assert_eq!(HAILO_THREAD_NOT_JOINABLE, 60);
        assert_eq!(HAILO_NOT_FOUND, 61);
        assert_eq!(HAILO_COMMUNICATION_CLOSED, 62);
        assert_eq!(HAILO_STREAM_ABORT, 63);
        assert_eq!(HAILO_DRIVER_NOT_INSTALLED, 64);
        assert_eq!(HAILO_NOT_AVAILABLE, 65);
        assert_eq!(HAILO_TRAFFIC_CONTROL_FAILURE, 66);
        assert_eq!(HAILO_INVALID_SECOND_STAGE, 67);
        assert_eq!(HAILO_INVALID_PIPELINE, 68);
        assert_eq!(HAILO_NETWORK_GROUP_NOT_ACTIVATED, 69);
        assert_eq!(HAILO_VSTREAM_PIPELINE_NOT_ACTIVATED, 70);
        assert_eq!(HAILO_OUT_OF_FW_MEMORY, 71);
        assert_eq!(HAILO_STREAM_NOT_ACTIVATED, 72);
        assert_eq!(HAILO_DEVICE_IN_USE, 73);
        assert_eq!(HAILO_OUT_OF_PHYSICAL_DEVICES, 74);
        assert_eq!(HAILO_INVALID_DEVICE_ARCHITECTURE, 75);
        assert_eq!(HAILO_INVALID_DRIVER_VERSION, 76);
        assert_eq!(HAILO_RPC_FAILED, 77);
        assert_eq!(HAILO_INVALID_SERVICE_VERSION, 78);
        assert_eq!(HAILO_NOT_SUPPORTED, 79);
        assert_eq!(HAILO_NMS_BURST_INVALID_DATA, 80);
        assert_eq!(HAILO_OUT_OF_HOST_CMA_MEMORY, 81);
        assert_eq!(HAILO_QUEUE_IS_FULL, 82);
        assert_eq!(HAILO_DMA_MAPPING_ALREADY_EXISTS, 83);
        assert_eq!(HAILO_CANT_MEET_BUFFER_REQUIREMENTS, 84);
        assert_eq!(HAILO_DRIVER_INVALID_RESPONSE, 85);
        assert_eq!(HAILO_DRIVER_INVALID_IOCTL, 86);
        assert_eq!(HAILO_DRIVER_TIMEOUT, 87);
        assert_eq!(HAILO_DRIVER_INTERRUPTED, 88);
        assert_eq!(HAILO_CONNECTION_REFUSED, 89);
        assert_eq!(HAILO_DRIVER_WAIT_CANCELED, 90);
        assert_eq!(HAILO_HEF_FILE_CORRUPTED, 91);
        assert_eq!(HAILO_HEF_NOT_SUPPORTED, 92);
        assert_eq!(HAILO_HEF_NOT_COMPATIBLE_WITH_DEVICE, 93);
        assert_eq!(HAILO_INVALID_HEF_USE, 94);
        assert_eq!(HAILO_OPERATION_ABORTED, 95);
        assert_eq!(HAILO_DEVICE_NOT_CONNECTED, 96);
        assert_eq!(HAILO_DEVICE_TEMPORARILY_UNAVAILABLE, 97);
    }
}
