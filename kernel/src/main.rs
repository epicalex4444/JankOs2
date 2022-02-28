#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]
#![allow(dead_code)]

mod asm;
mod efi;
mod paging;
mod print;
mod math;
mod gdt;

use print::Writer;

use crate::gdt::init_gdt;

#[no_mangle]
pub extern "C" fn _start(boot_info: *const efi::BootInfo) -> u64 {
    unsafe {
        Writer::init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);

        println!("Hello, World!");

        init_gdt();
        

        println!("GoodBye, World!");

        loop {
            asm::hlt();
        };
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop {
        asm::hlt();
    }
}
