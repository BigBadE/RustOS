use bootloader_api::info::{MemoryRegion, MemoryRegionKind, MemoryRegions};
use crate::memory::memory_block::{MemoryBlock};

pub struct Allocator {
    length: u8,
    block: *mut MemoryBlock,
}

unsafe impl Sync for Allocator {}

impl Allocator {
    pub unsafe fn new(memory: &mut MemoryRegions) -> Self {
        let mut block = 0 as *mut MemoryBlock;

        for i in 0..memory.len() {
            let mut region: MemoryRegion = *(memory).get_mut(i).unwrap();
            if region.kind == MemoryRegionKind::Usable {
                let length = region.end - region.start;
                core::ptr::write(region.start as *mut MemoryBlock,
                                 MemoryBlock::new(length, block));
                block = region.start as *mut MemoryBlock;
            }
        }

        return Allocator {
            length: memory.len() as u8,
            block,
        };
    }

    unsafe fn alloc(&self, size: u64) -> *mut u8 {
        let mut block = *self.block;
        let mut last = MemoryBlock::empty();

        // Find a suitable block
        loop {
            if block.size >= size + 8 {
                // Get the block header
                let start = &mut block as *mut MemoryBlock as *mut u64;
                if block.size < size + 8 {
                    // Shrink the block and relocate the header
                    let end = start as u64 + 8 + size as u64;
                    block.size -= size + 8;
                    last.next = end as *mut MemoryBlock;
                    core::ptr::write(last.next, block);
                } else {
                    //Skip the block
                    last.next = block.next;
                }
                //Write size to start of block
                core::ptr::write(start, block.size);
                return (start as u64 + 8) as *mut u8;
            }
            //Loop if there's another
            if block.next as u64 != 0 {
                last = block;
                block = *block.next;
            } else {
                break;
            }
        }

        return 0 as *mut u8;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8) {
        //Find the size of the block
        let start = (_ptr as u64 - 8) as *mut u64;
        //Create the new header and insert it
        let mut block = MemoryBlock::new(core::ptr::read(start), self.block);
        let start = start as *mut MemoryBlock;
        core::ptr::write(start, block);
        //Insert our block before the last one
        block.next = (*self.block).next;
        (*self.block).next = start;
    }
}

fn get_index(length: u64) -> usize {
    if length > 128 {
        return 4;
    } else if length > 64 {
        return 3;
    } else if length > 32 {
        return 2;
    } else if length > 16 {
        return 1;
    }
    return 0;
}