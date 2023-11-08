// A summary of the Gameboy CPU registers can be found here: https://bgb.bircd.org/pandocs.htm#cpuregistersandflags

// The Gameboy CPU has 8-bit registers, but some of them can be combined to form 16-bit registers.
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

// The F register is a special register that contains the flags of the CPU.
// The flags are stored in the upper 4 bits of the F register.
// The BCD Flags (N, H), the Carry Flag C and the Zero Flag Z
enum RegisterFlag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}

impl Registers {
    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_af(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00F0) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}
