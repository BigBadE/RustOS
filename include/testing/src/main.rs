#![no_std]
#![feature(start)]

use kernel::println;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Testing!");
    0
}