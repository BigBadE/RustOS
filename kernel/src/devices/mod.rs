use alloc::boxed::Box;
use alloc::vec::Vec;
use anyhow::Error;
use x86_64::instructions::port::Port;
use crate::devices::ata::{ATA, ATAHandler};
use crate::devices::atapi::ATAPI;
use crate::devices::pci::PCI;
use crate::devices::pcidevice::PCIDevice;
use crate::devices::sata::SATA;

pub mod ata;
pub mod atapi;
pub mod pci;
pub mod pcidevice;
pub mod sata;

const PRIMARY_START: u16 = 0x1F0;
const SECONDARY_START: u16 = 0x170;
const PRIMARY_CONTROL_REGISTER: u16 = 0x3F6;
const SECONDARY_CONTROL_REGISTER: u16 = 0x376;

pub struct Devices {
    pub pci: Vec<PCIDevice>,
    pub drives: Vec<Box<dyn ATAHandler>>,
}

impl Devices {
    pub fn new() -> Self {
        return Devices {
            pci: Vec::new(),
            drives: Vec::new(),
        };
    }

    pub fn init(&mut self) {
        println!("Loading devices");

        let pci = PCI::new();
        for i in 0..=255 {
            for j in 0..32 {
                let output = pci.config_read(i, j, 0, 0);
                let second = pci.config_read(i, j, 0, 0x4);
                let third = pci.config_read(i, j, 0, 0x8);
                let last = pci.config_read(i, j, 0, 0xC);
                if output & 0xFFFF != 0xFFFF {
                    self.pci.push(PCIDevice::new());
                    /*println!("Vender: {:x}, Device: {:x}, Status: {:x}, Command: {:x}\
    , Class Code: {:x}, Subclass: {:x}, Prog IF: {:x}, Revision ID: {:x}\
    , BIST: {:x}, Header Type: {:x}, Latency Timer: {:x}, Cache Line Size: {:x}",
                             output & 0xFFFF, (output & 0xFFFF0000) >> 16, second & 0xFFFF, (second & 0xFFFF0000) >> 16,
                             third & 0xFF, (third & 0xFF00) >> 8, (third & 0xFF0000) >> 16, (third & 0xFF000000) >> 24,
                             last & 0xFF, (last & 0xFF00) >> 8, (last & 0xFF0000) >> 16, (last & 0xFF000000) >> 24);*/
                }
            }
        }

        self.detect_ata_mode(true, true);
        self.detect_ata_mode(true, false);
        self.detect_ata_mode(false, true);
        self.detect_ata_mode(false, false);
        println!("PCI devices: {}, Drives: {}", self.pci.len(), self.drives.len());
    }


    pub fn detect_ata_mode(&mut self, primary: bool, master: bool) {
        //Detect start address
        let mut start;
        if primary {
            start = PRIMARY_START;
        } else {
            start = SECONDARY_START;
        }

        unsafe {
            //Select drive
            if master {
                Port::<u16>::new(start + 6).write(0xA0);
            } else {
                Port::<u16>::new(start + 6).write(0xB0);
            }
            for i in 2..=5 {
                //Clear read buffers
                Port::<u16>::new(start + i).write(0);
            }
            //Send IDENTIFY command
            Port::<u16>::new(start + 7).write(0xEC);

            //Get status
            let mut status_address: Port<u8> = Port::new(start + 7);

            let address = status_address.read();

            //Make sure a drive exists
            if address == 0 {
                return;
            }

            //Wait for the busy bit to end
            while status_address.read() & 0x80 != 0 {}

            let mut data_address = Port::<u16>::new(start);
            let mut lba48 = false;
            let mut sectors: u64 = 0;
            let mut found = false;
            for i in 0..256 {
                let data = data_address.read();
                if i == 83 && data & 0x200 != 0 {
                    lba48 = true;
                } else if i == 60 && data != 0 {
                    sectors = data as u64;
                    found = true;
                } else if i == 61 && sectors != 0 {
                    sectors = sectors << 16 + data;
                } else if i == 100 && data != 0 {
                    sectors = i;
                    found = true;
                } else if i == 101 && sectors != 0 {
                    sectors = sectors << 16 + data;
                } else if i == 102 && sectors != 0 {
                    sectors = sectors << 16 + data;
                } else if i == 103 && sectors != 0 {
                    sectors = sectors << 16 + data;
                }
            }

            if !found {
                println!("No sectors found, invalid disk.");
                return;
            }
            println!("Sectors: {}", sectors);
            //Get ATA types
            let ata_values = (Port::<u8>::new(start + 4).read(), Port::<u8>::new(start + 5).read());

            //Detect ATA type
            if ata_values.0 == 0x14 && ata_values.1 == 0x15 {
                self.drives.push(Box::new(ATAPI::new()));
            } else if ata_values.0 == 0x3c && ata_values.1 == 0xc3 {
                self.drives.push(Box::new(SATA::new()));
            } else {
                self.drives.push(Box::new(ATA::new(lba48)));
            }
        }
    }
}