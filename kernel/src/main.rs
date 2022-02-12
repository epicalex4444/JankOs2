#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]

mod basic_library;
mod efi_handover;

use basic_library::math::RoundMath;

use basic_library::paging::{
    init_paging
};
use basic_library::print::{
    print,
    print_hex,
    print_dec,
    init_print
};
use efi_handover::efi_bindings::{
    EFI_MEMORY_DESCRIPTOR,
    BootInfo,
};
use efi_handover::gop_functions;

#[no_mangle]
pub extern "C" fn _start(boot_info: *const BootInfo) -> u64 {
    handle_boot_handover(boot_info);

    print_dec(68.round(5) as u32);
    print("\n");

    unsafe {
        for i in 0..(*boot_info).memory_map_size / (*boot_info).descriptor_size {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = ((*boot_info).memory_map as u64 + i * (*boot_info).descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            if (*descriptor).t == 7 {
                print("start = ");
                print_hex((*descriptor).physical_start as u32);
                print(", pages = ");
                print_dec((*descriptor).number_of_pages as u32);
                print("\n");
            }
        }

        if init_paging((*boot_info).memory_map, (*boot_info).memory_map_size, (*boot_info).descriptor_size) {
            panic!("failed to init paging");
        }

        return (*boot_info).memory_map as u64;
    }
}

// Handles the absolutely neccesary setup before anything else can be done.
fn handle_boot_handover(boot_info: *const BootInfo) -> () {
    unsafe {
        gop_functions::gop_init((*boot_info).framebuffer);

        // Set backround to black
        gop_functions::plot_rect(
            0,
            0,
            (*(*boot_info).framebuffer).width,
            (*(*boot_info).framebuffer).height,
            0,
            0,
            0,
            (*boot_info).framebuffer,
        );
        
        init_print((*boot_info).glyphbuffer, (*boot_info).framebuffer, true);

    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        print("\n\nRuntime error encountered at: ");
        print(location.file());
        print(" in line: ");
        print_dec(location.line());
        if let Some(message) = _info.message() {
            if let Some(str_ptr) = message.as_str() {
                print("\nMessage: ");
                print(str_ptr);
            } else {
                if let Some(error) = _info.payload().downcast_ref::<&str>() {
                    print("\n Error: ");
                    print(error);
                } else {
                    print("\n Error");
                }
            }
        } else {
            print("\nNo Message")
        }
    }
    loop {}
}
