#![no_std] // don't link the rust std library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of the function
pub extern "C" fn _start() -> ! {
  // This is the entry point of the program
  // The function is named as _start since
  // the linker looks for the function called
  // `_start` by default

  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }
  loop {}
}

// This function is called on panic (Handling unrecoverable errors)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}