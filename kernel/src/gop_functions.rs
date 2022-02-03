use super::efi_bindings::Framebuffer;

pub fn PlotPixel(x:u32, y:u32, r:u8, g:u8, b:u8, framebuffer:Framebuffer) -> () {
    let colour:u32 = (u32::from(r) << 16) + (u32::from(g) << 8) + u32::from(b);    
    unsafe{
        *(framebuffer.BaseAddress.offset((framebuffer.PixelsPerScanLine * y) as isize ).offset((x) as isize)) = colour;
    }   
}
