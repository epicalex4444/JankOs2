mod pic;
pub mod keyboard;

use core::arch::asm;
use pic::ChainedPIC;
use keyboard::ps2::Ps2Controller;
use spin::Mutex;

pub static PIC: Mutex<ChainedPIC> = Mutex::new(ChainedPIC::new());
// make PS2 apart of the PIC struct?
pub static PS2: Mutex<Ps2Controller> = Mutex::new(Ps2Controller::new());

pub fn init_pic() -> () {
    let pic = PIC.lock();
    pic.remap();
    // Enable keyboard interrupt
    pic.set_interrupt_mask(0b11111101, 0b11111111);
}

pub unsafe fn out_b( port: u16, value: u8) -> (){
    asm!("out dx, al", in("dx") port, in("al") value);
}

pub unsafe fn in_b( port: u16) -> u8 {
    let output: u8;
    asm!("in al, dx", out("al") output, in("dx") port);
    return output
}

pub fn wait() -> (){
    unsafe{
        out_b(0, 0x80)
    }
}