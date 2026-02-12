pub struct Mmu {
    memory: [u8; 0x10000], // 64KB flat memory
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            memory: [0; 0x10000],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, value as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let size = data.len().min(0x8000); // Gameboy limits ROM area
        self.memory[0x0000..size].copy_from_slice(&data[0..size]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_mmu_is_zeroes() {
        let mmu = Mmu::new();
        assert_eq!(mmu.read_byte(0x0000), 0);
    }

    #[test]
    fn test_read_write_byte() {
        let mut mmu = Mmu::new();
        mmu.write_byte(0x1000, 0xFF);
        assert_eq!(mmu.read_byte(0x1000), 0xFF);
        assert_eq!(mmu.read_byte(0x1001), 0x0);
    }

    #[test]
    fn test_read_write_word() {
        let mut mmu = Mmu::new();
        mmu.write_word(0x1000, 0xBEEF);
        assert_eq!(mmu.read_word(0x1000), 0xBEEF);
        assert_eq!(mmu.read_word(0x1002), 0x0);
    }

    #[test]
    fn test_write_word_at_boundary() {
        let mut mmu = Mmu::new();
        // Write a word at 0xFFFF — should wrap around via wrapping_add
        mmu.write_word(0xFFFF, 0x1234);
        assert_eq!(mmu.read_byte(0xFFFF), 0x34); // low byte
        assert_eq!(mmu.read_byte(0x0000), 0x12); // high byte wraps to 0x0000
    }

    #[test]
    fn test_load_rom() {
        let mut mmu = Mmu::new();
        let data: Vec<u8> = vec![0x00, 0xC3, 0x50, 0x01, 0xCE, 0xED];
        mmu.load_rom(&data);

        assert_eq!(mmu.read_byte(0x0000), 0x00);
        assert_eq!(mmu.read_byte(0x0001), 0xC3);
        assert_eq!(mmu.read_byte(0x002), 0x50);
        assert_eq!(mmu.read_byte(0x003), 0x01);
        assert_eq!(mmu.read_byte(0x004), 0xCE);
        assert_eq!(mmu.read_byte(0x005), 0xED);
    }

    #[test]
    fn test_load_rom_clamps_to_32kb() {
        let mut mmu = Mmu::new();
        // Create a ROM larger than 32KB
        let rom_data: Vec<u8> = vec![0xFF; 0x10000];
        mmu.load_rom(&rom_data);

        // Data within 0x0000..0x8000 should be loaded
        assert_eq!(mmu.read_byte(0x0000), 0xFF);
        assert_eq!(mmu.read_byte(0x7FFF), 0xFF);
        // Data beyond ROM area should remain zero
        assert_eq!(mmu.read_byte(0x8000), 0x00);
    }
}
