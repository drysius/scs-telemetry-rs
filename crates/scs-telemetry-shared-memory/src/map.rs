use bytemuck::{Pod, Zeroable};
use crate::consts::{MAX_TRAILER_COUNT, PLUGIN_REVISION};
use crate::truck::TruckData;
use crate::trailer::TrailerData;
use crate::gameplay::JobData;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TelemetryMap {
    /// Plugin SDK revision — always PLUGIN_REVISION (12)
    pub sdk_active: u8,
    pub _sdk_pad: [u8; 3],
    pub plugin_revision: u32,

    /// Game timestamp in milliseconds (paused simulation time)
    pub time: u32,

    /// Simulation paused flag
    pub paused: u8,
    pub _paused_pad: [u8; 3],

    pub truck: TruckData,
    pub job: JobData,
    pub trailers: [TrailerData; MAX_TRAILER_COUNT],
}

impl TelemetryMap {
    pub fn new() -> Self {
        let mut map = Self::zeroed();
        map.plugin_revision = PLUGIN_REVISION;
        map
    }
}
