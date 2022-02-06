#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    pub r#type: u32,
    pub pad: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64
}

#[repr(C)]
pub struct Framebuffer {
    pub base_address: *mut u32,
    pub buffer_size: u64,
    pub width: u32,
    pub height: u32
}

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: Framebuffer,
    pub memory_map: EFI_MEMORY_DESCRIPTOR,
    pub memory_map_size: u64,
    pub memory_map_descriptor_size: u64,
    pub glyphbuffer: *mut u8
}
