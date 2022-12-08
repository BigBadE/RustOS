use core::borrow::BorrowMut;
use crate::threading::helper;
use crate::threading::helper::SavedState;

struct Coroutine {
    state: SavedState
}

impl Coroutine {
    pub fn run(function: fn(Coroutine)) {
        helper::save_state(Coroutine::start, function as *const u8 as u8);
    }

    pub fn start(state: SavedState, function: u8) {
        unsafe {
            (*(function as *const u8 as *const fn(Coroutine)))(Coroutine {
                state
            });
        }
    }

    pub fn call(&mut self) {
        self.yield_coro();
    }

    pub fn yield_coro(&mut self) {
        let pointer = self.state.borrow_mut() as *mut SavedState;
        self.state.swap(pointer);
    }
}