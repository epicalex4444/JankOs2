#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> i32 {
    123
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
