//! # Bindings for the C efi structs
//! 
//! [`EFI_MEMORY_DESCRIPTOR`] 
//! 
//! [`Framebuffer`] GOP Framebuffer binding
//! 
//! [`BootInfo`] Boot info struct defined in bootloader

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    pub r#type: u32,
    pub pad: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

//width means resolution width, pixels_per_scan_line is actually how wide the framebuffer is
#[repr(C)]
pub struct Framebuffer {
    pub base_address: *mut u32,
    pub buffer_size: u64,
    pub width: u32,
    pub height: u32,
    pub pixels_per_scan_line: u32,
}

#[repr(C)]
pub struct BootInfo {
    pub frame_buffer: *const Framebuffer,
    pub memory_map: *const EFI_MEMORY_DESCRIPTOR,
    pub memory_map_size: u64,
    pub descriptor_size: u64,
    pub glyph_buffer: *const u8,
}
