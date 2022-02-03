#![no_std]
#![no_main]

mod efi_handover;
mod math;

use efi_handover::efi_bindings;
use efi_handover::gop_functions;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start(bootinfo:efi_bindings::BootInfo) -> u32 {
    
    unsafe {
        gop_functions::plot_rect(0, 0, bootinfo.framebuffer.Width, bootinfo.framebuffer.Height, 0, 0, 0, &bootinfo.framebuffer);
        // TEMPORARY, until the glyph buffer pointer issue is resolved
        gop_functions::jank_print(HELLO.as_ptr(), 3, 3, &(bootinfo.framebuffer), bootinfo.glyphBuffer.offset(113127400));
    }
    loop{}
    return bootinfo.glyphBuffer as u32;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}