use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::mem::size_of;
use core::ptr;
use crate::memory::allocator::ALLOCATOR;

const STACK_SIZE: usize = 0x7C00 - 0x500;

pub struct SavedState {
    pub stack: *mut u8,
}

impl SavedState {
    pub fn load(&self) {}

    pub fn swap(&self, target: *mut SavedState) {}
}

impl Drop for SavedState {
    fn drop(&mut self) {
        unsafe {
            ALLOCATOR.dealloc((self.stack as usize - STACK_SIZE) as *mut u8, Layout::from_size_align_unchecked(STACK_SIZE, 8));
        }
    }
}

struct TempData {
    stack: *mut u8,
    target: fn(SavedState, u8),
    arg: u8,
}

pub fn save_state(target: fn(SavedState, u8), arg: u8) {
    unsafe {
        //Get a new stack
        let stack = (ALLOCATOR.alloc_zeroed(
            Layout::from_size_align_unchecked(STACK_SIZE, 8)) as usize + STACK_SIZE) as *mut u8;

        //Load current stack pointer
        let pointer: *mut u8;
        asm!("mov {}, rsp", out(reg) pointer);

        println!("Our stack is at {:x}", pointer as u64);

        //Push temp data
        let temp = ALLOCATOR.alloc_zeroed(Layout::from_size_align_unchecked(size_of::<TempData>(), 8));
        ptr::write(temp as *mut TempData, TempData { stack: pointer, target, arg });

        //Send temp data to loaded function
        asm!("mov r15, {}", in(reg) temp, options(nomem, nostack, preserves_flags));

        //Switch to new stack
        switch_state(stack, load_state as *mut fn());
    }
}

pub unsafe fn safe_return() {
    //popall();
    println!("Returned");
}

unsafe fn switch_state(stack: *mut u8, target: *mut fn()) {
    unsafe {
        //Directly loading the register to the jump page faults so I just do this. If anyone knows why, I'm all ears
        asm!("mov r14, {}", in(reg) target);
        asm!("mov rsp, {}", in(reg) stack);
        asm!("jmp r14");
    }
}

pub fn load_state() {
    unsafe {
        //Load the temp data
        let mut position: *mut TempData;
        asm!("mov {}, r15", out(reg) position, options(nomem, nostack, preserves_flags));
        let data = ptr::read(position);

        //Call the passed function
        (data.target)(SavedState { stack: data.stack }, data.arg);

        //If it doesn't load the last state itself, do it here
        println!("Didn't manually load last state, loading default");
        let pointer: *mut u8;
        asm!("mov {}, rsp", out(reg) pointer);

        println!("Our stack is now at {:x}", pointer as u64);
        switch_state(data.stack, safe_return as *mut fn());
    }
}

unsafe fn pushall() {
    asm!("push rax"
    //"push rbx",
    //"push rcx",
    //"push rdx",
    //"push rsi",
    //"push rdi",
    //"push rbp",
    //"push cs",
    //"push ds",
    //"push ss",
    //"push es",
    //"push fs",
    //"push gs"
    );
}

unsafe fn popall() {
    asm!(//"pop gs",
    //"pop fs",
    //"pop es",
    //"pop ss",
    //"pop ds",
    //"pop cs",
    //"pop rbp",
    //"pop rdi",
    //"pop rsi",
    //"pop rdx",
    //"pop rcx",
    //"pop rbx",
    "pop rax"
    );
}