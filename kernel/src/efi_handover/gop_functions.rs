use super::efi_bindings::Framebuffer;
use crate::math::minimum;

static mut CURSOR: u16 = 0;
static mut MAX_CURSOR: u16 = 0;

// Gets the current CURSOR position for printing
pub fn get_cursor() -> u16{
    unsafe{
        return CURSOR
    }    
}

pub fn inc_cursor(amount: u16) -> (){
    unsafe{
        CURSOR = (CURSOR + amount)%MAX_CURSOR;
    }
}

pub fn set_max_cursor(max: u16) -> (){
    unsafe{
        MAX_CURSOR = max;
    }
    
}

pub fn plot_pixel(x:u32, y:u32, r:u8, g:u8, b:u8, framebuffer:Framebuffer) -> () {
    let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);    
    unsafe{
        *(framebuffer.base_address.offset((framebuffer.width * y) as isize ).offset((x) as isize)) = colour;
    }   
}

pub unsafe fn plot_rect(x:u32, y:u32, width:u32, height:u32, r:u8, g:u8, b:u8, framebuffer: *const Framebuffer) -> () {
    if x > (*framebuffer).width || y > (*framebuffer).height {
        return
    }

    let colour:u32 = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
    let mut offset = (*framebuffer).width * y + x;
    let actual_height = minimum(height, (*framebuffer).height - y);
    let actual_width = minimum(width, (*framebuffer).width - x);

    for _ in y..y + actual_height {
        for _ in x..x + actual_width {
                *((*framebuffer).base_address.offset(offset as isize)) = colour;
            offset += 1;
        }
        offset += (*framebuffer).width - actual_width;
    }
}

pub unsafe fn jank_put_char(chr:u8, x:u32, y:u32, framebuffer:*const Framebuffer, glyphbuffer:*mut u8) -> () {
    let buf = &*framebuffer;
    let pix_ptr:*mut u32 = buf.base_address;
    let mut font_ptr:*mut u8 = glyphbuffer.offset(((chr as u32) * 16) as isize);
    for j in y..y + 16 {
        for i in x..x + 8 {
            if (*font_ptr & (0b10000000 >> (i - x))) > 0 {
                *(pix_ptr.offset((i + (j * buf.width)) as isize)) = 0xFFFFFFFF;
            }
        }
        font_ptr = font_ptr.offset(1);
    }
}

pub unsafe fn jank_print(str:*const u8, mut x:u32, mut y:u32, framebuffer:*const Framebuffer, glyphbuffer:*mut u8) -> () {
    let mut chr:*mut u8 = str as *mut u8;
    while *chr != 0 {
        jank_put_char(*chr, x, y, framebuffer, glyphbuffer);
        x += 8;
        if x + 8 > (*framebuffer).width {
            x = 0;
            y += 16;
        }
        chr = chr.offset(1);
    }
}