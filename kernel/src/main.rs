#![no_std]
#![no_main]
#![feature(abi_x86_interrupt, panic_info_message, alloc_error_handler, const_mut_refs)]

use core::panic::PanicInfo;
use bootloader_api::BootloaderConfig;
use bootloader_api::config::Mapping;
use bootloader_api::info::Optional;
use crate::memory::paging::BootInfoFrameAllocator;

pub use macros::{print, println};

mod display;
mod drivers;
mod interrupts;
mod threading;
pub mod memory;

#[allow(unused_imports)]
#[macro_use]
pub extern crate macros as kernel;

pub extern crate alloc;

pub const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.dynamic_range_start = Some(0xffff_8000_0000_0000);
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

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

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    //Setup logger
    display::init(unwrap(&mut boot_info.framebuffer));

    //Init interrupts
    interrupts::init();

    //Setup allocator
    memory::init(*unwrap(&mut boot_info.physical_memory_offset), &boot_info.memory_regions);

    //Run drivers
    drivers::init();

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