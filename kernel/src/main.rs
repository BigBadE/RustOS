#![no_std]
#![no_main]
#![feature(once_cell, abi_x86_interrupt, panic_info_message)]

use core::fmt::Write;
use core::panic::PanicInfo;
use bootloader_api::BootloaderConfig;
use bootloader_api::info::{FrameBufferInfo, Optional};
use bootloader_x86_64_common::logger::Logger;
use spinning_top::Spinlock;
use crate::display::writer::serial;
use conquer_once::spin::OnceCell;

mod display;
mod memory;
mod interrupts;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub struct LockedLogger(Spinlock<Logger>);

pub const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.dynamic_range_start = Some(0xffff_8000_0000_0000);
    config
};


impl LockedLogger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        LockedLogger(Spinlock::new(Logger::new(framebuffer, info)))
    }
}

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

    //Setup logger
    match &mut boot_info.framebuffer {
        Optional::Some(buffer) => {
            let info = buffer.info();
            LOGGER.get_or_init(|| { LockedLogger::new(buffer.buffer_mut(), info) });
        }
        Optional::None => panic!("No framebuffer!")
    }

    //Init interrupts
    interrupts::init();

    println!("Going into hlt loop");
    hlt_loop();
}

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}