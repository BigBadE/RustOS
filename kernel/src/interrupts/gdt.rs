use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;
use x86_64::instructions::tables::load_tss;
use x86_64::instructions::segmentation::{CS, Segment};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);

        x86_64::instructions::segmentation::SS::set_reg(SegmentSelector{0: 0});
    }
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.privilege_stack_table[0] = VirtAddr::new(0x7c00);
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };

    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let user_code_selector = gdt.add_entry(Descriptor::user_data_segment());
        let user_data_selector = gdt.add_entry(Descriptor::user_code_segment());
        (gdt, Selectors { code_selector, data_selector, tss_selector, user_code_selector, user_data_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
    user_data_selector: SegmentSelector,
    user_code_selector: SegmentSelector
}