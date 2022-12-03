use x86_64::instructions::port::Port;

pub struct PCI {}

impl PCI {
    pub fn new() -> Self {
        return PCI {

        }
    }

    pub fn config() {}

    pub fn config_read(&self, bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
        // Bit 31: Flag for determining when CONFIG_DATA should be translated to config cycles.
        // Bits 23-16: Choose a specific PCI bus
        // Bits 15-11: Choose a specific device
        // Bits 10-8: Choose a specific function (if multiple are supported)
        // Bits 7-0: Offset into 256-bit config space, Bits 0 and 1 are always 0
        let mut config_address: Port<u32> = Port::new(0xCF8);
        let mut config_data: Port<u32> = Port::new(0xCFC);

        //Combine the data together, following the format
        let address = ((bus as u32) << 16) | ((slot as u32) << 11) | ((function as u32) << 8) | ((offset as u32) & 0xFC) | 0x80000000;

        unsafe {
            config_address.write(address);

            //Align the bits because we didn't need all 16
            return config_data.read();
        }
    }
}