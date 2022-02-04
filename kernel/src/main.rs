#![no_std]
#![no_main]

mod efi_handover;
mod math;

use efi_handover::efi_bindings;
use efi_handover::gop_functions;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start(framebuffer:efi_bindings::Framebuffer, descriptor_table:efi_bindings::EFI_MEMORY_DESCRIPTOR, table_size:u64, table_desc_size:u64, glyphBuffer:*mut u8) -> u64 {
    unsafe {
        gop_functions::plot_rect(0, 0, framebuffer.Width, framebuffer.Height, 0, 0, 0, &framebuffer);
        gop_functions::jank_print(HELLO.as_ptr(), 3, 3, &framebuffer, glyphBuffer);
    }
    return glyphBuffer as u64;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}