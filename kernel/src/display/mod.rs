use bootloader_api::info::FrameBuffer;
use crate::{LockedLogger, LOGGER};

pub mod writer;

pub fn init(buffer: &'static mut FrameBuffer) {
    buffer.buffer_mut().fill(0);
    LOGGER.get_or_init(|| {
        let info = buffer.info();
        LockedLogger::new(buffer.buffer_mut(), info)
    });
}