#![no_std]
#![no_main]
#![feature(once_cell, abi_x86_interrupt, panic_info_message)]

use core::panic::PanicInfo;
use log::log;

mod display;
mod memory;
mod interrupts;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.message() {
        Some(message) =>
            println!("Error: {}", message),
        None => println!("Panic detected")
    }

    hlt_loop();
}

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    boot_info.framebuffer.as_mut().unwrap().buffer_mut().fill(0);

    //Init interrupts
    interrupts::init();

    log!(log::Level::Info, "Println!");


    hlt_loop();
}

bootloader_api::entry_point!(kernel_main);

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}