use bytemuck::{Pod, Zeroable};
use crate::common::{DPlacement, Euler, FPlacement, FVector};
use crate::consts::{MAX_WHEEL_COUNT, STRING_SIZE};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WheelData {
    pub susp_deflection: f32,
    pub on_ground: u32,
    pub substance: u32,
    pub velocity: f32,
    pub steering: f32,
    pub rotation: f32,
    pub lift: f32,
    pub lift_offset: f32,
}

/// All boolean fields stored as u32 (0/1) to avoid repr(C) padding gaps.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TruckData {
    // placement
    pub world_placement: DPlacement,
    pub local_linear_velocity: FVector,
    pub local_angular_velocity: FVector,
    pub local_linear_acceleration: FVector,
    pub local_angular_acceleration: FVector,
    pub cabin_angular_velocity: FVector,
    pub cabin_angular_acceleration: FVector,
    pub cabin_offset: FPlacement,
    pub head_offset: FPlacement,

    // scalars
    pub speed: f32,
    pub engine_rpm: f32,
    pub engine_gear: i32,
    pub displayed_gear: i32,

    // inputs
    pub input_steering: f32,
    pub input_throttle: f32,
    pub input_brake: f32,
    pub input_clutch: f32,
    pub effective_steering: f32,
    pub effective_throttle: f32,
    pub effective_brake: f32,
    pub effective_clutch: f32,

    pub cruise_control_speed: f32,
    pub retarder_level: u32,
    pub retarder_max: u32,

    pub brake_air_pressure: f32,
    pub brake_temperature: f32,
    pub fuel: f32,
    pub fuel_average_consumption: f32,
    pub fuel_range: f32,
    pub adblue: f32,
    pub adblue_average_consumption: f32,
    pub oil_pressure: f32,
    pub oil_temperature: f32,
    pub water_temperature: f32,
    pub battery_voltage: f32,

    // booleans as u32 — no padding gaps
    pub electric_enabled: u32,
    pub engine_enabled: u32,
    pub wipers: u32,
    pub parking_brake: u32,
    pub motor_brake: u32,
    pub cruise_control: u32,
    pub lblinker: u32,
    pub rblinker: u32,
    pub light_lblinker: u32,
    pub light_rblinker: u32,
    pub light_parking: u32,
    pub light_low_beam: u32,
    pub light_high_beam: u32,
    pub light_beacon: u32,
    pub light_brake: u32,
    pub light_reverse: u32,
    pub battery_voltage_warning: u32,
    pub oil_pressure_warning: u32,
    pub water_temperature_warning: u32,
    pub fuel_warning: u32,
    pub brake_air_pressure_warning: u32,
    pub brake_air_pressure_emergency: u32,
    pub adblue_warning: u32,

    // wear
    pub wear_engine: f32,
    pub wear_transmission: f32,
    pub wear_cabin: f32,
    pub wear_chassis: f32,
    pub wear_wheels: f32,

    pub odometer: f32,

    // navigation
    pub navigation_distance: f32,
    pub navigation_time: f32,
    pub navigation_speed_limit: f32,

    // config strings
    pub truck_id: [u8; STRING_SIZE],
    pub truck_brand_id: [u8; STRING_SIZE],
    pub truck_name: [u8; STRING_SIZE],

    pub wheel_count: u32,

    pub wheels: [WheelData; MAX_WHEEL_COUNT],

    pub forward_gear_count: u32,
    pub reverse_gear_count: u32,

    pub dashboard_backlight: f32,
    pub light_aux_front: u32,
    pub light_aux_roof: u32,
    pub hshift_selector: [u32; 2],

    pub cabin_position: FVector,
    pub head_position: FVector,
    pub hook_position: FVector,
    pub cabin_offset_euler: Euler,

    pub fuel_capacity: f32,
    pub adblue_capacity: f32,
    pub rpm_limit: f32,

    /// Explicit trailing pad to reach 8-byte alignment boundary for DPlacement
    pub _pad: [u8; 4],
}
