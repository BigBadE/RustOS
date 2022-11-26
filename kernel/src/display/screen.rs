use bootloader_api::info::FrameBuffer;
use crate::display::writer::Writer;

pub struct Screen {
    pub framebuffer: &'static mut FrameBuffer
}

impl Screen {
    pub fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        return Screen {
            framebuffer
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.buffer_mut().fill(0);
    }
}