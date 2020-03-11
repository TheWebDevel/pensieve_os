#![no_std] // don't link the rust std library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of the function
pub extern "C" fn _start() -> ! {
    // This is the entry point of the program
    // The function is named as _start since
    // the linker looks for the function called
    // `_start` by default
    loop {}
}

// This function is called on panic (Handling unrecoverable errors)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}