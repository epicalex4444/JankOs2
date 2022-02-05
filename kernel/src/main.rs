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
pub extern "C" fn _start(framebuffer:efi_bindings::Framebuffer, descriptor_table:efi_bindings::EFI_MEMORY_DESCRIPTOR, table_size:u64, table_desc_size:u64, glyphBuffer:*mut u8) -> u64 {
    
    gop_functions::set_max_cursor(7154 as u16);
    
    unsafe {
        gop_functions::plot_rect(0, 0, framebuffer.Width, framebuffer.Height, 0, 0, 0, &framebuffer);
    }

    print::print("Glyph buffer: ", &framebuffer, glyphBuffer);
    print::print_hex(glyphBuffer as u32, &framebuffer, glyphBuffer);
    print::print("\nFrame buffer: ", &framebuffer, glyphBuffer);
    unsafe {
        print::print_hex(&framebuffer as *const efi_bindings::Framebuffer as u32, &framebuffer, glyphBuffer);
    }
    return glyphBuffer as u64;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}