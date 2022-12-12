use x86_64::instructions::port::Port;

const PRIMARY_START: u16 = 0x1F0;
const SECONDARY_START: u16 = 0x170;
const PRIMARY_CONTROL_REGISTER: u16 = 0x3F6;
const SECONDARY_CONTROL_REGISTER: u16 = 0x376;

pub struct ATA {
    lba48: bool,
    primary: bool,
    master: bool,
    sectors: u64
}

impl ATA {
    pub fn new(lba48: bool, primary: bool, master: bool, sectors: u64) -> Self {
        return ATA {
            lba48,
            primary,
            master,
            sectors
        }
    }
}

impl ATAHandler for ATA {
    fn read(&self, lba: u64) {
        let start;
        if self.primary {
            start = PRIMARY_START;
        } else {
            start = SECONDARY_START;
        }
        unsafe {
            if self.master {
                Port::<u16>::new(start + 6).write(0x40);
            } else {
                Port::<u16>::new(start + 6).write(0x50);
            }
            Port::<u32>::new(start + 2).write((self.sectors & 0xFFFFFF00) as u32);
        }
    }
}

pub trait ATAHandler {
    fn read(&self, lba: u64) {}
}