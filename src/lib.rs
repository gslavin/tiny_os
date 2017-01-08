#![feature(lang_items)]
#![no_std]

/* Provides a replacement for basic functions of glibc */
extern crate rlibc;

/* This is the function we will call from our asm code,
 * so the name cannot be mangled
 */
#[no_mangle]
pub extern fn rust_main() {
    let x = ["Hello", "World", "!"];
    let y = x;
	let test = (0..3).flat_map(|x| 0..x).zip(0..);
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

