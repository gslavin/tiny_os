#![feature(lang_items)]
#![no_std]

/* Provides a replacement for basic functions of glibc */
extern crate rlibc;

/* This is the function we will call from our asm code,
 * so the name cannot be mangled
 */
#[no_mangle]
pub extern fn rust_main() {
 // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop{}
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

