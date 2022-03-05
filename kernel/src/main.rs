#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

mod asm;
mod efi;
mod gdt;
mod math;
mod paging;
mod print;
mod interrupts;

use print::Writer;
use core::arch::asm;
use crate::{gdt::init_gdt, interrupts::init_idt};

#[no_mangle]
pub extern "C" fn _start(boot_info: *const efi::BootInfo) -> ! {
    unsafe {
        Writer::init((*boot_info).glyph_buffer, (*boot_info).frame_buffer, false);
        
        println!("Hello, World!");

        //paging::init_paging((*boot_info).memory_map, (*boot_info).memory_map_size, (*boot_info).descriptor_size);
        init_gdt();
        init_idt();

        // Calls interrupt 0x03 - breakpoint
        asm!("INT 0x03");

        println!("GoodBye, World!");

        loop {
            asm::hlt();
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop {
        asm::hlt();
    }
}
