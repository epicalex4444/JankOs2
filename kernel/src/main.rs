#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(once_cell)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![allow(dead_code)]
#![reexport_test_harness_main = "test_main"]


mod asm;
mod efi;
mod gdt;
mod math;
mod paging;
mod print;
mod interrupts;
mod io;

use print::Writer;
use core::arch::asm;
use crate::{gdt::{init_gdt}, interrupts::init_idt};

extern "C"{
    fn set_interrupts() -> ();
}

#[no_mangle]
pub extern "C" fn _start(boot_info: *const efi::BootInfo) -> ! {
    unsafe {
        Writer::init((*boot_info).glyph_buffer, (*boot_info).frame_buffer, false);

        println!("Hello, World!");

        //paging::init_paging((*boot_info).memory_map, (*boot_info).memory_map_size, (*boot_info).descriptor_size);

        init_gdt();
        init_idt();

        #[cfg(test)]
        test_main();

        // Do we want a microkernel? if so this should be a service.
        io::init_pic();
        set_interrupts();

        // Calls interrupt 0x03 - breakpoint
        asm!("INT 0x03");
        
        stack_overflow();

        // Call div by zero interrupt (should not be possible after stack overflow)
        asm!("int 0x0");

        println!("GoodBye, World!");

        loop {
            asm::nop();
            // asm::hlt();
        }
    }
}


// Taken from https://os.phil-opp.com/double-fault-exceptions/#kernel-stack-overflow
#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Kenel panic!");
    println!("{}", _info);
    loop {
        asm::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 0);
    println!("[ok]");
}