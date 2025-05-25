use windows::Win32::Graphics::{Direct3D12::*, Dxgi::Common::*};

pub const NO_AA: DXGI_SAMPLE_DESC = DXGI_SAMPLE_DESC { Count: 1, Quality: 0 };

pub const BASIC_BUFFER_DESC: D3D12_RESOURCE_DESC = D3D12_RESOURCE_DESC {
    Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
    Alignment: 0,
    Width: 0,
    Height: 1,
    DepthOrArraySize: 1,
    MipLevels: 1,
    Format: DXGI_FORMAT_UNKNOWN,
    SampleDesc: NO_AA,
    Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
    Flags: D3D12_RESOURCE_FLAG_NONE,
};

// pub const CUBE_ID: usize = 0;
// pub const FLOOR_ID: usize = 1;
// pub const MIRROR_ID: usize = 2;
pub const INSTANCE_COUNT: usize = 3; // @Hardcode: cube, floor, mirror

#[rustfmt::skip]
pub const QUAD_VERTICES: [f32; 18] = [
    -1.0, 0.0, -1.0,
    -1.0, 0.0,  1.0,
     1.0, 0.0,  1.0,
    -1.0, 0.0, -1.0,
     1.0, 0.0, -1.0,
     1.0, 0.0,  1.0,
];

#[rustfmt::skip]
pub const CUBE_VERTICES: [f32; 24] = [
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
     1.0,  1.0, -1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,
];

/// https://landelare.github.io/2023/02/18/dxr-tutorial.html#mesh-data
///
/// The index buffer has two non-obvious properties that the shaders abuse:
/// - Cube faces are ordered so that their normal vectors are -X, -Y, -Z, +X, +Y, +Z
/// - The first vertex of each triangle is opposite its hypotenuse
///
/// The quad vertices are ordered to make barycentrics contiguous across the entire surface.
pub const CUBE_INDICES: [u16; 36] = [
    4, 6, 0, 2, 0, 6, 0, 1, 4, 5, 4, 1, //
    0, 2, 1, 3, 1, 2, 1, 3, 5, 7, 5, 3, //
    2, 6, 3, 7, 3, 6, 4, 5, 6, 7, 6, 5,
];
