use crate::println;

pub mod interrupts;
pub mod pic8259_interrupts;
pub mod gdt;

pub fn init() {
    println!("Loading interrupts");
    // Init the GDT first, then CPU interrupts
    gdt::init();
    interrupts::init_idt();

    // Init hardware interrupts last
    unsafe { pic8259_interrupts::PICS.lock().initialize() };

    // Enable interrupt controller
    x86_64::instructions::interrupts::enable();
}