use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::mem::size_of;
use core::ptr;
use crate::memory::allocator::ALLOCATOR;

const STACK_SIZE: usize = 0x7C00 - 0x500;

pub struct SavedState {
    pub stack: *mut u8
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
    stack_data: StackData,
    arg: u8,
}

struct StackData {
    rpb: u64
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

        //Save data
        let stack_data = pushall();
        ptr::write(temp as *mut TempData, TempData { stack: pointer, target, arg, stack_data });


        //Send temp data to loaded function
        asm!("mov r15, {}", in(reg) temp);

        //Switch to new stack
        switch_state(stack, load_state as *mut fn());
    }
}

pub unsafe fn safe_return() {
    //Load the temp data
    let mut position: *mut TempData;
    asm!("mov {}, r15", out(reg) position);
    let data = ptr::read(position);
    ALLOCATOR.dealloc(position as *mut u8, Layout::from_size_align_unchecked(size_of::<TempData>(), 8));

    popall(data.stack_data);
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
        asm!("mov {}, r15", out(reg) position);
        let data = ptr::read(position);

        //Call the passed function
        (data.target)(SavedState { stack: data.stack }, data.arg);

        //If it doesn't load the last state itself, do it here
        println!("Didn't manually load last state, loading default");
        let pointer: *mut u8;
        asm!("mov {}, rsp", out(reg) pointer);

        println!("Our stack is now at {:x}", pointer as u64);
        asm!("mov r15, {}", in(reg) position);
        switch_state(data.stack, safe_return as *mut fn());
    }
}

unsafe fn pushall() -> StackData {
    let mut data = StackData { rpb: 0};
    asm!(//"push rax"
    //"push rbx",
    //"push rcx",
    //"push rdx",
    //"push rsi",
    //"push rdi",
    "mov {}, rbp",
    //"push cs",
    //"push ds",
    //"push ss",
    //"push es",
    //"push fs",
    //"push gs",
    out(reg) data.rpb
    );
    return data;
}

unsafe fn popall(data: StackData) {
    asm!( //"pop gs",
    //"pop fs",
    //"pop es",
    //"pop ss",
    //"pop ds",
    //"pop cs",
    "mov rbp, {}",
    //"pop rdi",
    //"pop rsi",
    //"pop rdx",
    //"pop rcx",
    //"pop rbx",
    //"pop rax",
    in(reg) data.rpb
    );
}