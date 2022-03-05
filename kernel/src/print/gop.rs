//! # Graphics Interchage Protocol
//! 
//! Functions to interact with the GOP [`Framebuffer`] to draw graphics to the screen

use crate::efi::Framebuffer;
use crate::math::minimum;

static mut FRAMEBUFFER_PTR: *const Framebuffer = core::ptr::null();

pub unsafe fn gop_init(fb_ptr: *const Framebuffer) -> () {
    FRAMEBUFFER_PTR = fb_ptr;
}

pub fn plot_pixel(x:u32, y:u32, rgb: u32) -> () {
    unsafe{
        *((*FRAMEBUFFER_PTR).base_address.offset(((*FRAMEBUFFER_PTR).width * y) as isize ).offset((x) as isize)) = rgb;
    }   
}

pub unsafe fn plot_rect(x:u32, y:u32, width:u32, height:u32, r:u8, g:u8, b:u8) -> () {
    if x > (*FRAMEBUFFER_PTR).width || y > (*FRAMEBUFFER_PTR).height {
        return
    }

    let colour:u32 = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
    let mut offset = (*FRAMEBUFFER_PTR).width * y + x;
    let actual_height = minimum(height, (*FRAMEBUFFER_PTR).height - y);
    let actual_width = minimum(width, (*FRAMEBUFFER_PTR).width - x);

    for _ in y..y + actual_height {
        for _ in x..x + actual_width {            
            *((*FRAMEBUFFER_PTR).base_address.offset(offset as isize)) = colour;
            offset += 1;
        }
        offset += (*FRAMEBUFFER_PTR).width - actual_width;
    }
}

pub fn clear_screen() -> () {
    unsafe {
        plot_rect(0, 0, (*FRAMEBUFFER_PTR).width, (*FRAMEBUFFER_PTR).height, 0, 0, 0);
    }
}