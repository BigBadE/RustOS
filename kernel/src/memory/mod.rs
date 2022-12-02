use bootloader_api::info::MemoryRegions;
use x86_64::VirtAddr;
use crate::{BootInfoFrameAllocator, println};

pub mod allocator;
pub mod paging;

mod blocks;

pub fn init(memory_offset: u64, memory_regions: &'static MemoryRegions) {
    println!("Initializing allocator");
    unsafe {
        let mut pages = paging::init(VirtAddr::new(
            memory_offset));
        let mut frame_allocator = BootInfoFrameAllocator::init(memory_regions);

        allocator::ALLOCATOR.init(&mut pages, &mut frame_allocator).unwrap();
    };
}