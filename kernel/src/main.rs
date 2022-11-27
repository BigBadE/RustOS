#![no_std]
#![no_main]
#![feature(once_cell)]

use core::panic::PanicInfo;
use bootloader_api::info::{MemoryRegion, Optional};
use crate::display::screen::Screen;
use crate::memory::allocator::Allocator;
use log::log;

mod display;
mod memory;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log!(info.message().unwrap().as_str().unwrap());
    loop {}
}

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let mut display = match boot_info.framebuffer.as_mut() {
        Some(value) => Screen::new(value),
        _ => panic!()
    };

    let allocator = unsafe { Allocator::new(&mut boot_info.memory_regions) };

    //display.clear();

    let info = display.framebuffer.info();

    //Writer::new(display.framebuffer.buffer_mut(), info).write("Testing 123");

    panic!();
}

bootloader_api::entry_point!(kernel_main);

fn unwrap<T>(optional: Optional<T>) -> T {
    match optional {
        Optional::Some(found) => found,
        Optional::None => panic!()
    }
}