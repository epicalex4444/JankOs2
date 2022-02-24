#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]
#![allow(dead_code)]

mod efi_bindings;
mod math;
mod paging;
mod print;

use print::Writer;

use efi_bindings::BootInfo;
use paging::{init_paging, request_page};

#[no_mangle]
pub extern "C" fn _start(boot_info: *const BootInfo) -> u64 {
    unsafe {
        if init_paging(
            (*boot_info).memory_map,
            (*boot_info).memory_map_size,
            (*boot_info).descriptor_size,
        ) {
            panic!("failed to init paging");
        }
        Writer::init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);

        let address: u64 = request_page();
        println!("requested address = {:#X}", address);

        let address_ptr: *mut u64 = address as *mut u64;
        (*address_ptr) = 0xffffffffffffffff;

        loop {}
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
