pub struct ATA {
    lba48: bool,
    primary: bool,
    master: bool
}

impl ATA {
    pub fn new(lba48: bool, primary: bool, master: bool) -> Self {
        return ATA {
            lba48,
            primary,
            master
        }
    }
}

impl ATAHandler for ATA {

}

pub trait ATAHandler {

}