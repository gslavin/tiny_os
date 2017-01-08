#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]

#![no_std]

/* Provides a replacement for basic functions of glibc */
extern crate rlibc;
extern crate volatile;

pub mod vga_buffer;

use vga_buffer::*;
use core::ptr::Unique;
use core::fmt::Write;


/* This is the function we will call from our asm code,
 * so the name cannot be mangled
 */
#[no_mangle]
pub extern fn rust_main() {
 // ATTENTION: we have a very small stack and no guard page
	print_something();

    loop{}
}

const VGA_BUFFER: u64 = 0xb8000;

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { Unique::new(VGA_BUFFER as *mut _) },
    };

    writer.write_str("Hello, ");
	write!(writer, "the numbers are {} and {}", 42, 1.0/3.0).expect("Write error!");
}


/* Used for stacking unwinding.
 * Currently no stack unwinding features are supported
 */
#[lang = "eh_personality"]
extern fn eh_personality() {}

/* This is the entry point for a panic
 */
#[lang = "panic_fmt"]
#[no_mangle] 
pub extern fn panic_fmt() -> ! {loop{}}


/* TODO: recompile libcore with panic=abort
 * instead of using overriding function
 */
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

