use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use crate::interrupts::pic8259_interrupts;
use crate::println;
use crate::drivers::keyboard;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(crate::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.divide_error.set_handler_fn(divide_by_zero_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.overflow.set_handler_fn(overflow_handler);
    idt.bound_range_exceeded.set_handler_fn(bound_range_handler);
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
    idt.device_not_available.set_handler_fn(device_not_available_handler);
    idt.invalid_tss.set_handler_fn(invalid_tss_handler);
    idt.segment_not_present.set_handler_fn(segment_not_present_handler);
    idt.stack_segment_fault.set_handler_fn(stack_segment_handler);
    idt.general_protection_fault.set_handler_fn(general_protection_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
    idt.alignment_check.set_handler_fn(alignment_check_handler);
    idt.machine_check.set_handler_fn(machine_check_handler);
    idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
    idt.virtualization.set_handler_fn(virtualization_handler);
    idt.vmm_communication_exception.set_handler_fn(vmm_communication_handler);
    idt.security_exception.set_handler_fn(security_handler);
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        return idt;
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = pic8259_interrupts::PIC_1_OFFSET,
    Keyboard = pic8259_interrupts::PIC_1_OFFSET + 1,
    Secondary = pic8259_interrupts::PIC_1_OFFSET + 2,
    SerialPort2 = pic8259_interrupts::PIC_1_OFFSET + 3,
    SerialPort1 = pic8259_interrupts::PIC_1_OFFSET + 4,
    ParallelPort2 = pic8259_interrupts::PIC_1_OFFSET + 5,
    FloppyDisk = pic8259_interrupts::PIC_1_OFFSET + 6,
    ParallelPort1 = pic8259_interrupts::PIC_1_OFFSET + 7,
    RealTimeClock = pic8259_interrupts::PIC_2_OFFSET,
    ACPI = pic8259_interrupts::PIC_2_OFFSET + 1,
    Available1 = pic8259_interrupts::PIC_2_OFFSET + 2,
    Available2 = pic8259_interrupts::PIC_2_OFFSET + 3,
    Mouse = pic8259_interrupts::PIC_2_OFFSET + 4,
    CoProcessor = pic8259_interrupts::PIC_2_OFFSET + 5,
    PrimaryATA = pic8259_interrupts::PIC_2_OFFSET + 6,
    SecondaryATA = pic8259_interrupts::PIC_2_OFFSET + 7,
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

extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DEBUG\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: NON-MASKABLE INTERRUPT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn bound_range_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: INVALID TSS\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("Error code: {}", error_code);
    panic!("EXCEPTION: SEGMENT NOT PRESENT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_segment_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: STACK SEGMENT FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("ERROR CODE: {}", error_code);
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, _error_code: PageFaultErrorCode) {
    panic!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: x87 FLOATING POINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn vmm_communication_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: VMM COMMUNICATION\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn security_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: SECURITY\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    unsafe {
        pic8259_interrupts::PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    keyboard::add_scancode(scancode); // new

    unsafe {
        pic8259_interrupts::PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}