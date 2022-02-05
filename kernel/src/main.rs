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
pub extern "C" fn _start(bootInfo:efi_bindings::BootInfo) -> u64 {
    
    gop_functions::set_max_cursor(7154 as u16);
    
    unsafe {
        gop_functions::plot_rect(0, 0, bootInfo.framebuffer.Width, bootInfo.framebuffer.Height, 0, 0, 0, &bootInfo.framebuffer);
    }

    print::print("Glyph buffer: ", &bootInfo.framebuffer, bootInfo.glyphBuffer);
    print::print_hex(bootInfo.glyphBuffer as u32, &bootInfo.framebuffer, bootInfo.glyphBuffer);
    print::print("\nFrame buffer: ", &bootInfo.framebuffer, bootInfo.glyphBuffer);
    unsafe {
        print::print_hex(&bootInfo.framebuffer as *const efi_bindings::Framebuffer as u32, &bootInfo.framebuffer, bootInfo.glyphBuffer);
    }
    return bootInfo.glyphBuffer as u64;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}