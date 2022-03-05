//! # Graphics Interchage Protocol
//!
//! Functions to interact with the GOP [`Framebuffer`] to draw graphics to the screen

use crate::efi::Framebuffer;
use crate::math::minimum;

//abstract framebuffer to this file
static mut FB_PTR: *const Framebuffer = core::ptr::null();

pub unsafe fn gop_init(fb_ptr: *const Framebuffer) -> () {
    FB_PTR = fb_ptr;
}

//unsafe can write past framebuffer if x and y are too large
#[inline(always)]
pub unsafe fn plot_pixel(x: u32, y: u32, rgb: u32) -> () {
    *((*FB_PTR)
        .base_address
        .offset(((*FB_PTR).pixels_per_scan_line * y + x) as isize)) = rgb;
}

pub fn plot_rect(x: u32, y: u32, width: u32, height: u32, hex: u32) -> () {
    unsafe {
        if x > (*FB_PTR).pixels_per_scan_line || y > (*FB_PTR).height {
            return;
        }

        let mut offset = (*FB_PTR).pixels_per_scan_line * y + x;
        let actual_height = minimum(height, (*FB_PTR).height - y);
        let actual_width = minimum(width, (*FB_PTR).pixels_per_scan_line - x);

        for _ in y..y + actual_height {
            for _ in x..x + actual_width {
                *((*FB_PTR).base_address.offset(offset as isize)) = hex;
                offset += 1;
            }
            offset += (*FB_PTR).width - actual_width;
        }
    }
}

pub fn clear_screen() -> () {
    unsafe {
        plot_rect(0, 0, (*FB_PTR).pixels_per_scan_line, (*FB_PTR).height, 0u32);
    }
}
