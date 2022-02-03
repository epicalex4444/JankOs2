#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod efi_bindings;
mod gop_functions;

#[no_mangle]
pub extern "C" fn _start(bootloader: efi_bindings::BootInfo) -> u32 {
    gop_functions::PlotPixel(0, 1, 255, 0, 0, bootloader.framebuffer);
    return bootloader.descriptor_table.Type ;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}