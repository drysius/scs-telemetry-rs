/// Name of the Windows named shared memory object
pub const SHARED_MEMORY_NAME: &str = "Local\\SCSTelemetry";

/// Binary layout revision — must match scs-sdk-plugin PLUGIN_REVISION
pub const PLUGIN_REVISION: u32 = 12;

pub const MAX_WHEEL_COUNT: usize = 14;
pub const MAX_TRAILER_COUNT: usize = 10;

/// Each string field in the shared memory struct is this many bytes (UTF-8, null-padded)
pub const STRING_SIZE: usize = 64;
