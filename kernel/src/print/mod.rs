//! # JankOS printing module for low level safe printing using GOP
//! 
//! [`Writer`]
//! 
//! [`print`]
//! 
//! [`println`]

mod gop;

use crate::efi::Framebuffer;
use gop::{plot_pixel, clear_screen, gop_init};
use core::fmt::{self, Write};
use spin::Mutex;

/// # Writer singleton
/// A singleton that acts as the backend to the safe handling of the print! and println! macros
/// **Warning: Should only be initialised once, then left alone. Use the print! and println! macros**
/// 
/// ## Example
/// ```rust
/// Writer.init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);
/// println!("Address: {:#X}", boot_info);
/// ```
/// 
/// **Result:** 
/// "Address: 0x1000" 
pub struct Writer {
    cursor: u32,
    max_cursor: u32,
    line_length: u32,
    lines_count: u32,
    colour: u32,
    columns: bool,
}

// TODO: This raw pointer to the start of the glyph buffer is annoying, we should not keep a mutable raw pointer 
// floating around but safely incorperating it as a mutex or lazy static is not possible with raw pointers
static mut GB_PTR: *const u8 = core::ptr::null_mut();

// Global, thread-safe writer instance.
static WRITER: Mutex<Writer> = Mutex::new(Writer {
    cursor: 0,
    max_cursor: 0,
    line_length: 98,
    lines_count: 37,
    colour: 0x00FFFFFF,
    columns: false,
});

/// # Print
/// Prints formatted text to the screen
/// 
/// ## Arguments
/// * 'string' - the formatable text to be printed to the screen
/// * 'format_args' - the arguments for formatting of the string text
/// 
/// * 'colour' - is an optional argument that must preceede the string arguments and must be seperatedd using a ';'
/// 
/// ```
/// let w = "World!"
/// print!(0x00FF2222; "hello {}", w)
/// ```
/// 
/// Will produce the the statement "hello World!" in a bright red colour
/// 
/// ## Example
/// ```
/// print!("Hello World!");
/// ```
/// 
/// ## Basic formating:
/// 
/// ``` 
/// print!("Value: {}", 20) ;
/// // =>
/// // "Value: 20"
/// ``` 
/// 
/// will print the text and interpolate the subsequent args into the parthesis
/// 
/// ``` 
/// print!("Address: {:#X}", 4096);
/// // =>
/// // "Address: 0x1000"
/// ```
/// 
/// **[further formating](https://doc.rust-lang.org/std/fmt/)**
/// 
/// ## Useful formats:
/// - "{:#X}" prints a number in the form of hex
/// - "{:#b}" prints the number in the form of binary
/// - "{identifier:}, identifier: value" give values inside the formatted text an indentifier
#[macro_export]
macro_rules! print {
    ($c:expr; $($arg:tt)*) => ($crate::print::_print_colour($c, format_args!($($arg)*)));
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// # Println
/// Prints formatted text to the screen and moves to a new line
/// 
/// ## Arguments
/// * 'string' - the formatable text to be printed to the screen
/// * 'format_args' - the arguments for formatting of the string text
/// 
/// ## Example
/// ```
/// println!("Hello World!");
/// ```
/// 
/// ## Basic formating:
/// 
/// ``` 
/// println!("Value: {}", 20);
/// print!("foo");
/// // =>
/// // "Value: 20"
/// // "foo"
/// ``` 
/// 
/// will print the text and interpolate the subsequent args into the parthesis
/// 
/// ``` 
/// println!("Address: {:#X}", 4096) ;
/// // =>
/// // "Address: 0x1000"
/// ```
/// 
/// **[further formating](https://doc.rust-lang.org/std/fmt/)**
/// 
/// ## Useful formats:
/// - "{:#X}" prints a number in the form of hex
/// - "{:#b}" prints the number in the form of binary
/// - "{identifier:}, identifier: value" give values inside the formatted text an indentifier
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($c:expr; $($arg:tt)*) => ($crate::print!($c; "{}\n", format_args!($($arg)*)));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments ){
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _print_colour(c: u32, args: fmt::Arguments){
    let prev_colour = WRITER.lock().colour;
    WRITER.lock().colour = c;
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().colour = prev_colour;
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        return Ok(());
    }
}

