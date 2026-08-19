use core::ffi::c_void;
use std::ffi::CString;
use scs_telemetry_sys::{
    ScsTelemetryInitParamsV100, ScsString, ScsU32, ScsValue,
    SCS_TELEMETRY_EVENT_CONFIGURATION, SCS_TELEMETRY_EVENT_FRAME_START,
    SCS_TELEMETRY_EVENT_PAUSED, SCS_TELEMETRY_EVENT_STARTED,
    SCS_TELEMETRY_EVENT_GAMEPLAY,
    SCS_TELEMETRY_CHANNEL_FLAG_NONE, SCS_TELEMETRY_CHANNEL_INDEX_NONE,
    SCS_VALUE_TYPE_BOOL, SCS_VALUE_TYPE_FLOAT, SCS_VALUE_TYPE_S32,
    SCS_VALUE_TYPE_U32, SCS_VALUE_TYPE_DPLACEMENT, SCS_VALUE_TYPE_FPLACEMENT,
    SCS_VALUE_TYPE_FVECTOR,
    // Truck channels
    SCS_TELEMETRY_TRUCK_CHANNEL_speed,
    SCS_TELEMETRY_TRUCK_CHANNEL_engine_rpm,
    SCS_TELEMETRY_TRUCK_CHANNEL_engine_gear,
    SCS_TELEMETRY_TRUCK_CHANNEL_displayed_gear,
    SCS_TELEMETRY_TRUCK_CHANNEL_fuel,
    SCS_TELEMETRY_TRUCK_CHANNEL_fuel_warning,
    SCS_TELEMETRY_TRUCK_CHANNEL_fuel_range,
    SCS_TELEMETRY_TRUCK_CHANNEL_fuel_average_consumption,
    SCS_TELEMETRY_TRUCK_CHANNEL_adblue,
    SCS_TELEMETRY_TRUCK_CHANNEL_adblue_warning,
    SCS_TELEMETRY_TRUCK_CHANNEL_adblue_average_consumption,
    SCS_TELEMETRY_TRUCK_CHANNEL_oil_pressure,
    SCS_TELEMETRY_TRUCK_CHANNEL_oil_pressure_warning,
    SCS_TELEMETRY_TRUCK_CHANNEL_oil_temperature,
    SCS_TELEMETRY_TRUCK_CHANNEL_water_temperature,
    SCS_TELEMETRY_TRUCK_CHANNEL_water_temperature_warning,
    SCS_TELEMETRY_TRUCK_CHANNEL_battery_voltage,
    SCS_TELEMETRY_TRUCK_CHANNEL_battery_voltage_warning,
    SCS_TELEMETRY_TRUCK_CHANNEL_electric_enabled,
    SCS_TELEMETRY_TRUCK_CHANNEL_engine_enabled,
    SCS_TELEMETRY_TRUCK_CHANNEL_parking_brake,
    SCS_TELEMETRY_TRUCK_CHANNEL_motor_brake,
    SCS_TELEMETRY_TRUCK_CHANNEL_cruise_control,
    SCS_TELEMETRY_TRUCK_CHANNEL_hshift_selector,
    SCS_TELEMETRY_TRUCK_CHANNEL_lblinker,
    SCS_TELEMETRY_TRUCK_CHANNEL_rblinker,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_lblinker,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_rblinker,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_parking,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_low_beam,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_high_beam,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_aux_front,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_aux_roof,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_beacon,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_brake,
    SCS_TELEMETRY_TRUCK_CHANNEL_light_reverse,
    SCS_TELEMETRY_TRUCK_CHANNEL_wipers,
    SCS_TELEMETRY_TRUCK_CHANNEL_dashboard_backlight,
    SCS_TELEMETRY_TRUCK_CHANNEL_wear_engine,
    SCS_TELEMETRY_TRUCK_CHANNEL_wear_transmission,
    SCS_TELEMETRY_TRUCK_CHANNEL_wear_cabin,
    SCS_TELEMETRY_TRUCK_CHANNEL_wear_chassis,
    SCS_TELEMETRY_TRUCK_CHANNEL_wear_wheels,
    SCS_TELEMETRY_TRUCK_CHANNEL_odometer,
    SCS_TELEMETRY_TRUCK_CHANNEL_navigation_distance,
    SCS_TELEMETRY_TRUCK_CHANNEL_navigation_time,
    SCS_TELEMETRY_TRUCK_CHANNEL_navigation_speed_limit,
    SCS_TELEMETRY_TRUCK_CHANNEL_input_steering,
    SCS_TELEMETRY_TRUCK_CHANNEL_input_throttle,
    SCS_TELEMETRY_TRUCK_CHANNEL_input_brake,
    SCS_TELEMETRY_TRUCK_CHANNEL_input_clutch,
    SCS_TELEMETRY_TRUCK_CHANNEL_effective_steering,
    SCS_TELEMETRY_TRUCK_CHANNEL_effective_throttle,
    SCS_TELEMETRY_TRUCK_CHANNEL_effective_brake,
    SCS_TELEMETRY_TRUCK_CHANNEL_effective_clutch,
    SCS_TELEMETRY_TRUCK_CHANNEL_retarder_level,
    SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure,
    SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure_warning,
    SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure_emergency,
    SCS_TELEMETRY_TRUCK_CHANNEL_brake_temperature,
    SCS_TELEMETRY_TRUCK_CHANNEL_world_placement,
    SCS_TELEMETRY_TRUCK_CHANNEL_local_linear_velocity,
    SCS_TELEMETRY_TRUCK_CHANNEL_local_angular_velocity,
    SCS_TELEMETRY_TRUCK_CHANNEL_local_linear_acceleration,
    SCS_TELEMETRY_TRUCK_CHANNEL_local_angular_acceleration,
    SCS_TELEMETRY_TRUCK_CHANNEL_cabin_offset,
    SCS_TELEMETRY_TRUCK_CHANNEL_cabin_angular_velocity,
    SCS_TELEMETRY_TRUCK_CHANNEL_cabin_angular_acceleration,
    SCS_TELEMETRY_TRUCK_CHANNEL_head_offset,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_susp_deflection,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_on_ground,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_substance,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_velocity,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_steering,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_rotation,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_lift,
    SCS_TELEMETRY_TRUCK_CHANNEL_wheel_lift_offset,
    // Trailer channels
    SCS_TELEMETRY_TRAILER_CHANNEL_connected,
    SCS_TELEMETRY_TRAILER_CHANNEL_world_placement,
    SCS_TELEMETRY_TRAILER_CHANNEL_local_linear_velocity,
    SCS_TELEMETRY_TRAILER_CHANNEL_local_angular_velocity,
    SCS_TELEMETRY_TRAILER_CHANNEL_local_linear_acceleration,
    SCS_TELEMETRY_TRAILER_CHANNEL_local_angular_acceleration,
    SCS_TELEMETRY_TRAILER_CHANNEL_wear_chassis,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_susp_deflection,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_on_ground,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_substance,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_velocity,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_steering,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_rotation,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_lift,
    SCS_TELEMETRY_TRAILER_CHANNEL_wheel_lift_offset,
    // Job
    SCS_TELEMETRY_JOB_CHANNEL_cargo_damage,
    // Config IDs and attributes
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_count,
    SCS_TELEMETRY_CONFIG_truck,
    SCS_TELEMETRY_CONFIG_trailer,
    SCS_TELEMETRY_CONFIG_job,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_brand_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_name,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_fuel_capacity,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_adblue_capacity,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_rpm_limit,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_forward_gear_count,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_reverse_gear_count,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_retarder_step_count,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_cabin_position,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_head_position,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_hook_position,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_accessory_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_mass,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_city_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_city,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_company_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_company,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_city_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_city,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_company_id,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_company,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_income,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_delivery_time,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_is_cargo_loaded,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_job_market,
    SCS_TELEMETRY_CONFIG_ATTRIBUTE_special_job,
    // Gameplay event IDs
    SCS_TELEMETRY_GAMEPLAY_EVENT_job_delivered,
    SCS_TELEMETRY_GAMEPLAY_EVENT_job_cancelled,
    // Gameplay event attributes
    SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_revenue,
    SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_earned_xp,
    SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_cargo_damage,
    SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_distance_km,
    SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_delivery_time,
};
use scs_telemetry_shared_memory::{TelemetryMap, MAX_WHEEL_COUNT, MAX_TRAILER_COUNT};
use crate::shm;

