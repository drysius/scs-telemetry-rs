mod shm;
mod plugin;

use scs_telemetry_sys::{
    ScsS32, SCS_RESULT_OK, SCS_RESULT_UNSUPPORTED,
    SCS_TELEMETRY_VERSION_1_00, SCS_TELEMETRY_VERSION_1_01,
    ScsTelemetryInitParamsV100,
};

#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_init(
    version: u32,
    params: *const ScsTelemetryInitParamsV100,
) -> ScsS32 {
    if version != SCS_TELEMETRY_VERSION_1_00 && version != SCS_TELEMETRY_VERSION_1_01 {
        return SCS_RESULT_UNSUPPORTED;
    }
    if params.is_null() {
        return SCS_RESULT_UNSUPPORTED;
    }

    match plugin::init(&*params) {
        Ok(()) => SCS_RESULT_OK,
        Err(_) => SCS_RESULT_UNSUPPORTED,
    }
}

#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_shutdown() {
    plugin::shutdown();
}
