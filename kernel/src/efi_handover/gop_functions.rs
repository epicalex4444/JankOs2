use super::efi_bindings::Framebuffer;
use crate::math::minimum;

static mut FRAMEBUFFER_PTR: *const Framebuffer = core::ptr::null();

pub unsafe fn gop_init(fb_ptr: *const Framebuffer) -> (){
    FRAMEBUFFER_PTR = fb_ptr;
}

pub fn plot_pixel(x:u32, y:u32, r:u8, g:u8, b:u8) -> () {
    let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);    
    unsafe{
        *((*FRAMEBUFFER_PTR).base_address.offset(((*FRAMEBUFFER_PTR).width * y) as isize ).offset((x) as isize)) = colour;
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