fn cs(s: &str) -> CString {
    CString::new(s).unwrap_or_else(|_| CString::new("").unwrap())
}

/// Copy bytes from a C string pointer into a fixed-size null-padded buffer.
unsafe fn copy_str(dst: &mut [u8], src: scs_telemetry_sys::ScsString) {
    if src.is_null() { return; }
    let cstr = core::ffi::CStr::from_ptr(src);
    let bytes = cstr.to_bytes();
    let len = bytes.len().min(dst.len() - 1);
    dst[..len].copy_from_slice(&bytes[..len]);
    dst[len..].fill(0);
}

pub fn init(params: &ScsTelemetryInitParamsV100) -> Result<(), ()> {
    shm::init()?;

    let reg_event = params.register_for_event;
    let reg_ch = params.register_for_channel;

    unsafe {
        reg_event(SCS_TELEMETRY_EVENT_FRAME_START,   on_frame_start,   core::ptr::null_mut());
        reg_event(SCS_TELEMETRY_EVENT_PAUSED,        on_paused,        core::ptr::null_mut());
        reg_event(SCS_TELEMETRY_EVENT_STARTED,       on_started,       core::ptr::null_mut());
        reg_event(SCS_TELEMETRY_EVENT_CONFIGURATION, on_configuration, core::ptr::null_mut());
        reg_event(SCS_TELEMETRY_EVENT_GAMEPLAY,      on_gameplay,      core::ptr::null_mut());
    }

    let reg_f = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_FLOAT, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };
    let reg_s32 = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_S32, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };
    let reg_u32 = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_U32, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };
    let reg_bool = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_BOOL, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };
    let reg_dplace = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_DPLACEMENT, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };
    let reg_fplace = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_FPLACEMENT, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };
    let reg_fvec = |name: &str, cb: scs_telemetry_sys::ScsTelemetryChannelCallbackFn| {
        let n = cs(name);
        unsafe { reg_ch(n.as_ptr(), SCS_TELEMETRY_CHANNEL_INDEX_NONE, SCS_VALUE_TYPE_FVECTOR, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, core::ptr::null_mut()); }
    };

    // --- Truck scalar channels ---
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_speed,                    ch_speed);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_engine_rpm,               ch_engine_rpm);
    reg_s32(SCS_TELEMETRY_TRUCK_CHANNEL_engine_gear,            ch_engine_gear);
    reg_s32(SCS_TELEMETRY_TRUCK_CHANNEL_displayed_gear,         ch_displayed_gear);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_fuel,                     ch_fuel);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_fuel_range,               ch_fuel_range);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_fuel_average_consumption, ch_fuel_avg);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_adblue,                   ch_adblue);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_adblue_average_consumption, ch_adblue_avg);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_oil_pressure,             ch_oil_pressure);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_oil_temperature,          ch_oil_temp);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_water_temperature,        ch_water_temp);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_battery_voltage,          ch_battery_voltage);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure,       ch_brake_air);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_brake_temperature,        ch_brake_temp);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_wear_engine,              ch_wear_engine);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_wear_transmission,        ch_wear_tx);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_wear_cabin,               ch_wear_cabin);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_wear_chassis,             ch_wear_chassis);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_wear_wheels,              ch_wear_wheels);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_odometer,                 ch_odometer);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_navigation_distance,      ch_nav_distance);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_navigation_time,          ch_nav_time);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_navigation_speed_limit,   ch_nav_speed_limit);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_input_steering,           ch_input_steering);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_input_throttle,           ch_input_throttle);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_input_brake,              ch_input_brake);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_input_clutch,             ch_input_clutch);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_effective_steering,       ch_eff_steering);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_effective_throttle,       ch_eff_throttle);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_effective_brake,          ch_eff_brake);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_effective_clutch,         ch_eff_clutch);
    reg_f(SCS_TELEMETRY_TRUCK_CHANNEL_dashboard_backlight,      ch_dashboard_backlight);
    reg_u32(SCS_TELEMETRY_TRUCK_CHANNEL_retarder_level,         ch_retarder);
    reg_u32(SCS_TELEMETRY_TRUCK_CHANNEL_light_aux_front,        ch_light_aux_front);
    reg_u32(SCS_TELEMETRY_TRUCK_CHANNEL_light_aux_roof,         ch_light_aux_roof);

    // --- Truck bool channels ---
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_electric_enabled,           ch_electric);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_engine_enabled,             ch_engine_on);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_parking_brake,              ch_parking_brake);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_motor_brake,                ch_motor_brake);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_cruise_control,             ch_cruise_control);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_lblinker,                   ch_lblinker);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_rblinker,                   ch_rblinker);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_lblinker,             ch_light_lblinker);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_rblinker,             ch_light_rblinker);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_parking,              ch_light_parking);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_low_beam,             ch_light_low);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_high_beam,            ch_light_high);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_beacon,               ch_light_beacon);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_brake,                ch_light_brake);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_light_reverse,              ch_light_reverse);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_wipers,                     ch_wipers);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_fuel_warning,               ch_fuel_warning);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_adblue_warning,             ch_adblue_warning);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_oil_pressure_warning,       ch_oil_pressure_warning);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_water_temperature_warning,  ch_water_temp_warning);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_battery_voltage_warning,    ch_battery_warning);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure_warning, ch_brake_air_warning);
    reg_bool(SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure_emergency, ch_brake_air_emergency);

    // --- Truck placement/vector channels ---
    reg_dplace(SCS_TELEMETRY_TRUCK_CHANNEL_world_placement,           ch_world_placement);
    reg_fplace(SCS_TELEMETRY_TRUCK_CHANNEL_cabin_offset,              ch_cabin_offset);
    reg_fplace(SCS_TELEMETRY_TRUCK_CHANNEL_head_offset,               ch_head_offset);
    reg_fvec(SCS_TELEMETRY_TRUCK_CHANNEL_local_linear_velocity,       ch_linear_vel);
    reg_fvec(SCS_TELEMETRY_TRUCK_CHANNEL_local_angular_velocity,      ch_angular_vel);
    reg_fvec(SCS_TELEMETRY_TRUCK_CHANNEL_local_linear_acceleration,   ch_linear_acc);
    reg_fvec(SCS_TELEMETRY_TRUCK_CHANNEL_local_angular_acceleration,  ch_angular_acc);
    reg_fvec(SCS_TELEMETRY_TRUCK_CHANNEL_cabin_angular_velocity,      ch_cabin_angular_vel);
    reg_fvec(SCS_TELEMETRY_TRUCK_CHANNEL_cabin_angular_acceleration,  ch_cabin_angular_acc);

    // --- Truck per-wheel channels (index = wheel slot 0..13) ---
    for i in 0..MAX_WHEEL_COUNT as u32 {
        let ctx = i as usize as *mut c_void;
        let wheel_channels: &[(&str, u32, scs_telemetry_sys::ScsTelemetryChannelCallbackFn)] = &[
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_susp_deflection, SCS_VALUE_TYPE_FLOAT, ch_wheel_susp),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_on_ground,       SCS_VALUE_TYPE_BOOL,  ch_wheel_on_ground),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_substance,       SCS_VALUE_TYPE_U32,   ch_wheel_substance),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_velocity,        SCS_VALUE_TYPE_FLOAT, ch_wheel_velocity),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_steering,        SCS_VALUE_TYPE_FLOAT, ch_wheel_steering),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_rotation,        SCS_VALUE_TYPE_FLOAT, ch_wheel_rotation),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_lift,            SCS_VALUE_TYPE_FLOAT, ch_wheel_lift),
            (SCS_TELEMETRY_TRUCK_CHANNEL_wheel_lift_offset,     SCS_VALUE_TYPE_FLOAT, ch_wheel_lift_offset),
        ];
        for &(name, type_, cb) in wheel_channels {
            let n = cs(name);
            unsafe { reg_ch(n.as_ptr(), i, type_, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, ctx); }
        }
    }

    // --- H-shifter selector (index = selector slot 0..1) ---
    for i in 0u32..2 {
        let ctx = i as usize as *mut c_void;
        let n = cs(SCS_TELEMETRY_TRUCK_CHANNEL_hshift_selector);
        unsafe { reg_ch(n.as_ptr(), i, SCS_VALUE_TYPE_U32, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_hshift_selector, ctx); }
    }

    // --- Trailer channels (index = trailer slot 0..9) ---
    for slot in 0..MAX_TRAILER_COUNT as u32 {
        let ctx_slot = slot as usize as *mut c_void;

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_connected);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_BOOL, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_connected, ctx_slot); }

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_world_placement);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_DPLACEMENT, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_placement, ctx_slot); }

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_local_linear_velocity);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_FVECTOR, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_linear_vel, ctx_slot); }

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_local_angular_velocity);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_FVECTOR, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_angular_vel, ctx_slot); }

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_local_linear_acceleration);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_FVECTOR, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_linear_acc, ctx_slot); }

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_local_angular_acceleration);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_FVECTOR, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_angular_acc, ctx_slot); }

        let n = cs(SCS_TELEMETRY_TRAILER_CHANNEL_wear_chassis);
        unsafe { reg_ch(n.as_ptr(), slot, SCS_VALUE_TYPE_FLOAT, SCS_TELEMETRY_CHANNEL_FLAG_NONE, ch_trailer_wear_chassis, ctx_slot); }

        // Per-trailer-wheel channels: encode as slot * MAX_WHEEL + wheel
        for w in 0..MAX_WHEEL_COUNT as u32 {
            let combined = slot * MAX_WHEEL_COUNT as u32 + w;
            let ctx_combined = combined as usize as *mut c_void;
            let trailer_wheel_channels: &[(&str, u32, scs_telemetry_sys::ScsTelemetryChannelCallbackFn)] = &[
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_susp_deflection, SCS_VALUE_TYPE_FLOAT, ch_twheel_susp),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_on_ground,       SCS_VALUE_TYPE_BOOL,  ch_twheel_on_ground),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_substance,       SCS_VALUE_TYPE_U32,   ch_twheel_substance),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_velocity,        SCS_VALUE_TYPE_FLOAT, ch_twheel_velocity),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_steering,        SCS_VALUE_TYPE_FLOAT, ch_twheel_steering),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_rotation,        SCS_VALUE_TYPE_FLOAT, ch_twheel_rotation),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_lift,            SCS_VALUE_TYPE_FLOAT, ch_twheel_lift),
                (SCS_TELEMETRY_TRAILER_CHANNEL_wheel_lift_offset,     SCS_VALUE_TYPE_FLOAT, ch_twheel_lift_offset),
            ];
            for &(name, type_, cb) in trailer_wheel_channels {
                let n = cs(name);
                unsafe { reg_ch(n.as_ptr(), combined, type_, SCS_TELEMETRY_CHANNEL_FLAG_NONE, cb, ctx_combined); }
            }
        }
    }

    reg_f(SCS_TELEMETRY_JOB_CHANNEL_cargo_damage, ch_cargo_damage);

    shm::with_map(|m| { m.sdk_active = 1; });
    Ok(())
}

