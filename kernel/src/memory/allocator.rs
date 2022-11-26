use core::alloc::Layout;
use core::ptr::null_mut;
use core::sync::atomic::Ordering::SeqCst;
use bootloader_api::info::{MemoryRegion, MemoryRegionKind, MemoryRegions};
use crate::memory::memory_block::{MemoryBlock, MemoryBlocks};

const ALLOCATOR_TABLE_SIZE: u64 = (core::mem::size_of::<MemoryBlock>() * 100) as u64;

pub struct Allocator {
    length: u8,
    blocks: MemoryBlocks
}

unsafe impl Sync for Allocator {}

impl Allocator {
    pub fn new(memory: &mut MemoryRegions) -> Self {
        let mut blocks = MemoryBlocks::empty();

        //Locate an empty region for our allocator table
        for i in 0..memory.len() {
            let mut region: MemoryRegion = *(memory).get_mut(i).unwrap();
            if region.kind == MemoryRegionKind::Usable && region.end - region.start > ALLOCATOR_TABLE_SIZE+1 {
                blocks.ptr = region.start as *mut MemoryBlock;
                region.start += ALLOCATOR_TABLE_SIZE;
                break;
            }
        }

        if blocks.ptr as i32 == 0 {
            panic!("Empty memory!");
        }

        for i in 0..memory.len() {
            let mut region: MemoryRegion = *(memory).get_mut(i).unwrap();
            if region.kind == MemoryRegionKind::Usable {
                blocks.len += 1;
                let block = (*blocks).get_mut(blocks.len).unwrap();
                block.start = region.start;
                block.end = region.end;
                block.used = false;
            }
        }

        return Allocator {
            length: memory.len() as u8,
            blocks
        }
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > 256 {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        self.arena.get().cast::<u8>().add(allocated)
    }
    unsafe fn dealloc(&self, _ptr: *mut u8) {}
}
