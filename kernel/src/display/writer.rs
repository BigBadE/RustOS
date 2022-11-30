use core::fmt;
use core::fmt::Write;
use log::{log, RecordBuilder};
use x86_64::instructions::interrupts;
use crate::LOGGER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        LOGGER.get().unwrap().0.lock().write_fmt(args).unwrap();
        writeln!(serial(), "Testing!").expect("Failed serial");
    });
}

pub fn serial() -> uart_16550::SerialPort {
    let mut port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    port.init();
    port
}