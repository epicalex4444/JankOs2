#![no_std]
#![no_main]

#![feature(int_log)]
#![feature(panic_info_message)]
#![feature(once_cell)]

#![allow(dead_code)]

mod bitmap;
mod asm;
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

#[no_mangle]
pub extern "C" fn _start(boot_info: *const BootInfo) -> u64 {
    unsafe {
        gop_init((*boot_info).framebuffer);
        Writer::init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);
        clear_screen();

        println!("Hello, World!");

        //let memory_map_entries: u64 = (*boot_info).memory_map_size / (*boot_info).descriptor_size;
        //for i in 0..memory_map_entries {
        //    let descriptor: *const EFI_MEMORY_DESCRIPTOR = ((*boot_info).memory_map as u64 + i * (*boot_info).descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
        //    println!("physical {:#x}, virtual {:#x}", (*descriptor).physical_start, (*descriptor).virtual_start);
        //}

        println!("cr0 = {:#x}", asm::read_cr0());
        println!("cr2 = {:#x}", asm::read_cr2());
        println!("cr3 = {:#x}", asm::read_cr3());
        println!("cr4 = {:#x}", asm::read_cr4());
        println!("cr8 = {:#x}", asm::read_cr8());
        println!("efer = {:#x}", asm::read_efer());

        let mut eax:u32 = 0x80000000u32;
        let mut ebx:u32 = 0;
        let mut ecx:u32 = 0;
        let mut edx:u32 = 0;
        asm::cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
        println!("cpuid:");
        println!("eax = {:#x}", eax);
        println!("ebx = {:#x}", ebx);
        println!("ecx = {:#x}", ecx);
        println!("edx = {:#x}", edx);

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
