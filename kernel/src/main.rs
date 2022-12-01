#![no_std]
#![no_main]
#![feature(once_cell, abi_x86_interrupt, panic_info_message, alloc_error_handler, const_mut_refs)]

use alloc::boxed::Box;
use core::panic::PanicInfo;
use bootloader_api::BootloaderConfig;
use bootloader_api::config::Mapping;
use bootloader_api::info::{FrameBufferInfo, Optional};
use bootloader_x86_64_common::logger::Logger;
use spinning_top::Spinlock;
use conquer_once::spin::OnceCell;
use x86_64::VirtAddr;
use crate::allocator::LockedAllocator;
use crate::memory::allocator;
use crate::memory::paging::BootInfoFrameAllocator;

mod display;
mod drivers;
mod memory;
mod interrupts;

extern crate alloc;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub struct LockedLogger(Spinlock<Logger>);

pub const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.dynamic_range_start = Some(0xffff_8000_0000_0000);
    config.mappings.physical_memory = Some(Mapping::Dynamic);
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
    //Setup logger
    display::init(unwrap(&mut boot_info.framebuffer));

    //Init interrupts
    interrupts::init();

    //Setup allocator
    memory::init(*unwrap(&mut boot_info.physical_memory_offset), &boot_info.memory_regions);

    println!("Going into hlt loop");
    hlt_loop();
}

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

pub fn unwrap<T>(option: &mut Optional<T>) -> &mut T {
    return match option {
        Optional::Some(value) => value,
        Optional::None => panic!("Failed to unwrap optional")
    };
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}