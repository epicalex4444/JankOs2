use crate::efi_bindings::Framebuffer;
use crate::math::minimum;

static mut FB_PTR: *const Framebuffer = core::ptr::null();

pub unsafe fn gop_init(fb_ptr: *const Framebuffer) -> () {
    FB_PTR = fb_ptr;
}

#[inline(always)]
pub fn plot_pixel(x:u32, y:u32, r:u8, g:u8, b:u8) -> () {
    unsafe {
        let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);
        let address:*mut u32 = (*FB_PTR).base_address.offset(((*FB_PTR).pixels_per_scan_line * y + x) as isize);
        *(address) = colour;
    }
}

pub unsafe fn plot_rect(x:u32, y:u32, width:u32, height:u32, r:u8, g:u8, b:u8) -> () {
    if x > (*FB_PTR).pixels_per_scan_line || y > (*FB_PTR).height {
        return
    }

    let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);
    let mut offset:u32 = (*FB_PTR).pixels_per_scan_line * y + x;
    let actual_height:u32 = minimum(height, (*FB_PTR).height - y);
    let actual_width:u32 = minimum(width, (*FB_PTR).pixels_per_scan_line - x);

    for _ in y..y + actual_height {
        for _ in x..x + actual_width {
                *((*FB_PTR).base_address.offset(offset as isize)) = colour;
            offset += 1;
        }
        offset += (*FB_PTR).pixels_per_scan_line - actual_width;
    }
}

pub fn clear_screen() -> () {
    unsafe {
        plot_rect(0, 0, (*FB_PTR).width, (*FB_PTR).height, 0, 0, 0);
    }
}
