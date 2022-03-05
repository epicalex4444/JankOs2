#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

mod asm;
mod efi;
mod paging;
mod print;
mod math;
mod gdt;
mod interrupts;

use print::Writer;
use core::arch::asm;
use crate::{gdt::init_gdt, interrupts::init_idt};

#[no_mangle]
pub extern "C" fn _start(boot_info: *const efi::BootInfo) -> ! {
    unsafe {
        Writer::init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);
        
        println!("Hello, World!");

        init_gdt();
        init_idt();

        // Calls interrupt 0x03 - breakpoint
        asm!("INT 0x03");

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