pub fn shutdown() {
    shm::with_map(|m| { m.sdk_active = 0; });
    shm::shutdown();
}

// ---------------------------------------------------------------------------
// Event callbacks
// ---------------------------------------------------------------------------

unsafe extern "C" fn on_frame_start(event: ScsU32, info: *const c_void, _ctx: *mut c_void) {
    use scs_telemetry_sys::ScsTelemetryFrameStartInfo;
    let _ = event;
    if info.is_null() { return; }
    let fi = &*(info as *const ScsTelemetryFrameStartInfo);
    shm::with_map(|m| {
        m.time = (fi.simulation_time & 0xFFFF_FFFF) as u32;
        m.paused = 0;
    });
}

unsafe extern "C" fn on_paused(_event: ScsU32, _info: *const c_void, _ctx: *mut c_void) {
    shm::with_map(|m| { m.paused = 1; });
}

unsafe extern "C" fn on_started(_event: ScsU32, _info: *const c_void, _ctx: *mut c_void) {
    shm::with_map(|m| { m.paused = 0; });
}

unsafe extern "C" fn on_configuration(
    _event: ScsU32,
    info: *const c_void,
    _ctx: *mut c_void,
) {
    use scs_telemetry_sys::{ScsTelemetryConfiguration, ScsNamedValue, SCS_VALUE_TYPE_FLOAT, SCS_VALUE_TYPE_U32, SCS_VALUE_TYPE_BOOL, SCS_VALUE_TYPE_FVECTOR, SCS_VALUE_TYPE_STRING};
    if info.is_null() { return; }
    let cfg = &*(info as *const ScsTelemetryConfiguration);
    if cfg.id.is_null() { return; }
    let id = core::ffi::CStr::from_ptr(cfg.id).to_bytes();

    if id == SCS_TELEMETRY_CONFIG_truck.as_bytes() {
        let mut attr: *const ScsNamedValue = cfg.attributes;
        while !(*attr).name.is_null() {
            let aname = core::ffi::CStr::from_ptr((*attr).name).to_bytes();
            let val = &(*attr).value;
            if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_count.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.truck.wheel_count = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.truck.truck_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_brand_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.truck.truck_brand_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_name.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.truck.truck_name, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_fuel_capacity.as_bytes() && val.type_ == SCS_VALUE_TYPE_FLOAT {
                let v = val.value.value_float;
                shm::with_map(|m| { m.truck.fuel_capacity = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_adblue_capacity.as_bytes() && val.type_ == SCS_VALUE_TYPE_FLOAT {
                let v = val.value.value_float;
                shm::with_map(|m| { m.truck.adblue_capacity = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_rpm_limit.as_bytes() && val.type_ == SCS_VALUE_TYPE_FLOAT {
                let v = val.value.value_float;
                shm::with_map(|m| { m.truck.rpm_limit = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_forward_gear_count.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.truck.forward_gear_count = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_reverse_gear_count.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.truck.reverse_gear_count = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_retarder_step_count.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.truck.retarder_max = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_cabin_position.as_bytes() && val.type_ == SCS_VALUE_TYPE_FVECTOR {
                let v = val.value.value_fvector;
                shm::with_map(|m| {
                    m.truck.cabin_position = scs_telemetry_shared_memory::FVector { x: v.x, y: v.y, z: v.z };
                });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_head_position.as_bytes() && val.type_ == SCS_VALUE_TYPE_FVECTOR {
                let v = val.value.value_fvector;
                shm::with_map(|m| {
                    m.truck.head_position = scs_telemetry_shared_memory::FVector { x: v.x, y: v.y, z: v.z };
                });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_hook_position.as_bytes() && val.type_ == SCS_VALUE_TYPE_FVECTOR {
                let v = val.value.value_fvector;
                shm::with_map(|m| {
                    m.truck.hook_position = scs_telemetry_shared_memory::FVector { x: v.x, y: v.y, z: v.z };
                });
            }
            attr = attr.add(1);
        }
    } else if id == SCS_TELEMETRY_CONFIG_job.as_bytes() {
        // Clear job data before repopulating
        shm::with_map(|m| { m.job = unsafe { core::mem::zeroed() }; });
        let mut attr: *const ScsNamedValue = cfg.attributes;
        while !(*attr).name.is_null() {
            let aname = core::ffi::CStr::from_ptr((*attr).name).to_bytes();
            let val = &(*attr).value;
            if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.cargo_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.cargo_name, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_mass.as_bytes() && val.type_ == SCS_VALUE_TYPE_FLOAT {
                let v = val.value.value_float;
                shm::with_map(|m| { m.job.cargo_mass = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_city_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.source_city_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_city.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.source_city, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_company_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.source_company_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_company.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.source_company, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_city_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.destination_city_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_city.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.destination_city, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_company_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.destination_company_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_company.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.job.destination_company, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_income.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32 as u64;
                shm::with_map(|m| { m.job.income = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_delivery_time.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.job.delivery_time = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_is_cargo_loaded.as_bytes() && val.type_ == SCS_VALUE_TYPE_BOOL {
                let v = val.value.value_bool as u8;
                shm::with_map(|m| { m.job.is_cargo_loaded = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_job_market.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                // job.market is a string enum; store first byte so client can detect non-empty
                let ptr = val.value.value_string;
                if !ptr.is_null() {
                    let first = *ptr as u8;
                    shm::with_map(|m| { m.job.job_market = first; });
                }
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_special_job.as_bytes() && val.type_ == SCS_VALUE_TYPE_BOOL {
                let v = val.value.value_bool as u8;
                shm::with_map(|m| { m.job.special_job = v; });
            }
            attr = attr.add(1);
        }
    } else if id == SCS_TELEMETRY_CONFIG_trailer.as_bytes() {
        // Trailer config: update slot 0 only (single-trailer common case).
        // Multi-trailer: SCS fires this once per connected trailer; to fully
        // support it we would need a slot counter, which is left for a future revision.
        let mut attr: *const ScsNamedValue = cfg.attributes;
        while !(*attr).name.is_null() {
            let aname = core::ffi::CStr::from_ptr((*attr).name).to_bytes();
            let val = &(*attr).value;
            if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_count.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.trailers[0].wheel_count = v; });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_accessory_id.as_bytes() && val.type_ == SCS_VALUE_TYPE_STRING {
                let ptr = val.value.value_string;
                shm::with_map(|m| { copy_str(&mut m.trailers[0].cargo_accessory_id, ptr); });
            } else if aname == SCS_TELEMETRY_CONFIG_ATTRIBUTE_hook_position.as_bytes() && val.type_ == SCS_VALUE_TYPE_FVECTOR {
                let v = val.value.value_fvector;
                shm::with_map(|m| {
                    m.trailers[0].hook_position = scs_telemetry_shared_memory::FVector { x: v.x, y: v.y, z: v.z };
                });
            }
            attr = attr.add(1);
        }
    }
}

unsafe extern "C" fn on_gameplay(
    _event: ScsU32,
    info: *const c_void,
    _ctx: *mut c_void,
) {
    use scs_telemetry_sys::{ScsTelemetryGameplayEvent, ScsNamedValue, SCS_VALUE_TYPE_FLOAT, SCS_VALUE_TYPE_U32};
    if info.is_null() { return; }
    let ev = &*(info as *const ScsTelemetryGameplayEvent);
    if ev.id.is_null() { return; }
    let id = core::ffi::CStr::from_ptr(ev.id).to_bytes();

    if id == SCS_TELEMETRY_GAMEPLAY_EVENT_job_delivered.as_bytes() {
        let mut attr: *const ScsNamedValue = ev.attributes;
        while !(*attr).name.is_null() {
            let aname = core::ffi::CStr::from_ptr((*attr).name).to_bytes();
            let val = &(*attr).value;
            if aname == SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_revenue.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32 as u64;
                shm::with_map(|m| { m.job.income = v; });
            } else if aname == SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_earned_xp.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.job.xp_earned = v; });
            } else if aname == SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_cargo_damage.as_bytes() && val.type_ == SCS_VALUE_TYPE_FLOAT {
                let v = val.value.value_float;
                shm::with_map(|m| { m.job.cargo_damage = v; });
            } else if aname == SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_distance_km.as_bytes() && val.type_ == SCS_VALUE_TYPE_FLOAT {
                let v = val.value.value_float;
                shm::with_map(|m| { m.job.distance_km = v; });
            } else if aname == SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_delivery_time.as_bytes() && val.type_ == SCS_VALUE_TYPE_U32 {
                let v = val.value.value_u32;
                shm::with_map(|m| { m.job.delivery_time = v; });
            }
            attr = attr.add(1);
        }
    } else if id == SCS_TELEMETRY_GAMEPLAY_EVENT_job_cancelled.as_bytes() {
        shm::with_map(|m| { m.job = unsafe { core::mem::zeroed() }; });
    }
}

// ---------------------------------------------------------------------------
// Channel callbacks — macros
// ---------------------------------------------------------------------------

macro_rules! float_cb {
    ($name:ident, $assign:expr) => {
        unsafe extern "C" fn $name(
            _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
        ) {
            if value.is_null() { return; }
            let v = (*value).value.value_float;
            shm::with_map(|m| { $assign(m, v); });
        }
    };
}
macro_rules! s32_cb {
    ($name:ident, $assign:expr) => {
        unsafe extern "C" fn $name(
            _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
        ) {
            if value.is_null() { return; }
            let v = (*value).value.value_s32;
            shm::with_map(|m| { $assign(m, v); });
        }
    };
}
macro_rules! u32_cb {
    ($name:ident, $assign:expr) => {
        unsafe extern "C" fn $name(
            _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
        ) {
            if value.is_null() { return; }
            let v = (*value).value.value_u32;
            shm::with_map(|m| { $assign(m, v); });
        }
    };
}
macro_rules! bool_as_u32_cb {
    ($name:ident, $assign:expr) => {
        unsafe extern "C" fn $name(
            _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
        ) {
            if value.is_null() { return; }
            let v = (*value).value.value_bool as u32;
            shm::with_map(|m| { $assign(m, v); });
        }
    };
}

// ---------------------------------------------------------------------------
// Truck channel callbacks
// ---------------------------------------------------------------------------

float_cb!(ch_speed,               |m: &mut TelemetryMap, v| m.truck.speed = v);
float_cb!(ch_engine_rpm,          |m: &mut TelemetryMap, v| m.truck.engine_rpm = v);
s32_cb!(ch_engine_gear,           |m: &mut TelemetryMap, v| m.truck.engine_gear = v);
s32_cb!(ch_displayed_gear,        |m: &mut TelemetryMap, v| m.truck.displayed_gear = v);
float_cb!(ch_fuel,                |m: &mut TelemetryMap, v| m.truck.fuel = v);
float_cb!(ch_fuel_range,          |m: &mut TelemetryMap, v| m.truck.fuel_range = v);
float_cb!(ch_fuel_avg,            |m: &mut TelemetryMap, v| m.truck.fuel_average_consumption = v);
float_cb!(ch_adblue,              |m: &mut TelemetryMap, v| m.truck.adblue = v);
float_cb!(ch_adblue_avg,          |m: &mut TelemetryMap, v| m.truck.adblue_average_consumption = v);
float_cb!(ch_oil_pressure,        |m: &mut TelemetryMap, v| m.truck.oil_pressure = v);
float_cb!(ch_oil_temp,            |m: &mut TelemetryMap, v| m.truck.oil_temperature = v);
float_cb!(ch_water_temp,          |m: &mut TelemetryMap, v| m.truck.water_temperature = v);
float_cb!(ch_battery_voltage,     |m: &mut TelemetryMap, v| m.truck.battery_voltage = v);
float_cb!(ch_brake_air,           |m: &mut TelemetryMap, v| m.truck.brake_air_pressure = v);
float_cb!(ch_brake_temp,          |m: &mut TelemetryMap, v| m.truck.brake_temperature = v);
float_cb!(ch_wear_engine,         |m: &mut TelemetryMap, v| m.truck.wear_engine = v);
float_cb!(ch_wear_tx,             |m: &mut TelemetryMap, v| m.truck.wear_transmission = v);
float_cb!(ch_wear_cabin,          |m: &mut TelemetryMap, v| m.truck.wear_cabin = v);
float_cb!(ch_wear_chassis,        |m: &mut TelemetryMap, v| m.truck.wear_chassis = v);
float_cb!(ch_wear_wheels,         |m: &mut TelemetryMap, v| m.truck.wear_wheels = v);
float_cb!(ch_odometer,            |m: &mut TelemetryMap, v| m.truck.odometer = v);
float_cb!(ch_nav_distance,        |m: &mut TelemetryMap, v| m.truck.navigation_distance = v);
float_cb!(ch_nav_time,            |m: &mut TelemetryMap, v| m.truck.navigation_time = v);
float_cb!(ch_nav_speed_limit,     |m: &mut TelemetryMap, v| m.truck.navigation_speed_limit = v);
float_cb!(ch_input_steering,      |m: &mut TelemetryMap, v| m.truck.input_steering = v);
float_cb!(ch_input_throttle,      |m: &mut TelemetryMap, v| m.truck.input_throttle = v);
float_cb!(ch_input_brake,         |m: &mut TelemetryMap, v| m.truck.input_brake = v);
float_cb!(ch_input_clutch,        |m: &mut TelemetryMap, v| m.truck.input_clutch = v);
float_cb!(ch_eff_steering,        |m: &mut TelemetryMap, v| m.truck.effective_steering = v);
float_cb!(ch_eff_throttle,        |m: &mut TelemetryMap, v| m.truck.effective_throttle = v);
float_cb!(ch_eff_brake,           |m: &mut TelemetryMap, v| m.truck.effective_brake = v);
float_cb!(ch_eff_clutch,          |m: &mut TelemetryMap, v| m.truck.effective_clutch = v);
float_cb!(ch_dashboard_backlight, |m: &mut TelemetryMap, v| m.truck.dashboard_backlight = v);
float_cb!(ch_cargo_damage,        |m: &mut TelemetryMap, v| m.job.cargo_damage = v);
u32_cb!(ch_retarder,              |m: &mut TelemetryMap, v| m.truck.retarder_level = v);
u32_cb!(ch_light_aux_front,       |m: &mut TelemetryMap, v| m.truck.light_aux_front = v);
u32_cb!(ch_light_aux_roof,        |m: &mut TelemetryMap, v| m.truck.light_aux_roof = v);
bool_as_u32_cb!(ch_electric,             |m: &mut TelemetryMap, v| m.truck.electric_enabled = v);
bool_as_u32_cb!(ch_engine_on,            |m: &mut TelemetryMap, v| m.truck.engine_enabled = v);
bool_as_u32_cb!(ch_parking_brake,        |m: &mut TelemetryMap, v| m.truck.parking_brake = v);
bool_as_u32_cb!(ch_motor_brake,          |m: &mut TelemetryMap, v| m.truck.motor_brake = v);
bool_as_u32_cb!(ch_cruise_control,       |m: &mut TelemetryMap, v| m.truck.cruise_control = v);
bool_as_u32_cb!(ch_lblinker,             |m: &mut TelemetryMap, v| m.truck.lblinker = v);
bool_as_u32_cb!(ch_rblinker,             |m: &mut TelemetryMap, v| m.truck.rblinker = v);
bool_as_u32_cb!(ch_light_lblinker,       |m: &mut TelemetryMap, v| m.truck.light_lblinker = v);
bool_as_u32_cb!(ch_light_rblinker,       |m: &mut TelemetryMap, v| m.truck.light_rblinker = v);
bool_as_u32_cb!(ch_light_parking,        |m: &mut TelemetryMap, v| m.truck.light_parking = v);
bool_as_u32_cb!(ch_light_low,            |m: &mut TelemetryMap, v| m.truck.light_low_beam = v);
bool_as_u32_cb!(ch_light_high,           |m: &mut TelemetryMap, v| m.truck.light_high_beam = v);
bool_as_u32_cb!(ch_light_beacon,         |m: &mut TelemetryMap, v| m.truck.light_beacon = v);
bool_as_u32_cb!(ch_light_brake,          |m: &mut TelemetryMap, v| m.truck.light_brake = v);
bool_as_u32_cb!(ch_light_reverse,        |m: &mut TelemetryMap, v| m.truck.light_reverse = v);
bool_as_u32_cb!(ch_wipers,               |m: &mut TelemetryMap, v| m.truck.wipers = v);
bool_as_u32_cb!(ch_fuel_warning,         |m: &mut TelemetryMap, v| m.truck.fuel_warning = v);
bool_as_u32_cb!(ch_adblue_warning,       |m: &mut TelemetryMap, v| m.truck.adblue_warning = v);
bool_as_u32_cb!(ch_oil_pressure_warning, |m: &mut TelemetryMap, v| m.truck.oil_pressure_warning = v);
bool_as_u32_cb!(ch_water_temp_warning,   |m: &mut TelemetryMap, v| m.truck.water_temperature_warning = v);
bool_as_u32_cb!(ch_battery_warning,      |m: &mut TelemetryMap, v| m.truck.battery_voltage_warning = v);
bool_as_u32_cb!(ch_brake_air_warning,    |m: &mut TelemetryMap, v| m.truck.brake_air_pressure_warning = v);
bool_as_u32_cb!(ch_brake_air_emergency,  |m: &mut TelemetryMap, v| m.truck.brake_air_pressure_emergency = v);

unsafe extern "C" fn ch_hshift_selector(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void,
) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_u32;
    shm::with_map(|m| { if idx < 2 { m.truck.hshift_selector[idx] = v; } });
}

unsafe extern "C" fn ch_world_placement(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::{DPlacement, DVector, Euler};
    if value.is_null() { return; }
    let dp = (*value).value.value_dplacement;
    shm::with_map(|m| {
        m.truck.world_placement = DPlacement {
            position: DVector { x: dp.position.x, y: dp.position.y, z: dp.position.z },
            orientation: Euler { heading: dp.orientation.heading, pitch: dp.orientation.pitch, roll: dp.orientation.roll },
            _pad: [0; 4],
        };
    });
}

unsafe extern "C" fn ch_cabin_offset(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::{FPlacement, FVector, Euler};
    if value.is_null() { return; }
    let fp = (*value).value.value_fplacement;
    shm::with_map(|m| {
        m.truck.cabin_offset = FPlacement {
            position: FVector { x: fp.position.x, y: fp.position.y, z: fp.position.z },
            orientation: Euler { heading: fp.orientation.heading, pitch: fp.orientation.pitch, roll: fp.orientation.roll },
        };
    });
}

unsafe extern "C" fn ch_head_offset(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::{FPlacement, FVector, Euler};
    if value.is_null() { return; }
    let fp = (*value).value.value_fplacement;
    shm::with_map(|m| {
        m.truck.head_offset = FPlacement {
            position: FVector { x: fp.position.x, y: fp.position.y, z: fp.position.z },
            orientation: Euler { heading: fp.orientation.heading, pitch: fp.orientation.pitch, roll: fp.orientation.roll },
        };
    });
}

unsafe extern "C" fn ch_linear_vel(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { m.truck.local_linear_velocity = FVector { x: v.x, y: v.y, z: v.z }; });
}

unsafe extern "C" fn ch_angular_vel(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { m.truck.local_angular_velocity = FVector { x: v.x, y: v.y, z: v.z }; });
}

unsafe extern "C" fn ch_linear_acc(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { m.truck.local_linear_acceleration = FVector { x: v.x, y: v.y, z: v.z }; });
}

unsafe extern "C" fn ch_angular_acc(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { m.truck.local_angular_acceleration = FVector { x: v.x, y: v.y, z: v.z }; });
}

unsafe extern "C" fn ch_cabin_angular_vel(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { m.truck.cabin_angular_velocity = FVector { x: v.x, y: v.y, z: v.z }; });
}

unsafe extern "C" fn ch_cabin_angular_acc(
    _n: ScsString, _i: ScsU32, value: *const ScsValue, _ctx: *mut c_void,
) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { m.truck.cabin_angular_acceleration = FVector { x: v.x, y: v.y, z: v.z }; });
}

// Truck per-wheel callbacks (ctx = wheel index)

unsafe extern "C" fn ch_wheel_susp(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].susp_deflection = v; } });
}
unsafe extern "C" fn ch_wheel_on_ground(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_bool as u32;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].on_ground = v; } });
}
unsafe extern "C" fn ch_wheel_substance(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_u32;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].substance = v; } });
}
unsafe extern "C" fn ch_wheel_velocity(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].velocity = v; } });
}
unsafe extern "C" fn ch_wheel_steering(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].steering = v; } });
}
unsafe extern "C" fn ch_wheel_rotation(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].rotation = v; } });
}
unsafe extern "C" fn ch_wheel_lift(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].lift = v; } });
}
unsafe extern "C" fn ch_wheel_lift_offset(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let idx = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if idx < MAX_WHEEL_COUNT { m.truck.wheels[idx].lift_offset = v; } });
}

