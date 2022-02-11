#![no_std]
#![no_main]
#![feature(int_log)]
#![feature(panic_info_message)]

mod basic_library;
mod efi_handover;
mod math;

use basic_library::paging;
use basic_library::print;
use efi_handover::efi_bindings;
use efi_handover::gop_functions;
use efi_bindings::EFI_MEMORY_DESCRIPTOR;

#[no_mangle]
pub extern "C" fn _start(boot_info: efi_bindings::BootInfo) -> u64 {
    handle_boot_handover(&boot_info);
    print::print("Framebuffer width: ");
    print::print("\n");
    print::print("Memory map size: ");
    print::print_dec(boot_info.memory_map_size as u32);
    print::print("\nDescriptor size: ");
    print::print_dec(boot_info.memory_map_descriptor_size as u32);
    print::print("\nCount: ");
    print::print_dec((boot_info.memory_map_size / boot_info.memory_map_descriptor_size) as u32);
    unsafe{
        let mut res = 0;
        let mut oem = 0;
        for i in 0..(boot_info.memory_map_size / boot_info.memory_map_descriptor_size){
            
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (&boot_info.memory_map as *const EFI_MEMORY_DESCRIPTOR).offset((i * boot_info.memory_map_descriptor_size) as isize);
            if (*descriptor).t <= 14{
                print::print("\n");
                print::print_dec(i.try_into().unwrap());
                print::print(": ");
                print::print_dec((*descriptor).t);
            }
            else if (*descriptor).t <= 0x6FFFFFFF {
                res += 1;
            }
            else{
                oem += 1;
            }
        }

        print::print("\n Reserved descriptors: ");
        print::print_dec(res);
        print::print("\n OEM Descriptors: ");
        print::print_dec(oem);

        print::print("\nMmap location: ");
        print::print_hex(((&boot_info.memory_map) as *const efi_bindings::EFI_MEMORY_DESCRIPTOR) as u32);
    }
   
    paging::init_paging(&boot_info.memory_map, boot_info.memory_map_size, boot_info.memory_map_descriptor_size);

    return boot_info.memory_map.physical_start as u64;
}

// Handles the absolutely neccesary setup before anything else can be done.
fn handle_boot_handover(boot_info: *const efi_bindings::BootInfo) -> () {
    unsafe {
        gop_functions::gop_init(&(*boot_info).framebuffer);                
        // Set backroundd to black
        gop_functions::plot_rect(
            0,
            0,
            (*boot_info).framebuffer.width,
            (*boot_info).framebuffer.height,
            0,
            0,
            0,
            &(*boot_info).framebuffer,
        );
        
        print::init_print((*boot_info).glyphbuffer, &(*boot_info).framebuffer, true);

    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        print::print("Runtime error encountered at: ");
        print::print(location.file());
        print::print(" in line: ");
        print::print_dec(location.line());
        if let Some(message) = _info.message() {
            if let Some(str_ptr) = message.as_str() {
                print::print("\nMessage: ");
                print::print(str_ptr);
            } else {
                if let Some(error) = _info.payload().downcast_ref::<&str>() {
                    print::print("\n Error: ");
                    print::print(error);
                } else {
                    print::print("\n Error");
                }
            }
        } else {
            print::print("\nNo Message")
        }
    }
    loop {}
}
