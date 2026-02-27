//! Sizing limits, array capacities, and default parameter values.
//!
//! These mirror the `#define` constants in `<hailo/hailort.h>`.

// --- Name / ID lengths ------------------------------------------------------

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

// --- Topology limits --------------------------------------------------------

pub const HAILO_MAX_STREAMS_COUNT: usize = 40;
pub const HAILO_MAX_NETWORK_GROUPS: usize = 8;
pub const HAILO_MAX_NETWORKS_IN_NETWORK_GROUP: usize = 8;

// --- Hardware identifier lengths --------------------------------------------

pub const HAILO_SOC_ID_LENGTH: usize = 32;
pub const HAILO_ETH_MAC_LENGTH: usize = 6;
pub const HAILO_UNIT_LEVEL_TRACKING_BYTES_LENGTH: usize = 12;
pub const HAILO_SOC_PM_VALUES_BYTES_LENGTH: usize = 24;
pub const HAILO_MAX_TEMPERATURE_THROTTLING_LEVELS_NUMBER: usize = 4;

// --- Image buffer limits ----------------------------------------------------

pub const MAX_NUMBER_OF_PLANES: usize = 4;
pub const NUMBER_OF_PLANES_NV12_NV21: usize = 2;
pub const NUMBER_OF_PLANES_I420: usize = 3;

// --- Sentinel values --------------------------------------------------------

pub const HAILO_INFINITE: u32 = u32::MAX;
pub const HAILO_PCIE_ANY_DOMAIN: u32 = u32::MAX;
pub const HAILO_RANDOM_SEED: u32 = u32::MAX;

// --- Default parameter values -----------------------------------------------

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

// --- Scheduler priority range -----------------------------------------------

pub const HAILO_SCHEDULER_PRIORITY_NORMAL: u8 = 16;
pub const HAILO_SCHEDULER_PRIORITY_MAX: u8 = 31;
pub const HAILO_SCHEDULER_PRIORITY_MIN: u8 = 0;
