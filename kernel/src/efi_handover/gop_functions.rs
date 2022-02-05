use super::efi_bindings::Framebuffer;
use crate::math::minimum;

static mut cursor: u16 = 0;
static mut max_cursor: u16 = 0;

// Gets the current cursor position for printing
pub fn get_cursor() -> u16{
    unsafe{
        return cursor
    }    
}

pub fn inc_cursor(amount: u16) -> (){
    unsafe{
        cursor = (cursor + amount)%max_cursor;
    }
}

pub fn set_max_cursor(max: u16) -> (){
    unsafe{
        max_cursor = max;
    }
    
}

pub fn PlotPixel(x:u32, y:u32, r:u8, g:u8, b:u8, framebuffer:Framebuffer) -> () {
    let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);    
    unsafe{
        *(framebuffer.BaseAddress.offset((framebuffer.Width * y) as isize ).offset((x) as isize)) = colour;
    }   
}

pub unsafe fn plot_rect(x:u32, y:u32, width:u32, height:u32, r:u8, g:u8, b:u8, framebuffer: *const Framebuffer) -> () {
    if x > (*framebuffer).Width || y > (*framebuffer).Height {
        return
    }

    let colour:u32 = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
    let mut offset = (*framebuffer).Width * y + x;
    let actualHeight = minimum(height, (*framebuffer).Height - y);
    let actualWidth = minimum(width, (*framebuffer).Width - x);

    for _ in y..y + actualHeight {
        for _ in x..x + actualWidth {
                *((*framebuffer).BaseAddress.offset(offset as isize)) = colour;
            offset += 1;
        }
        offset += (*framebuffer).Width - actualWidth;
    }
}

pub unsafe fn jank_put_char(chr:u8, x:u32, y:u32, framebuffer:*const Framebuffer, glyphBuffer:*mut u8) -> () {
    let buf = &*framebuffer;
    let pixPtr:*mut u32 = buf.BaseAddress;
    let mut fontPtr:*mut u8 = glyphBuffer.offset(((chr as u32) * 16) as isize);
    for j in y..y + 16 {
        for i in x..x + 8 {
            if (*fontPtr & (0b10000000 >> (i - x))) > 0 {
                *(pixPtr.offset((i + (j * buf.Width)) as isize)) = 0xFFFFFFFF;
            }
        }
        fontPtr =fontPtr.offset(1);
    }
}

pub unsafe fn jank_print(str:*const u8, mut x:u32, mut y:u32, framebuffer:*const Framebuffer, glyphBuffer:*mut u8) -> () {
    let mut chr:*mut u8 = str as *mut u8;
    while *chr != 0 {
        jank_put_char(*chr, x, y, framebuffer, glyphBuffer);
        x += 8;
        if x + 8 > (*framebuffer).Width {
            x = 0;
            y += 16;
        }
        chr = chr.offset(1);
    }
}