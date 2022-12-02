#![feature(panic_info_message)]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
extern crate macros;

/// This function is called on panic.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    match info.message() {
        Some(message) =>
            println!("Error: {}", message),
        None => println!("Panic detected")
    }

    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}