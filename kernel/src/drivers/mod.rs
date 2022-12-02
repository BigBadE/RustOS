use crate::println;
use crate::threading::executor::Executor;
use crate::threading::task::Task;

pub mod keyboard;

pub fn init() {
    println!("Setting up drivers");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();
}