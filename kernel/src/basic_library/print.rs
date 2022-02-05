use crate::efi_handover::gop_functions;
use crate::efi_handover::efi_bindings;

// Prints the given string, including functionality for '\t' and '\n'
pub fn print(data_ptr: &str, framebuffer: *const efi_bindings::Framebuffer, glyphbuffer: *mut u8) -> (){
    let mut data: *mut u8 = data_ptr.as_ptr() as *mut u8;
    unsafe{
        while *data != 0{
            let c = *data;
            match c as char{
                '\t' => tab(),
                '\n' => newline(),
                _ => place_char(c as u8, framebuffer, glyphbuffer)
            }
            data = data.offset(1);
        }
    }
}

// Prints a character aligned with the character buffer grid
fn place_char(c: u8, framebuffer: *const efi_bindings::Framebuffer, glyphbuffer: *mut u8) -> (){
    let loc = gop_functions::get_cursor();
    let x = (loc%98)*8 + 8;
    let y = (loc/98)*16;
    unsafe{
        gop_functions::jank_put_char(c as u8, x as u32, y as u32, framebuffer, glyphbuffer)
    }
    gop_functions::inc_cursor(1);
}

// Moves cursor to next line
fn newline(){
    let number = 98 - (gop_functions::get_cursor()%98);
    gop_functions::inc_cursor(number);
}

// Moves cursor to nearest denomination of 4
fn tab(){
    let number = 4 - (98 - (gop_functions::get_cursor()%98))%4;
    gop_functions::inc_cursor(number);
}

// Prints the entirety of a u32 as binary
pub fn print_binary(number: u32, framebuffer: *const efi_bindings::Framebuffer, glyphbuffer: *mut u8) -> () {
    for i in (0..31).rev() {
        if number & (1 << i) > 0{
            // Print '1'
            place_char(49, framebuffer, glyphbuffer)
        }
        else{
            // Print '0'
            place_char(48, framebuffer, glyphbuffer)
        }
    }
}

// Prints a u32 as a decimal
pub fn print_dec(number: u32, framebuffer: *const efi_bindings::Framebuffer, glyphbuffer: *mut u8) -> (){
    let magnitude: u32 = number.log10() + 1;
    let mut prev = number;
    for i in (0..magnitude).rev() {
        let current = number%10u32.pow(i);
        let print_num = (prev - current)/10u32.pow(i);
        place_char((print_num + 48) as u8, framebuffer, glyphbuffer);
        prev = current;
    }
}

// Prints a u32 as hex
pub fn print_hex(number: u32, framebuffer: *const efi_bindings::Framebuffer, glyphbuffer: *mut u8) -> (){
    let magnitude: u32 = number.log2()/16u32.log2() + 1;
    place_char('0' as u8, framebuffer, glyphbuffer);
    place_char('x' as u8, framebuffer, glyphbuffer);
    // From left to right of the digits of the number in hex
    for i in (0..magnitude).rev() {
        let quartet = (number & 0b1111 << ((i) * 4 ))/16u32.pow(i);
        if quartet < 10 {
            place_char((quartet + 48) as u8, framebuffer, glyphbuffer);
        }
        else{
            place_char((quartet + 55) as u8, framebuffer, glyphbuffer);
        }
    }
}