use core::{ops, slice};

pub struct MemoryBlocks {
    pub ptr: *mut MemoryBlock,
    pub len: usize
}

impl MemoryBlocks {
    pub fn empty() -> Self {
        return MemoryBlocks {
            ptr: 0 as *mut MemoryBlock,
            len: 0
        }
    }

    pub fn new(ptr: *mut MemoryBlock, len: usize) -> Self {
        return MemoryBlocks {
            ptr,
            len
        }
    }
}

pub struct MemoryBlock {
    /// The physical start address of the region.
    pub start: u64,
    /// The physical end address (exclusive) of the region.
    pub end: u64,
    /// The memory type of the memory region.
    ///
    /// Only [`Usable`][MemoryRegionKind::Usable] regions can be freely used.
    pub used: bool,
}

impl ops::Deref for MemoryBlocks {
    type Target = [MemoryBlock];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl ops::DerefMut for MemoryBlocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}