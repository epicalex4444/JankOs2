use crate::efi_handover::gop_functions;
use crate::efi_bindings::Framebuffer;

static mut CURSOR: u32 = 0;
static mut MAX_CURSOR: u32 = 0;
static mut GLYPH_PTR: *const u8 = core::ptr::null();
static mut LINE_LENGTH: u32 = 98;
static mut LINES_COUNT: u32 = 37;
static mut COLUMNS: bool = false;

pub unsafe fn init_print(gb_ptr: *const u8, fb_ptr: *const Framebuffer, columns: bool) -> () {
    COLUMNS = columns;
    GLYPH_PTR = gb_ptr;
    LINE_LENGTH = (*fb_ptr).width / 8 - 2;
    LINES_COUNT = (*fb_ptr).height / 16;
    MAX_CURSOR = if COLUMNS {
        LINE_LENGTH  = LINE_LENGTH / 2 - 1;
        (LINE_LENGTH * 2) * LINES_COUNT
    }
    else{
        LINE_LENGTH * LINES_COUNT
    };
}

fn inc_cursor(amount: u32) -> () {
    unsafe {
        CURSOR = CURSOR + amount; //% MAX_CURSOR;
    }
}

// Prints a character aligned with the character buffer grid
#[inline(always)]
unsafe fn place_char(c: u8) -> () {
    let loc = CURSOR;
    let x;
    let y;
    if COLUMNS && (loc >= (LINE_LENGTH * LINES_COUNT)) {
        x = (loc % LINE_LENGTH) * 8 + ((LINE_LENGTH + 1) * 8);
        y = ((loc / LINE_LENGTH) * 16) - (LINES_COUNT * 16);
    }
    else{
        x = (loc % LINE_LENGTH) * 8 + 8;
        y = (loc / LINE_LENGTH) * 16;
    }

    //gop_functions::jank_put_char(c as u8, x as u32, y as u32, glyphbuffer)
    let mut font_ptr: *const u8 = GLYPH_PTR.offset(((c as u32) * 16) as isize);
    for i in y..y + 16 {
        //gop_functions::plot_horizontal_u8(*font_ptr.offset((i - y) as isize), x as u32, i as u32, 0xFFu8, 0xFFu8, 0xFFu8)
        for j in x..x + 8 {
            if (*font_ptr & 0b10000000 >> (j - x)) > 0 {
                gop_functions::plot_pixel(j as u32, i as u32, 0xFFu8, 0xFFu8, 0xFFu8)
            }
        }
        font_ptr = font_ptr.offset(1);
    }
    inc_cursor(1);
}

// Moves cursor to next line
unsafe fn newline() -> () {
    let number = LINE_LENGTH - (CURSOR % LINE_LENGTH);
    inc_cursor(number);
}

// Moves cursor to nearest denomination of 4
unsafe fn tab() -> () {
    let number = 4 - (LINE_LENGTH - (CURSOR % LINE_LENGTH)) % 4;
    inc_cursor(number);
}

// Prints the given string, including functionality for '\t' and '\n'
pub fn print(data_ptr: &str) -> () {
    let mut data: *const u8 = data_ptr.as_ptr() as *mut u8;
    let mut amount = data_ptr.len();
    unsafe {
        while amount > 0 {
            let c = *data;
            match c as char {
                '\t' => tab(),
                '\n' => newline(),
                _ => place_char(c),
            }
            amount -= 1;
            data = data.offset(1);
        }
    }
}

pub fn println(data_ptr: &str) -> () {
    print(data_ptr);
    print("\n");
}

// Prints the entirety of a u32 as binary
pub fn print_binary(number: u32) -> () {
    for i in (0..31).rev() {
        unsafe{
            if number & (1 << i) > 0 {
                // Print '1'
                place_char(49)
            } else {
                // Print '0'
                place_char(48)
            }
        }
    }
}

// Prints a u32 as a decimal
pub fn print_dec(number: u32) -> () {
    let magnitude: u32 = number.log10() + 1;
    let mut prev = number;
    for i in (0..magnitude).rev() {
        let current = number % 10u32.pow(i);
        let print_num = (prev - current) / 10u32.pow(i);
        unsafe{
            place_char((print_num + 48) as u8);
        }

        prev = current;
    }
}

// Prints a u32 as hex
pub fn print_hex(number: u32) -> () {
    let magnitude: u32 = number.log2() / 16u32.log2() + 1;
    unsafe{
        place_char('0' as u8);
        place_char('x' as u8);
    }
    // From left to right of the digits of the number in hex
    for i in (0..magnitude).rev() {
        let quartet = (number & 0b1111 << ((i) * 4)) / 16u32.pow(i);
        unsafe{
            if quartet < 10 {
                place_char((quartet + 48) as u8);
            } else {
                place_char((quartet + 55) as u8);
            }
        }

    }
}
