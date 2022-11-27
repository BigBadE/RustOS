#[derive(Copy, Clone)]
pub struct MemoryBlock {
    pub size: u64,
    pub next: *mut MemoryBlock
}

impl MemoryBlock {
    pub fn empty() -> Self {
        return MemoryBlock {
            size: 0,
            next: 0 as *mut MemoryBlock
        }
    }

    pub fn new(size: u64, next: *mut MemoryBlock) -> Self {
        return MemoryBlock {
            size,
            next
        }
    }
}