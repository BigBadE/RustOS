#![no_std]
use core::fmt;
use core::fmt::Write;
use bootloader_api::info::FrameBufferInfo;
use x86_64::instructions::interrupts;
use bootloader_x86_64_common::logger::Logger;
use conquer_once::spin::OnceCell;
use spinning_top::Spinlock;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub struct LockedLogger(Spinlock<Logger>);

impl LockedLogger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        LockedLogger(Spinlock::new(Logger::new(framebuffer, info)))
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (macros::print!("\n"));
    ($($arg:tt)*) => (macros::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        LOGGER.get().unwrap().0.lock().write_fmt(args).unwrap();
        serial().write_fmt(args).expect("Failed serial");
    });
}

pub fn serial() -> uart_16550::SerialPort {
    let mut port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    port.init();
    port
}