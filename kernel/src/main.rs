#![no_std]
#![no_main]

#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]

#![allow(dead_code)]

mod bitmap;
mod efi_bindings;
mod gop_functions;
mod math;
mod paging;
mod print;
mod rounding;

use print::Writer;
use efi_bindings::BootInfo;
use gop_functions::{
    gop_init,
    clear_screen
};
use paging::{
    request_page,
    init_paging
};

#[no_mangle]
pub extern "C" fn _start(boot_info: *const BootInfo) -> u64 {
    unsafe {
        gop_init((*boot_info).framebuffer);
        Writer::init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);
        clear_screen();

        if init_paging((*boot_info).memory_map, (*boot_info).memory_map_size, (*boot_info).descriptor_size) {
            panic!("failed to init paging");
        }

        let address: u64 = request_page();
        println!("requested address = {:#X}", address);

        loop {};
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