// ---------------------------------------------------------------------------
// Trailer channel callbacks (ctx = trailer slot)
// ---------------------------------------------------------------------------

unsafe extern "C" fn ch_trailer_connected(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let slot = ctx as usize;
    let v = (*value).value.value_bool;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT { m.trailers[slot].connected = v; } });
}

unsafe extern "C" fn ch_trailer_placement(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    use scs_telemetry_shared_memory::common::{DPlacement, DVector, Euler};
    if value.is_null() { return; }
    let slot = ctx as usize;
    let dp = (*value).value.value_dplacement;
    shm::with_map(|m| {
        if slot < MAX_TRAILER_COUNT {
            m.trailers[slot].world_placement = DPlacement {
                position: DVector { x: dp.position.x, y: dp.position.y, z: dp.position.z },
                orientation: Euler { heading: dp.orientation.heading, pitch: dp.orientation.pitch, roll: dp.orientation.roll },
                _pad: [0; 4],
            };
        }
    });
}

unsafe extern "C" fn ch_trailer_linear_vel(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let slot = ctx as usize;
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT { m.trailers[slot].local_linear_velocity = FVector { x: v.x, y: v.y, z: v.z }; } });
}

unsafe extern "C" fn ch_trailer_angular_vel(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let slot = ctx as usize;
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT { m.trailers[slot].local_angular_velocity = FVector { x: v.x, y: v.y, z: v.z }; } });
}

