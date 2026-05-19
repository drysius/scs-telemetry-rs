use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct DVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct Euler {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct FPlacement {
    pub position: FVector,
    pub orientation: Euler,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct DPlacement {
    pub position: DVector,
    pub orientation: Euler,
    /// 4 bytes padding to align to 8 bytes after DVector(24) + Euler(12) = 36 → pad to 40
    pub _pad: [u8; 4],
}
