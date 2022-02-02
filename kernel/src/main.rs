#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod efi_bindings;

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
    pub Height: u32,
    pub PixelsPerScanLine: u32
}

#[repr(C)]
pub struct BootInfo {
    // TODO: Either implament the gop table manually or get a dependancy like bindgen to do it for us
    //gop: efi_bindings::EFI_GRAPHICS_OUTPUT_PROTOCOL,
    pub framebuffer: *mut Framebuffer,
	pub descriptor_table: *mut EFI_MEMORY_DESCRIPTOR,
	pub table_size: u64,
	pub table_desc_size: u64
}

fn PlotPixel(x:u32, y:u32, r:u8, g:u8, b:u8, framebuffer: *const Framebuffer) -> () {
    let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);
    
    unsafe{
        let buffer = &*framebuffer;
        *((*buffer).BaseAddress.offset((4 * (*buffer).PixelsPerScanLine * y).try_into().unwrap() ).offset((4 * x).try_into().unwrap())) = colour;
    }
   
}

#[no_mangle]
pub extern "C" fn _start(bootloader: *mut BootInfo) -> u32 {
        unsafe{
        let info = &*bootloader;
        let framebuf_p = &*(*info).framebuffer;
        PlotPixel(0, 1, 255, 0, 0, framebuf_p);
        return (*(*info).descriptor_table).Type;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}