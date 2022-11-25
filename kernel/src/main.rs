#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    log::info!("Testing!");
    panic!();
}

bootloader_api::entry_point!(kernel_main);

