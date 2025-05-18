/* use windows::Win32::Graphics::Direct3D12::*; */
use windows::Win32::Graphics::Dxgi::Common::*;

pub const NO_AA: DXGI_SAMPLE_DESC = DXGI_SAMPLE_DESC {
    Count: 1,
    Quality: 0,
};

/*
pub const DEFAULT_HEAP: D3D12_HEAP_PROPERTIES = D3D12_HEAP_PROPERTIES {
    Type: D3D12_HEAP_TYPE_DEFAULT,
    // ..Default::default()
    CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
    MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
    CreationNodeMask: 0,
    VisibleNodeMask: 0,
};

pub const UPLOAD_HEAP: D3D12_HEAP_PROPERTIES = D3D12_HEAP_PROPERTIES {
    Type: D3D12_HEAP_TYPE_UPLOAD,
    // ..Default::default()
    CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
    MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
    CreationNodeMask: 0,
    VisibleNodeMask: 0,
};

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
*/
