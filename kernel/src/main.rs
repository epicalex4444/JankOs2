#![no_std]
#![no_main]
#![feature(int_log)]

mod basic_library;
mod efi_handover;
mod math;

use basic_library::bitmap;
use basic_library::print;
use efi_handover::efi_bindings;
use efi_handover::gop_functions;

#[no_mangle]
pub extern "C" fn _start(boot_info: efi_bindings::BootInfo) -> u64 {
    handle_boot_handover(&boot_info);

    let mut bmap = bitmap::Bitmap::new();
    print::print_binary(bmap.bits as u32);
    bmap.set_bit(3, true);
    print::print("\n");
    print::print_binary(bmap.bits as u32);
    if bmap.get_bit(3) {
        print::print("\nbit 3 is true")
    }
    bmap.set_bit(3, false);
    print::print("\n");
    print::print_binary(bmap.bits as u32);
    if bmap.get_bit(3) {
        print::print("\nbit 3 is true")
    } else {
        print::print("\nbit 3 is false")
    }
    //return boot_info.glyphbuffer as u64;
    return boot_info.glyphbuffer as u64;
}

// Handles the absolutely neccesary setup before anything else can be done.
fn handle_boot_handover(boot_info: *const efi_bindings::BootInfo) -> () {
    unsafe {
        print::set_max_cursor((((*boot_info).framebuffer.width / 8) * ((*boot_info).framebuffer.height / 16)) as u16);
        print::set_glyphbuffer_ptr((*boot_info).glyphbuffer);
        gop_functions::set_frambuffer_ptr(&(*boot_info).framebuffer);
        
        // Set backroundd to black
        gop_functions::plot_rect(
            0,
            0,
            (*boot_info).framebuffer.width,
            (*boot_info).framebuffer.height,
            0,
            0,
            0,
            &(*boot_info).framebuffer,
        );
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
