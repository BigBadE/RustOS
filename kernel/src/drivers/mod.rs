use alloc::boxed::Box;
use crate::interrupts::interrupts;
use crate::interrupts::interrupts::InterruptIndex;
use crate::println;
use crate::threading::executor::Executor;
use crate::threading::task::Task;

pub mod keyboard;

pub fn init() {
    println!("Setting up drivers");

    unsafe {
        interrupts::INTERRUPTS[InterruptIndex::Keyboard as usize].push(Box::new(keyboard::add_scancode));
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();
}