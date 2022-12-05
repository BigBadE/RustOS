pub struct ATA {
    lba48: bool
}

impl ATA {
    pub fn new(lba48: bool) -> Self {
        println!("LBA? {}", lba48);
        return ATA {
            lba48
        }
    }
}

impl ATAHandler for ATA {

}

pub trait ATAHandler {

}