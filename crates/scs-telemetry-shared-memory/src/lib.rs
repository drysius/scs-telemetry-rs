#![no_std]

pub mod consts;
pub mod common;
pub mod truck;
pub mod trailer;
pub mod gameplay;
pub mod map;

pub use consts::*;
pub use common::*;
pub use map::TelemetryMap;

// Compile-time layout assertions
const _: () = {
    use core::mem::size_of;
    assert!(size_of::<truck::WheelData>()        == 32);
    assert!(size_of::<truck::TruckData>()        == 1128);
    assert!(size_of::<trailer::TrailerWheelData>() == 32);
    assert!(size_of::<trailer::TrailerData>()    == 768);
    assert!(size_of::<gameplay::JobData>()       == 672);
    assert!(size_of::<TelemetryMap>()            == 9496);
};
