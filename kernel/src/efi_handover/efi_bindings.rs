#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    pub Type: u32,
    pub Pad: u32,
    pub PhysicalStart: u64,
    pub VirtualStart: u64,
    pub NumberOfPages: u64,
    pub Attribute: u64
}

#[repr(C)]
pub struct Framebuffer {
    pub BaseAddress: *mut u32,
    pub BufferSize: u64,
    pub Width: u32,
    pub Height: u32
}

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: Framebuffer,
	pub descriptor_table: EFI_MEMORY_DESCRIPTOR,
	pub table_size: u64,
	pub table_desc_size: u64,
    pub glyphBuffer: *mut u8
}