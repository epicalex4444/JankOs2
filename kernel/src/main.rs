#![no_std]
#![no_main]

mod efi_bindings;
mod gop_functions;

fn Maximum(a:u32, b:u32) -> u32 {
    if a > b {
        return a;
    }
    return b;
}

fn Minimum(a:u32, b:u32) -> u32 {
    if a < b {
        return a;
    }
    return b;
}

fn PlotRect(x:u32, y:u32, width:u32, height:u32, r:u8, g:u8, b:u8, framebuffer:efi_bindings::Framebuffer) -> () {
    if x > framebuffer.Width || y > framebuffer.Height {
        return
    }

    let colour:u32 = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
    let mut offset = framebuffer.Width * y + x;
    let actualHeight = Minimum(height, framebuffer.Height - y);
    let actualWidth = Minimum(width, framebuffer.Width - x);

    for _ in y..y + actualHeight {
        for _ in x..x + actualWidth {
            unsafe {
                *(framebuffer.BaseAddress.offset(offset as isize)) = colour;
            }
            offset += 1;
        }
        offset += framebuffer.Width - actualWidth;
    }
}

unsafe fn JankPutChar(chr:u8, x:u32, y:u32, framebuffer:*const efi_bindings::Framebuffer, glyphBuffer:*mut u8) -> () {
    let pixPtr:*mut u32 = (*framebuffer).BaseAddress;
    let fontPtr:*mut u8 = glyphBuffer.offset((chr * 16) as isize);
    for j in y..y + 16 {
        for i in x..x + 8 {
            if (*fontPtr & (0b10000000 >> (i - x))) > 0 {
                *(pixPtr.offset((i + (j * (*framebuffer).Width)) as isize)) = 0xFFFFFFFF;
            }
        }
        fontPtr.offset(1);
    }
}

unsafe fn JankPrint(str:*const u8, mut x:u32, mut y:u32, framebuffer:*const efi_bindings::Framebuffer, glyphBuffer:*mut u8) -> () {
    let chr:*mut u8 = str as *mut u8;
    while *chr != 0 {
        JankPutChar(*chr, x, y, framebuffer, glyphBuffer);
        x += 8;
        if x + 8 > (*framebuffer).Width {
            x = 0;
            y += 16;
        }
        chr.offset(1);
    }
}

#[no_mangle]
pub extern "C" fn _start(bootinfo:efi_bindings::BootInfo) -> ! {
    PlotRect(100, 100, 400, 300, 255, 0, 0, bootinfo.framebuffer);
    //unsafe {
    //    JankPrint("jank rust hello world!".as_ptr(), 0, 3, &(bootinfo.framebuffer), bootinfo.glyphBuffer);
    //}
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}