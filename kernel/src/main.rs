#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]

mod basic_library;
mod efi_handover;
mod math;

use basic_library::paging;
use basic_library::print;
use efi_handover::efi_bindings;
use efi_handover::gop_functions;
use efi_bindings::EFI_MEMORY_DESCRIPTOR;

#[no_mangle]
pub extern "C" fn _start(boot_info: *const efi_bindings::BootInfo) -> u64 {
    handle_boot_handover(boot_info);

    unsafe {
        return (*boot_info).memory_map as u64;
    }
}

// Handles the absolutely neccesary setup before anything else can be done.
fn handle_boot_handover(boot_info: *const efi_bindings::BootInfo) -> () {
    unsafe {
        gop_functions::gop_init((*boot_info).framebuffer);                
        // Set backroundd to black
        //gop_functions::plot_rect(
        //    0,
        //    0,
        //    (*(*boot_info).framebuffer).width,
        //    (*(*boot_info).framebuffer).height,
        //    0,
        //    0,
        //    0,
        //    (*boot_info).framebuffer,
        //);
        
        print::init_print((*boot_info).glyphbuffer, (*boot_info).framebuffer, true);

    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        print::print("Runtime error encountered at: ");
        print::print(location.file());
        print::print(" in line: ");
        print::print_dec(location.line());
        if let Some(message) = _info.message() {
            if let Some(str_ptr) = message.as_str() {
                print::print("\nMessage: ");
                print::print(str_ptr);
            } else {
                if let Some(error) = _info.payload().downcast_ref::<&str>() {
                    print::print("\n Error: ");
                    print::print(error);
                } else {
                    print::print("\n Error");
                }
            }
        } else {
            print::print("\nNo Message")
        }
    }
    loop {}
}