unsafe extern "C" fn ch_trailer_linear_acc(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let slot = ctx as usize;
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT { m.trailers[slot].local_linear_acceleration = FVector { x: v.x, y: v.y, z: v.z }; } });
}

unsafe extern "C" fn ch_trailer_angular_acc(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    use scs_telemetry_shared_memory::common::FVector;
    if value.is_null() { return; }
    let slot = ctx as usize;
    let v = (*value).value.value_fvector;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT { m.trailers[slot].local_angular_acceleration = FVector { x: v.x, y: v.y, z: v.z }; } });
}

unsafe extern "C" fn ch_trailer_wear_chassis(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let slot = ctx as usize;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT { m.trailers[slot].wear_chassis = v; } });
}

// Trailer per-wheel callbacks (ctx = slot * MAX_WHEEL + wheel)

unsafe extern "C" fn ch_twheel_susp(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].susp_deflection = v; } });
}
unsafe extern "C" fn ch_twheel_on_ground(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_bool as u32;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].on_ground = v; } });
}
unsafe extern "C" fn ch_twheel_substance(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_u32;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].substance = v; } });
}
unsafe extern "C" fn ch_twheel_velocity(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].velocity = v; } });
}
unsafe extern "C" fn ch_twheel_steering(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].steering = v; } });
}
unsafe extern "C" fn ch_twheel_rotation(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].rotation = v; } });
}
unsafe extern "C" fn ch_twheel_lift(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].lift = v; } });
}
unsafe extern "C" fn ch_twheel_lift_offset(_n: ScsString, _i: ScsU32, value: *const ScsValue, ctx: *mut c_void) {
    if value.is_null() { return; }
    let combined = ctx as usize;
    let slot = combined / MAX_WHEEL_COUNT;
    let w = combined % MAX_WHEEL_COUNT;
    let v = (*value).value.value_float;
    shm::with_map(|m| { if slot < MAX_TRAILER_COUNT && w < MAX_WHEEL_COUNT { m.trailers[slot].wheels[w].lift_offset = v; } });
}
