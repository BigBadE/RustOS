use core::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};
use core::ptr::NonNull;
use linked_list_allocator::Heap;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::VirtAddr;
use crate::memory::blocks::FixedSizeBlock;

#[global_allocator]
pub static ALLOCATOR: LockedAllocator = LockedAllocator::new();

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub struct LockedAllocator {
    default: Mutex<DefaultAllocator>,
}

struct DefaultAllocator {
    list_heads: [Option<&'static mut FixedSizeBlock>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl DefaultAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut FixedSizeBlock> = Option::None;
        return DefaultAllocator {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: Heap::empty(),
        };
    }

    pub unsafe fn init(&mut self) {
        self.fallback_allocator.init(HEAP_START as *mut u8, HEAP_SIZE);
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        match list_index(&layout) {
            Some(index) => {
                match self.list_heads[index].take() {
                    Some(node) => {
                        self.list_heads[index] = node.next.take();
                        node as *mut FixedSizeBlock as *mut u8
                    }
                    None => {
                        // no block exists in list => allocate new block
                        let block_size = BLOCK_SIZES[index];
                        // only works if all block sizes are a power of 2
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();
                        self.fallback_alloc(layout)
                    }
                }
            }
            None => self.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        match list_index(&layout) {
            Some(index) => {
                let new_node = FixedSizeBlock {
                    next: self.list_heads[index].take(),
                };

                let new_node_ptr = ptr as *mut FixedSizeBlock;
                new_node_ptr.write(new_node);
                self.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                self.fallback_allocator.deallocate(ptr, layout);
            }
        }
    }

    /// Allocates using the fallback allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return self.default.lock().alloc(layout);
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        return self.default.lock().dealloc(ptr, layout);
    }
}

impl LockedAllocator {
    pub const fn new() -> Self {
        return LockedAllocator {
            default: Mutex::new(DefaultAllocator::new())
        };
    }

    pub fn init(&self, mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
        let page_range = {
            let heap_start = VirtAddr::new(HEAP_START as u64);
            let heap_end = heap_start + HEAP_SIZE - 1u64;
            let heap_start_page = Page::containing_address(heap_start);
            let heap_end_page = Page::containing_address(heap_end);
            Page::range_inclusive(heap_start_page, heap_end_page)
        };

        for page in page_range {
            let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
            unsafe {
                mapper.map_to(page, frame, flags, frame_allocator)?.flush()
            };
        }

        unsafe {
            self.default.lock().init();
        }
        Ok(())
    }
}

fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

#[alloc_error_handler]
pub fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("Allocator error!");
}