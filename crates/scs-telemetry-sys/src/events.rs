#![allow(non_upper_case_globals)]

/// Event ID string constants from scssdk_telemetry_event.h
/// and scssdk_telemetry_gameplay_event.h

// ---------------------------------------------------------------------------
// Configuration event IDs
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_CONFIG_substances: &str = "substances";
pub const SCS_TELEMETRY_CONFIG_controls: &str = "controls";
pub const SCS_TELEMETRY_CONFIG_hshifter: &str = "hshifter";
pub const SCS_TELEMETRY_CONFIG_truck: &str = "truck";
pub const SCS_TELEMETRY_CONFIG_trailer: &str = "trailer";
pub const SCS_TELEMETRY_CONFIG_job: &str = "job";

// ---------------------------------------------------------------------------
// Configuration attribute names — truck
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_brand_id: &str = "brand.id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_brand: &str = "brand";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_id: &str = "id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_name: &str = "name";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_fuel_capacity: &str = "fuel.capacity";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_fuel_warning_factor: &str = "fuel.warning.factor";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_adblue_capacity: &str = "adblue.capacity";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_adblue_warning_factor: &str = "adblue.warning.factor";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_air_pressure_warning: &str =
    "brake.air.pressure.warning";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_air_pressure_emergency: &str =
    "brake.air.pressure.emergency";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_oil_pressure_warning: &str = "oil.pressure.warning";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_water_temperature_warning: &str =
    "water.temperature.warning";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_battery_voltage_warning: &str =
    "battery.voltage.warning";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_rpm_limit: &str = "rpm.limit";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_forward_gear_count: &str = "gears.forward";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_reverse_gear_count: &str = "gears.reverse";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_retarder_step_count: &str = "retarder.steps";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_cabin_position: &str = "cabin.position";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_head_position: &str = "head.position";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_hook_position: &str = "hook.position";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_count: &str = "wheels.count";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_position: &str = "wheel.position";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_steerable: &str = "wheel.steerable";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_simulated: &str = "wheel.simulated";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_radius: &str = "wheel.radius";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_powered: &str = "wheel.powered";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_wheel_liftable: &str = "wheel.liftable";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_has_retarder: &str = "has.retarder";

// ---------------------------------------------------------------------------
// Configuration attribute names — trailer
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_accessory_id: &str = "accessory.id";

// ---------------------------------------------------------------------------
// Configuration attribute names — job
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_id: &str = "cargo.id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo: &str = "cargo";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_cargo_mass: &str = "cargo.mass";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_city_id: &str = "destination.city.id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_city: &str = "destination.city";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_company_id: &str =
    "destination.company.id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_destination_company: &str = "destination.company";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_city_id: &str = "source.city.id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_city: &str = "source.city";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_company_id: &str = "source.company.id";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_source_company: &str = "source.company";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_income: &str = "income";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_delivery_time: &str = "delivery.time";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_is_cargo_loaded: &str = "cargo.loaded";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_job_market: &str = "job.market";
pub const SCS_TELEMETRY_CONFIG_ATTRIBUTE_special_job: &str = "job.special";

// ---------------------------------------------------------------------------
// Gameplay event IDs
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_GAMEPLAY_EVENT_job_cancelled: &str = "job.cancelled";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_job_delivered: &str = "job.delivered";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_player_fined: &str = "player.fined";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_player_tollgate_paid: &str = "player.tollgate.paid";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_player_use_ferry: &str = "player.use.ferry";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_player_use_train: &str = "player.use.train";

// ---------------------------------------------------------------------------
// Gameplay event attribute names
// ---------------------------------------------------------------------------

pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_cancel_penalty: &str = "penalty";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_revenue: &str = "revenue";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_earned_xp: &str = "xp.earned";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_cargo_damage: &str = "cargo.damage";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_distance_km: &str = "distance.km";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_delivery_time: &str = "delivery.time";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_auto_park_used: &str = "auto.park.used";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_auto_load_used: &str = "auto.load.used";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_fine_offence: &str = "offence.type";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_fine_amount: &str = "offence.penalty";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_pay_amount: &str = "pay.amount";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_source_name: &str = "source.name";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_target_name: &str = "target.name";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_source_id: &str = "source.id";
pub const SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_target_id: &str = "target.id";
