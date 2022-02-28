#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]
#![allow(dead_code)]

mod asm;
mod bitmap;
mod efi;
mod paging;
mod print;
mod math;

use print::Writer;

#[no_mangle]
pub extern "C" fn _start(boot_info: *const efi::BootInfo) -> u64 {
    unsafe {
        Writer::init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);

        println!("Hello, World!");

        if paging::init_paging((*boot_info).memory_map, (*boot_info).memory_map_size, (*boot_info).descriptor_size) {
            println!("temp function failed");
        }

        for i in 0..(*boot_info).memory_map_size / (*boot_info).descriptor_size {
            let descriptor:*const efi::EFI_MEMORY_DESCRIPTOR = ((*boot_info).memory_map as u64 + i * (*boot_info).descriptor_size) as *const efi::EFI_MEMORY_DESCRIPTOR;
            println!("physical {:#x}, virtual {:#x}, type {}, pages {}, attributes {:#x}", (*descriptor).physical_start, (*descriptor).virtual_start, (*descriptor).r#type, (*descriptor).number_of_pages, (*descriptor).attribute);
        }

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
