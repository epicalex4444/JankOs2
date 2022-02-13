#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]
#[allow(dead_code)]

mod basic_library;
mod efi_handover;

use basic_library::print::{
    print,
    print_hex,
    print_dec,
    init_print
};
use efi_handover::efi_bindings::{
    BootInfo
};
use efi_handover::gop_functions::{
    gop_init,
    clear_screen
};
use basic_library::paging::{
    request_page,
    init_paging
};

#[no_mangle]
pub extern "C" fn _start(boot_info: *const BootInfo) -> u64 {
    unsafe {
        gop_init((*boot_info).framebuffer);

        init_print((*boot_info).glyphbuffer, (*boot_info).framebuffer, true);

        if init_paging((*boot_info).memory_map, (*boot_info).memory_map_size, (*boot_info).descriptor_size) {
            panic!("failed to init paging");
        }

        clear_screen();

        let address: u64 = request_page();
        print("requested addresss = ");
        print_hex(address as u32);

        let address_ptr: *mut u64 = address as *mut u64;
        (*address_ptr) = 0xffffffffffffffff;

        loop {};
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
