pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8, // Flags register
    pub h: u8,
    pub l: u8,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    // 16-bit register pair getters
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    // 16-bit register pair setters
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    // Flag register helpers
    pub fn flag_z(&self) -> bool {
        (self.f & 0b1000_0000) != 0
    }

    pub fn flag_n(&self) -> bool {
        (self.f & 0b0100_0000) != 0
    }

    pub fn flag_h(&self) -> bool {
        (self.f & 0b0010_0000) != 0
    }

    pub fn flag_c(&self) -> bool {
        (self.f & 0b0001_0000) != 0
    }

    pub fn set_flag_z(&mut self, value: bool) {
        match value {
            true => self.f |= 0b1000_0000,
            false => self.f &= 0b0111_1111,
        }
    }

    pub fn set_flag_n(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0100_0000,
            false => self.f &= 0b1011_1111,
        }
    }

    pub fn set_flag_h(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0010_0000,
            false => self.f &= 0b1101_1111,
        }
    }

    pub fn set_flag_c(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0001_0000,
            false => self.f &= 0b1110_1111,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_new_registers_are_zero() {
        let regs = Registers::new();
        assert_eq!(regs.a, 0);
        assert_eq!(regs.b, 0);
        assert_eq!(regs.c, 0);
        assert_eq!(regs.d, 0);
        assert_eq!(regs.e, 0);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_get_af_register_pair() {
        let mut regs = Registers::new();
        regs.a = 0x12;
        regs.f = 0x34;
        assert_eq!(regs.af(), 0x1234);
    }

    #[test]
    fn test_set_af_register_pair() {
        let mut regs = Registers::new();
        regs.set_af(0x1234);
        assert_eq!(regs.af(), 0x1230);
    }

    #[test]
    fn test_get_bc_register_pair() {
        let mut regs = Registers::new();
        regs.b = 0x12;
        regs.c = 0x34;
        assert_eq!(regs.bc(), 0x1234);
    }

    #[test]
    fn test_set_bc_register_pair() {
        let mut regs = Registers::new();
        regs.set_bc(0x1234);
        assert_eq!(regs.bc(), 0x1234);
    }
    #[test]
    fn test_get_de_register_pair() {
        let mut regs = Registers::new();
        regs.d = 0x12;
        regs.e = 0x34;
        assert_eq!(regs.de(), 0x1234);
    }

    #[test]
    fn test_set_de_register_pair() {
        let mut regs = Registers::new();
        regs.set_de(0x1234);
        assert_eq!(regs.de(), 0x1234);
    }

    #[test]
    fn test_get_hl_register_pair() {
        let mut regs = Registers::new();
        regs.h = 0x12;
        regs.l = 0x34;
        assert_eq!(regs.hl(), 0x1234);
    }

    #[test]
    fn test_set_hl_register_pair() {
        let mut regs = Registers::new();
        regs.set_hl(0x1234);
        assert_eq!(regs.hl(), 0x1234);
    }

    #[test]
    fn test_flag_c_operations() {
        let mut regs = Registers::new();
        regs.set_flag_c(true);
        assert!(regs.flag_c());
    }

    #[test]
    fn test_flag_h_operations() {
        let mut regs = Registers::new();
        regs.set_flag_h(true);
        assert!(regs.flag_h());
    }

    #[test]
    fn test_flag_n_operations() {
        let mut regs = Registers::new();
        regs.set_flag_n(true);
        assert!(regs.flag_n());
    }

    #[test]
    fn test_flag_z_operations() {
        let mut regs = Registers::new();
        regs.set_flag_z(true);
        assert!(regs.flag_z());
    }

    #[test]
    fn test_multiple_flags() {
        let mut regs = Registers::new();
        assert_eq!(regs.f, 0x0);
        // z, n, h, c
        regs.set_flag_c(true);
        assert_eq!(regs.f, 16);
        regs.set_flag_h(true);
        assert_eq!(regs.f, 48);
        regs.set_flag_n(true);
        assert_eq!(regs.f, 112);
        regs.set_flag_z(true);
        assert_eq!(regs.f, 240);
    }

    #[test]
    fn test_roundtrip_register_pairs() {
        let mut regs = Registers::new();

        regs.set_bc(0x1234);
        assert_eq!(regs.bc(), 0x1234);

        regs.set_de(0x5678);
        assert_eq!(regs.de(), 0x5678);

        regs.set_hl(0x9ABC);
        assert_eq!(regs.hl(), 0x9ABC);
    }
}
