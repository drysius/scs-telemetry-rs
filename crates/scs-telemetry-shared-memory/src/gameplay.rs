use bytemuck::{Pod, Zeroable};
use crate::consts::STRING_SIZE;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct JobData {
    pub income: u64,
    pub delivery_time: u32,
    pub xp_earned: u32,

    pub cargo_damage: f32,
    pub distance_km: f32,

    pub is_cargo_loaded: u8,
    pub job_market: u8,
    pub special_job: u8,
    pub _pad: u8,

    pub cargo_mass: f32,

    pub cargo_id: [u8; STRING_SIZE],
    pub cargo_name: [u8; STRING_SIZE],
    pub source_city_id: [u8; STRING_SIZE],
    pub source_city: [u8; STRING_SIZE],
    pub source_company_id: [u8; STRING_SIZE],
    pub source_company: [u8; STRING_SIZE],
    pub destination_city_id: [u8; STRING_SIZE],
    pub destination_city: [u8; STRING_SIZE],
    pub destination_company_id: [u8; STRING_SIZE],
    pub destination_company: [u8; STRING_SIZE],
}
