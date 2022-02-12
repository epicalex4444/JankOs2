#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]

mod basic_library;
mod efi_handover;
mod math;

use basic_library::paging::{
    init_paging
};
use basic_library::print::{
    print,
    print_hex,
    init_print
};
use efi_handover::efi_bindings::{
    EFI_MEMORY_DESCRIPTOR,
    BootInfo,
    Framebuffer
};
use efi_handover::gop_functions;

#[no_mangle]
<<<<<<< HEAD
<<<<<<< HEAD
pub extern "C" fn _start(boot_info: efi_bindings::BootInfo) -> u64 {
    handle_boot_handover(&boot_info);
    
    print::print("Memory map size: ");
    print::print_dec(boot_info.memory_map_size as u32);
    print::print("\nDescriptor size: ");
    print::print_dec(boot_info.memory_map_descriptor_size as u32);
    print::print("\nCount: ");
    print::print_dec((boot_info.memory_map_size / boot_info.memory_map_descriptor_size) as u32);
    unsafe{
        let mut res = 0;
        let mut oem = 0;
        // for i in 0..(boot_info.memory_map_size / boot_info.memory_map_descriptor_size){
        //     //let descriptor: *const efi_bindings::EFI_MEMORY_DESCRIPTOR = (&boot_info.memory_map as *const efi_bindings::EFI_MEMORY_DESCRIPTOR).offset(i);
        //     let descriptor: *const efi_bindings::EFI_MEMORY_DESCRIPTOR = (((&boot_info.memory_map as *const efi_bindings::EFI_MEMORY_DESCRIPTOR) as u64) + (i * boot_info.memory_map_descriptor_size)) as *const efi_bindings::EFI_MEMORY_DESCRIPTOR;
        //     if (*descriptor).r#type <= 14{
        //         print::print("\n");
        //         print::print_dec(i as u32);
        //         print::print(": ");
        //         print::print_hex((*descriptor).r#type as u32);
        //     }
        //     else if (*descriptor).r#type <= 0x6FFFFFFF {
        //         res += 1;
        //     }
        //     else{
        //         oem += 1;
        //     }
        // }
=======
pub extern "C" fn _start(boot_info: *const efi_bindings::BootInfo) -> u64 {
=======
pub extern "C" fn _start(boot_info: *const BootInfo) -> u64 {
>>>>>>> 98f671a42ef1bb6826a3ca73b90142af6ad384ba
    handle_boot_handover(boot_info);
>>>>>>> 262f68b8c93e4714ff2e21626539f4aded490901

    unsafe {
        for i in 0..(*boot_info).memory_map_size / (*boot_info).descriptor_size {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = ((*boot_info).memory_map as u64 + i * (*boot_info).descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            if (*descriptor).t == 7 {
                print("start = ");
                print_hex((*descriptor).physical_start as u32);
                print(", pages = ");
                print_hex((*descriptor).number_of_pages as u32);
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
<<<<<<< HEAD
<<<<<<< HEAD
        gop_functions::gop_init(&(*boot_info).framebuffer);
=======
        gop_functions::gop_init((*boot_info).framebuffer);                
>>>>>>> 262f68b8c93e4714ff2e21626539f4aded490901
        // Set backroundd to black
        //gop_functions::plot_rect(
        //    0,
        //    0,
        //    (*(*boot_info).framebuffer).width,
        //    (*(*boot_info).framebuffer).height,
        //    0,
        //    0,
        //    0,
        //    (*boot_info).framebuffer,
        //);
=======
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
>>>>>>> 98f671a42ef1bb6826a3ca73b90142af6ad384ba
        
        init_print((*boot_info).glyphbuffer, (*boot_info).framebuffer, true);

    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        print("Runtime error encountered at: ");
        print(location.file());
        print(" in line: ");
        print_hex(location.line());
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
