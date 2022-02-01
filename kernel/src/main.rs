#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// TODO:Dingo: Exit boot services
#[no_mangle]
pub extern "C" fn _start() -> i32 {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    1
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
