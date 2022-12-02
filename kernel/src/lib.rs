#![no_std]
#![feature(abi_x86_interrupt, panic_info_message, alloc_error_handler, const_mut_refs)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
#[macro_use]
pub extern crate macros as kernel;
pub extern crate alloc;

pub use macros::{print, println, _print};

mod display;
mod drivers;
mod interrupts;
mod threading;
pub mod memory;

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