impl Writer {
    /// Initialises the writer used in the print! and println! macros
    /// 
    /// ## Arguments
    /// * 'gb_ptr' - glyphbuffer pointer
    /// * 'fb_ptr' - framebuffer pointer
    /// * 'columns' - columns option
    /// 
    /// Sets the values of a static Mutex Writer to fit as many characters in a line as the framebuffer will allow, 
    /// with as many lines as is allowed. Columns is an option that enables two side-by-side columns of lines as opposed
    /// to just a single column of lines. Once this has been called with the correct arguments, the print! and println!
    /// macros become usable
    /// 
    /// ## Examples:
    /// 
    /// ```
    /// Writer.init((*boot_info).glyphbuffer, (*boot_info).framebuffer, false);
    /// println!("Address: {:#X}", boot_info);
    /// 
    /// // Result
    /// // "Address: 0x1000" 
    /// ```
    pub fn init(gb_ptr: *const u8, fb_ptr: *const Framebuffer, columns: bool) -> () {
        unsafe {
            gop_init(fb_ptr);
            clear_screen();

            GB_PTR = gb_ptr;

            let mut len = (*fb_ptr).pixels_per_scan_line / 8 - 2;
            let lin = (*fb_ptr).height / 16;
            let max = if columns {
                len = len / 2 - 1;
                (len * 2) * lin
            } else {
                len * lin
            };

            let mut writer = WRITER.lock();
            writer.max_cursor = max;
            writer.line_length = len;
            writer.lines_count = lin;
            writer.columns = columns;
        }
    }

    fn inc_cursor(&mut self, amount: u32) -> () {
        self.cursor += amount;
    }

    // Prints a character aligned with the character buffer grid
    unsafe fn place_char(&mut self, c: u8) {
        let loc = self.cursor;
        let x;
        let y;
        if self.columns && (loc >= (self.line_length * self.lines_count)) {
            x = (loc % self.line_length) * 8 + ((self.line_length + 1) * 8);
            y = ((loc / self.line_length) * 16) - (self.lines_count * 16);
        } else {
            x = (loc % self.line_length) * 8 + 8;
            y = (loc / self.line_length) * 16;
        }

        let mut font_ptr: *const u8 = GB_PTR.offset(((c as u32) * 16) as isize);
        for i in y..y + 16 {
            for j in x..x + 8 {
                if (*font_ptr & 0b10000000 >> (j - x)) > 0 {
                    plot_pixel(j as u32, i as u32, self.colour)
                }
            }
            font_ptr = font_ptr.offset(1);
        }
        self.inc_cursor(1);
    }

    pub unsafe fn set_colour(&self, rgb: u8){
        static mut COLOUR: u8 = 0;
        COLOUR = rgb;
    }

    // fn get_colour(&self, index: u32) -> u32{
    //     if index < 43 {
    //         return ((index*6%255) << 16) + (255);
    //     }
    //     else if index < 85{
    //         return (255 << 16) + (255 - index*6) % 255;
    //     }
    //     else if index < 128{
    //         return (255 << 16) + ((index*6) % 255 << 8);
    //     }
    //     else if index < 171{
    //         return ((255 - index*6) % 255 << 16) + (255 << 8);
    //     }
    //     else if index < 213{
    //         return (255 << 8) + (index * 6) % 255;
    //     }
    //     else{
    //         return ((255 - index*6)%255 << 8) + 255;
    //     }
    // }

    // Moves cursor to next line
    unsafe fn newline(&mut self) -> () {
        let number = self.line_length - (self.cursor % self.line_length);
        self.inc_cursor(number);
    }

    // Moves cursor to nearest denomination of 4
    unsafe fn tab(&mut self) -> () {
        let number = 4 - (self.line_length - (self.cursor % self.line_length)) % 4;
        self.inc_cursor(number);
    }

    // Prints the given string, including functionality for '\t' and '\n'
    fn print(&mut self, data_ptr: &str) -> () {
        let mut data: *const u8 = data_ptr.as_ptr() as *mut u8;
        let mut amount = data_ptr.len();
        unsafe {
            while amount > 0 {
                let c = *data;
                match c as char {
                    '\t' => self.tab(),
                    '\n' => self.newline(),
                    _ => self.place_char(c),
                }
                amount -= 1;
                data = data.offset(1);
            }
        }
    }
}
