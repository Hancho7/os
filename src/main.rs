// src/main.rs
#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

use crate::vga_buffer::print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"This is my first operating system kernel-Hancho!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let greet = "Hello this is another update";
    print(&greet);

    loop {}
}
