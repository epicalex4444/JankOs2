#![no_std]
#![no_main]
#![feature(int_log)]

mod efi_handover;
mod basic_library;
mod math;

use basic_library::print;
use efi_handover::efi_bindings;
use efi_handover::gop_functions;

#[no_mangle]
pub extern "C" fn _start(boot_info:efi_bindings::BootInfo) -> u64 {
    
    gop_functions::set_max_cursor(7154 as u16);
    
    unsafe {
        gop_functions::plot_rect(0, 0, boot_info.framebuffer.width, boot_info.framebuffer.height, 0, 0, 0, &boot_info.framebuffer);
    }

    print::print("Glyph buffer: ", &boot_info.framebuffer, boot_info.glyphbuffer);
    print::print_hex(boot_info.glyphbuffer as u32, &boot_info.framebuffer, boot_info.glyphbuffer);
    print::print("\nFrame buffer: ", &boot_info.framebuffer, boot_info.glyphbuffer);
    print::print_hex(&boot_info.framebuffer as *const efi_bindings::Framebuffer as u32, &boot_info.framebuffer, boot_info.glyphbuffer);
    
    return boot_info.glyphbuffer as u64;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}