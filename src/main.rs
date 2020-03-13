#![no_std] // don't link the rust std library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of the function
pub extern "C" fn _start() -> ! {
    // This is the entry point of the program
    // The function is named as _start since
    // the linker looks for the function called
    // `_start` by default

    use core::fmt::Write;
    println!("Hello World{}", "!");
    loop {}
}

// This function is called on panic (Handling unrecoverable errors)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
