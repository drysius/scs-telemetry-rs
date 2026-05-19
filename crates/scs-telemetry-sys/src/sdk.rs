use core::ffi::{c_char, c_float, c_int, c_uint, c_void};

// ---------------------------------------------------------------------------
// Primitive type aliases matching scssdk.h
// ---------------------------------------------------------------------------

pub type ScsU8 = u8;
pub type ScsU16 = u16;
pub type ScsU32 = c_uint;
pub type ScsU64 = u64;
pub type ScsS32 = c_int;
pub type ScsFloat = c_float;
pub type ScsDouble = f64;
pub type ScsBool = u8;
pub type ScsString = *const c_char;

// ---------------------------------------------------------------------------
// Value type tags (scssdk_value.h)
// ---------------------------------------------------------------------------

pub const SCS_VALUE_TYPE_INVALID: ScsU32 = 0;
pub const SCS_VALUE_TYPE_BOOL: ScsU32 = 1;
pub const SCS_VALUE_TYPE_S32: ScsU32 = 2;
pub const SCS_VALUE_TYPE_U32: ScsU32 = 3;
pub const SCS_VALUE_TYPE_U64: ScsU32 = 4;
pub const SCS_VALUE_TYPE_FLOAT: ScsU32 = 5;
pub const SCS_VALUE_TYPE_DOUBLE: ScsU32 = 6;
pub const SCS_VALUE_TYPE_FVECTOR: ScsU32 = 7;
pub const SCS_VALUE_TYPE_DVECTOR: ScsU32 = 8;
pub const SCS_VALUE_TYPE_EULER: ScsU32 = 9;
pub const SCS_VALUE_TYPE_FPLACEMENT: ScsU32 = 10;
pub const SCS_VALUE_TYPE_DPLACEMENT: ScsU32 = 11;
pub const SCS_VALUE_TYPE_STRING: ScsU32 = 12;
pub const SCS_VALUE_TYPE_S64: ScsU32 = 13;

// ---------------------------------------------------------------------------
// Math types (#[repr(C)] exact C struct layout)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScsFVector {
    pub x: ScsFloat,
    pub y: ScsFloat,
    pub z: ScsFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScsDVector {
    pub x: ScsDouble,
    pub y: ScsDouble,
    pub z: ScsDouble,
}

