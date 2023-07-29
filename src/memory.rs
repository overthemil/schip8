use crate::errors::ChipError;
use crate::MEMORY_SIZE;
use crate::Chip8;

pub const FONT_BASE_ADDRESS: usize = 0x050;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    /// Write a byte of data to the address specified.
    pub fn write(&mut self, address: usize, data: u8) -> Result<(), ChipError> {
        if address >= MEMORY_SIZE {
            return Err(ChipError::AddressOutOfBounds {
                address,
                limit: self.memory.len(),
            });
        }

        self.memory[address] = data;

        Ok(())
    }

    /// Read a byte of data from the address specified.
    pub fn read(&self, address: usize) -> Result<u8, ChipError> {
        if address >= MEMORY_SIZE {
            return Err(ChipError::AddressOutOfBounds {
                address,
                limit: self.memory.len(),
            });
        }

        Ok(self.memory[address])
    }

    /// Write an array of bytes to memory starting at the base address.
    pub fn load(&mut self, base_address: usize, data: &[u8]) -> Result<(), ChipError> {
        let end_address = base_address + data.len();
        if (end_address) >= MEMORY_SIZE {
            return Err(ChipError::AddressOutOfBounds {
                address: end_address,
                limit: self.memory.len(),
            });
        }

        self.memory[base_address..end_address].copy_from_slice(data);

        Ok(())
    }

    /// Write an array of bytes to memory starting at the ROM base address.
    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), ChipError> {
        self.rom = data.to_vec();
        self.load(self.config.rom_base_addr, data)?;

        Ok(())
    }

    /// Write an array of bytes to memory starting at the font base address.
    pub fn load_font(&mut self, font: &[u8]) -> Result<(), ChipError> {
        self.load(FONT_BASE_ADDRESS, font)?;

        Ok(())
    }

    /// Write the default font to memory.
    pub fn load_default_font(&mut self) {
        let _ = self.load_font(&FONT);
    }

    /// Set all values in memory to zero, reload default font and last loaded ROM.
    pub fn reset_memory(&mut self) {
        self.memory = [0; MEMORY_SIZE];
        self.load_default_font();
        let rom_data = self.rom.clone();
        let _ = self.load(self.config.rom_base_addr, &rom_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Chip8;
    use crate::ChipError;

    #[test]
    fn write() {
        let mut c8 = Chip8::default();

        let _ = c8.write(1, 15);
        assert_eq!(c8.memory[1], 15);

        let e = c8.write(123456, 2);
        assert!(matches!(e, Err(ChipError::AddressOutOfBounds { .. })));
    }

    #[test]
    fn read() {
        let mut c8 = Chip8::default();

        let val = c8.read(FONT_BASE_ADDRESS).unwrap();
        assert_eq!(val, 0xF0);
        let val = c8.read(0x200).unwrap();
        assert_eq!(val, 0);

        c8.memory[0x200] = 0xFF;
        let val = c8.read(0x200).unwrap();
        assert_eq!(val, 0xFF);

        let e = c8.read(123456);
        assert!(matches!(e, Err(ChipError::AddressOutOfBounds { .. })));
    }

    #[test]
    fn load() {
        let mut c8 = Chip8::default();

        c8.load(0, &[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(c8.memory[1], 2);
        assert_eq!(c8.memory[3], 4);

        let e = c8.load(4091, &[1, 2, 3, 4, 5]);
        assert!(matches!(e, Err(ChipError::AddressOutOfBounds { .. })));
    }
}
