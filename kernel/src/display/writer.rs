use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use log::{Level, log, RecordBuilder};
use spin::Mutex;
use uart_16550::SerialPort;
use crate::display;

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
    //interrupts::without_interrupts(|| {
    display::vga::VGA_WRITER.lock().write_fmt(args).unwrap();
    //});
}