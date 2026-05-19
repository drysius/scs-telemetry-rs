use bytemuck::{Pod, Zeroable};
use crate::common::{DPlacement, FVector};
use crate::consts::{MAX_WHEEL_COUNT, STRING_SIZE};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TrailerWheelData {
    pub susp_deflection: f32,
    pub on_ground: u32,
    pub substance: u32,
    pub velocity: f32,
    pub steering: f32,
    pub rotation: f32,
    pub lift: f32,
    pub lift_offset: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TrailerData {
    /// 1-byte flag padded to 8 bytes to align the following DPlacement
    pub connected: u8,
    pub _connected_pad: [u8; 7],

    pub world_placement: DPlacement,
    pub local_linear_velocity: FVector,
    pub local_angular_velocity: FVector,
    pub local_linear_acceleration: FVector,
    pub local_angular_acceleration: FVector,

    pub wear_chassis: f32,
    pub cargo_damage: f32,

    pub trailer_id: [u8; STRING_SIZE],
    pub trailer_name: [u8; STRING_SIZE],
    pub cargo_accessory_id: [u8; STRING_SIZE],

    pub hook_position: FVector,
    pub wheel_count: u32,
    pub wheels: [TrailerWheelData; MAX_WHEEL_COUNT],

    /// Trailing pad to reach next 8-byte alignment boundary
    pub _pad: [u8; 8],
}
