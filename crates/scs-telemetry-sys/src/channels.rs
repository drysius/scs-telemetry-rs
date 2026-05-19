#![allow(non_upper_case_globals)]

/// Channel name constants from scssdk_telemetry_truck_common_channels.h
/// and scssdk_telemetry_job_common_channels.h

// ---------------------------------------------------------------------------
// Truck channels
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_TRUCK_CHANNEL_world_placement: &str =
    "truck.world.placement";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_local_linear_velocity: &str =
    "truck.local.velocity.linear";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_local_angular_velocity: &str =
    "truck.local.velocity.angular";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_local_linear_acceleration: &str =
    "truck.local.acceleration.linear";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_local_angular_acceleration: &str =
    "truck.local.acceleration.angular";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_cabin_offset: &str =
    "truck.cabin.offset";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_cabin_angular_velocity: &str =
    "truck.cabin.velocity.angular";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_cabin_angular_acceleration: &str =
    "truck.cabin.acceleration.angular";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_head_offset: &str =
    "truck.head.offset";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_speed: &str =
    "truck.speed";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_engine_rpm: &str =
    "truck.engine.rpm";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_engine_gear: &str =
    "truck.engine.gear";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_displayed_gear: &str =
    "truck.displayed.gear";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_input_steering: &str =
    "truck.input.steering";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_input_throttle: &str =
    "truck.input.throttle";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_input_brake: &str =
    "truck.input.brake";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_input_clutch: &str =
    "truck.input.clutch";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_effective_steering: &str =
    "truck.effective.steering";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_effective_throttle: &str =
    "truck.effective.throttle";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_effective_brake: &str =
    "truck.effective.brake";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_effective_clutch: &str =
    "truck.effective.clutch";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_cruise_control: &str =
    "truck.cruise_control";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_hshift_selector: &str =
    "truck.hshift.selector";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_parking_brake: &str =
    "truck.parking.brake";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_motor_brake: &str =
    "truck.motor.brake";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_retarder_level: &str =
    "truck.retarder.level";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure: &str =
    "truck.brake.air.pressure";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure_warning: &str =
    "truck.brake.air.pressure.warning";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_brake_air_pressure_emergency: &str =
    "truck.brake.air.pressure.emergency";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_brake_temperature: &str =
    "truck.brake.temperature";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_fuel: &str =
    "truck.fuel.amount";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_fuel_warning: &str =
    "truck.fuel.warning";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_fuel_average_consumption: &str =
    "truck.fuel.consumption.average";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_fuel_range: &str =
    "truck.fuel.range";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_adblue: &str =
    "truck.adblue";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_adblue_warning: &str =
    "truck.adblue.warning";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_adblue_average_consumption: &str =
    "truck.adblue.consumption.average";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_oil_pressure: &str =
    "truck.oil.pressure";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_oil_pressure_warning: &str =
    "truck.oil.pressure.warning";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_oil_temperature: &str =
    "truck.oil.temperature";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_water_temperature: &str =
    "truck.water.temperature";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_water_temperature_warning: &str =
    "truck.water.temperature.warning";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_battery_voltage: &str =
    "truck.battery.voltage";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_battery_voltage_warning: &str =
    "truck.battery.voltage.warning";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_electric_enabled: &str =
    "truck.electric.enabled";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_engine_enabled: &str =
    "truck.engine.enabled";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_lblinker: &str =
    "truck.lblinker";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_rblinker: &str =
    "truck.rblinker";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_lblinker: &str =
    "truck.light.lblinker";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_rblinker: &str =
    "truck.light.rblinker";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_parking: &str =
    "truck.light.parking";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_low_beam: &str =
    "truck.light.beam.low";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_high_beam: &str =
    "truck.light.beam.high";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_aux_front: &str =
    "truck.light.aux.front";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_aux_roof: &str =
    "truck.light.aux.roof";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_beacon: &str =
    "truck.light.beacon";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_brake: &str =
    "truck.light.brake";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_light_reverse: &str =
    "truck.light.reverse";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wipers: &str =
    "truck.wipers";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_dashboard_backlight: &str =
    "truck.dashboard.backlight";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wear_engine: &str =
    "truck.wear.engine";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wear_transmission: &str =
    "truck.wear.transmission";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wear_cabin: &str =
    "truck.wear.cabin";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wear_chassis: &str =
    "truck.wear.chassis";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wear_wheels: &str =
    "truck.wear.wheels";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_odometer: &str =
    "truck.odometer";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_navigation_distance: &str =
    "truck.navigation.distance";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_navigation_time: &str =
    "truck.navigation.time";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_navigation_speed_limit: &str =
    "truck.navigation.speed.limit";

// Per-wheel channels (use with index 0..N)
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_susp_deflection: &str =
    "truck.wheel.suspension.deflection";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_on_ground: &str =
    "truck.wheel.on_ground";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_substance: &str =
    "truck.wheel.substance";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_velocity: &str =
    "truck.wheel.angular_velocity";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_steering: &str =
    "truck.wheel.steering";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_rotation: &str =
    "truck.wheel.rotation";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_lift: &str =
    "truck.wheel.lift";
pub const SCS_TELEMETRY_TRUCK_CHANNEL_wheel_lift_offset: &str =
    "truck.wheel.lift.offset";

// ---------------------------------------------------------------------------
// Trailer channels (use with index 0..9 for trailer slot)
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_TRAILER_CHANNEL_connected: &str =
    "trailer.connected";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_world_placement: &str =
    "trailer.world.placement";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_local_linear_velocity: &str =
    "trailer.local.velocity.linear";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_local_angular_velocity: &str =
    "trailer.local.velocity.angular";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_local_linear_acceleration: &str =
    "trailer.local.acceleration.linear";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_local_angular_acceleration: &str =
    "trailer.local.acceleration.angular";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wear_chassis: &str =
    "trailer.wear.chassis";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_susp_deflection: &str =
    "trailer.wheel.suspension.deflection";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_on_ground: &str =
    "trailer.wheel.on_ground";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_substance: &str =
    "trailer.wheel.substance";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_velocity: &str =
    "trailer.wheel.angular_velocity";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_steering: &str =
    "trailer.wheel.steering";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_rotation: &str =
    "trailer.wheel.rotation";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_lift: &str =
    "trailer.wheel.lift";
pub const SCS_TELEMETRY_TRAILER_CHANNEL_wheel_lift_offset: &str =
    "trailer.wheel.lift.offset";

// ---------------------------------------------------------------------------
// Job channels
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_JOB_CHANNEL_cargo_damage: &str =
    "job.cargo.damage";
