use crate::devices::pci::PCI;

pub mod pci;
pub mod ata;

pub fn init() {
    println!("Loading devices");

    let pci = PCI::new();
    for i in 0..=255 {
        for j in 0..32 {
            let output = pci.config_read(i, j, 0, 0);
            let second = pci.config_read(i, j, 0, 0x4);
            let third = pci.config_read(i, j, 0, 0x8);
            let last = pci.config_read(i, j, 0, 0xC);
            if output & 0xFFFF != 0xFFFF {
                println!("Vender: {:x}, Device: {:x}, Status: {:x}, Command: {:x}\
    , Class Code: {:x}, Subclass: {:x}, Prog IF: {:x}, Revision ID: {:x}\
    , BIST: {:x}, Header Type: {:x}, Latency Timer: {:x}, Cache Line Size: {:x}",
                         output & 0xFFFF, (output & 0xFFFF0000) >> 16, second & 0xFFFF, (second & 0xFFFF0000) >> 16,
                         third & 0xFF, (third & 0xFF00) >> 8, (third & 0xFF0000) >> 16, (third & 0xFF000000) >> 24,
                         last & 0xFF, (last & 0xFF00) >> 8, (last & 0xFF0000) >> 16, (last & 0xFF000000) >> 24);
            }
        }
    }
}