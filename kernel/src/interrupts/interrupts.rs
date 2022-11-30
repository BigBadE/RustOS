use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::interrupts::pic8259_interrupts;
use crate::print;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(crate::interrupts::gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        for i in 0..7 {
            idt[i].set_handler_fn(timer_interrupt_handler);
        }
        for i in 32..(32+16) {
            idt[i].set_handler_fn(timer_interrupt_handler);
        }
        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = pic8259_interrupts::PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe {
        pic8259_interrupts::PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}