/// Euler angles (heading, pitch, roll) in radians
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScsEuler {
    pub heading: ScsFloat,
    pub pitch: ScsFloat,
    pub roll: ScsFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScsFPlacement {
    pub position: ScsFVector,
    pub orientation: ScsEuler,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScsDPlacement {
    pub position: ScsDVector,
    pub orientation: ScsEuler,
}

// ---------------------------------------------------------------------------
// Value union / tagged value (scssdk_value.h: scs_value_t)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub union ScsValueUnion {
    pub value_bool: ScsBool,
    pub value_s32: ScsS32,
    pub value_u32: ScsU32,
    pub value_u64: ScsU64,
    pub value_float: ScsFloat,
    pub value_double: ScsDouble,
    pub value_fvector: ScsFVector,
    pub value_dvector: ScsDVector,
    pub value_euler: ScsEuler,
    pub value_fplacement: ScsFPlacement,
    pub value_dplacement: ScsDPlacement,
    pub value_string: ScsString,
    pub value_s64: i64,
    _pad: [u8; 40],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScsValue {
    pub type_: ScsU32,
    pub _pad: ScsU32,
    pub value: ScsValueUnion,
}

// ---------------------------------------------------------------------------
// Named value (used in config/gameplay events)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct ScsNamedValue {
    pub name: ScsString,
    pub index: ScsU32,
    pub _pad: ScsU32,
    pub value: ScsValue,
}

// ---------------------------------------------------------------------------
// Telemetry API version constants (scssdk_telemetry.h)
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_VERSION_1_00: ScsU32 = 0x0001_0000;
pub const SCS_TELEMETRY_VERSION_1_01: ScsU32 = 0x0001_0001;

/// Returned from scs_telemetry_init to signal success
pub const SCS_RESULT_OK: ScsS32 = 0;
/// Returned to signal unsupported version
pub const SCS_RESULT_UNSUPPORTED: ScsS32 = -1;
/// Generic error
pub const SCS_RESULT_GENERIC_ERROR: ScsS32 = -2;
pub const SCS_RESULT_ALREADY_REGISTERED: ScsS32 = -3;
pub const SCS_RESULT_NOT_FOUND: ScsS32 = -4;
pub const SCS_RESULT_UNSUPPORTED_TYPE: ScsS32 = -5;
pub const SCS_RESULT_NOT_NOW: ScsS32 = -6;
pub const SCS_RESULT_EMPTY: ScsS32 = -7;

// ---------------------------------------------------------------------------
// Log types
// ---------------------------------------------------------------------------

pub const SCS_LOG_TYPE_MESSAGE: ScsS32 = 0;
pub const SCS_LOG_TYPE_WARNING: ScsS32 = 1;
pub const SCS_LOG_TYPE_ERROR: ScsS32 = 2;

// ---------------------------------------------------------------------------
// Callback function pointer types
// ---------------------------------------------------------------------------

pub type ScsLogFn = unsafe extern "C" fn(log_type: ScsS32, message: ScsString);

pub type ScsTelemetryRegisterForChannelFn = unsafe extern "C" fn(
    name: ScsString,
    index: ScsU32,
    type_: ScsU32,
    flags: ScsU32,
    callback: ScsTelemetryChannelCallbackFn,
    context: *mut c_void,
) -> ScsS32;

pub type ScsTelemetryUnregisterFromChannelFn = unsafe extern "C" fn(
    name: ScsString,
    index: ScsU32,
    type_: ScsU32,
    callback: ScsTelemetryChannelCallbackFn,
    context: *mut c_void,
) -> ScsS32;

pub type ScsTelemetryChannelCallbackFn = unsafe extern "C" fn(
    name: ScsString,
    index: ScsU32,
    value: *const ScsValue,
    context: *mut c_void,
);

pub type ScsTelemetryEventCallbackFn =
    unsafe extern "C" fn(event: ScsU32, event_info: *const c_void, context: *mut c_void);

pub type ScsTelemetryRegisterForEventFn = unsafe extern "C" fn(
    event: ScsU32,
    callback: ScsTelemetryEventCallbackFn,
    context: *mut c_void,
) -> ScsS32;

pub type ScsTelemetryUnregisterFromEventFn = unsafe extern "C" fn(
    event: ScsU32,
    callback: ScsTelemetryEventCallbackFn,
    context: *mut c_void,
) -> ScsS32;

// ---------------------------------------------------------------------------
// Init params struct (scssdk_telemetry.h: scs_telemetry_init_params_v100_t)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct ScsTelemetryInitParamsV100 {
    pub common: ScsInitParams,
    pub register_for_event: ScsTelemetryRegisterForEventFn,
    pub unregister_from_event: ScsTelemetryUnregisterFromEventFn,
    pub register_for_channel: ScsTelemetryRegisterForChannelFn,
    pub unregister_from_channel: ScsTelemetryUnregisterFromChannelFn,
}

/// Common base — present in all API versions
#[repr(C)]
pub struct ScsInitParams {
    pub game_id: ScsString,
    pub game_version: ScsU32,
    pub game_name: ScsString,
    pub log: ScsLogFn,
}

// ---------------------------------------------------------------------------
// Telemetry event IDs
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_EVENT_INVALID: ScsU32 = 0;
pub const SCS_TELEMETRY_EVENT_FRAME_START: ScsU32 = 1;
pub const SCS_TELEMETRY_EVENT_FRAME_END: ScsU32 = 2;
pub const SCS_TELEMETRY_EVENT_PAUSED: ScsU32 = 3;
pub const SCS_TELEMETRY_EVENT_STARTED: ScsU32 = 4;
pub const SCS_TELEMETRY_EVENT_CONFIGURATION: ScsU32 = 5;
pub const SCS_TELEMETRY_EVENT_GAMEPLAY: ScsU32 = 6;

// ---------------------------------------------------------------------------
// Frame start info (scssdk_telemetry.h)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct ScsTelemetryFrameStartInfo {
    pub flags: ScsU32,
    pub _pad: ScsU32,
    pub render_time: ScsU64,
    pub simulation_time: ScsU64,
    pub paused_simulation_time: ScsU64,
}

// ---------------------------------------------------------------------------
// Configuration / gameplay event data
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct ScsTelemetryConfiguration {
    pub id: ScsString,
    pub attributes: *const ScsNamedValue,
}

#[repr(C)]
pub struct ScsTelemetryGameplayEvent {
    pub id: ScsString,
    pub attributes: *const ScsNamedValue,
}

// ---------------------------------------------------------------------------
// Channel registration flags
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_CHANNEL_FLAG_NONE: ScsU32 = 0;
/// No value when paused
pub const SCS_TELEMETRY_CHANNEL_FLAG_NO_VALUE: ScsU32 = 0x0000_0001;

pub const SCS_TELEMETRY_CHANNEL_INDEX_NONE: ScsU32 = 0xFFFF_FFFF;
