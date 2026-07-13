use crate::mmu::MMU;
use crate::registers::{ByteRegister, Flag, Registers, WordRegister};

// TODO o nome correto é Timer ou Clock?
#[derive(Debug)]
struct Timer {
    // TODO o que significa o m e o t?
    m: u8,
    t: u8,
}

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub mmu: MMU,
    timer: Timer,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            mmu: MMU::new(),
            timer: Timer { m: 0, t: 0 },
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let b = self.mmu.read_byte(self.registers.pc);
        self.registers.pc += 1;
        return b;
    }

    fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch_byte() as u16;
        let hi = self.fetch_byte() as u16;
        return (hi << 8) | lo;
    }

    fn xor(&mut self, value: u8) {
        let xor = self.registers.a ^ value;
        self.registers.a = xor;
        self.registers.clear_flags();
        if xor == 0 {
            self.registers.set_flag(Flag::Z);
        }
    }

    fn jrcc(&mut self, flag: Flag, condition: bool, offset: u8) {
        let flag = self.registers.get_flag(flag);
        if flag == condition {
            self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i8 as i16);
        }
    }

    pub fn step(&mut self) {
        let op = self.fetch_byte();

        // TODO 0xF8 is signed!
        // TODO 0xE8 is signed!
        // TODO 0x18 is signed!
        match op {
            0x06 => self.registers.b = self.fetch_byte(),
            0x0E => self.registers.c = self.fetch_byte(),
            0x16 => self.registers.d = self.fetch_byte(),
            0x1E => self.registers.e = self.fetch_byte(),
            0x26 => self.registers.h = self.fetch_byte(),
            0x2E => self.registers.l = self.fetch_byte(),
            0x3E => self.registers.a = self.fetch_byte(),
            0x20 => {
                let offset = self.fetch_byte();
                self.jrcc(Flag::Z, false, offset)
            }
            0x28 => {
                let offset = self.fetch_byte();
                self.jrcc(Flag::Z, true, offset)
            }
            0x30 => {
                let offset = self.fetch_byte();
                self.jrcc(Flag::C, false, offset)
            }
            0x38 => {
                let offset = self.fetch_byte();
                self.jrcc(Flag::C, true, offset)
            }
            0x01 => {
                let word = self.fetch_word();
                self.registers.set_bc(word);
            }
            0x11 => {
                let word = self.fetch_word();
                self.registers.set_de(word);
            }
            0x21 => {
                let word = self.fetch_word();
                self.registers.set_hl(word);
            }
            0x31 => self.registers.sp = self.fetch_word(),
            0x32 => {
                self.mmu.write_byte(self.registers.hl(), self.registers.a);
                self.registers.dec_w(WordRegister::HL);
            }
            0x1A => {
                let value = self.mmu.read_byte(self.registers.de());
                self.registers.a = value;
            }
            0x4F => self.registers.c = self.registers.a,
            0x77 => self.mmu.write_byte(self.registers.hl(), self.registers.a),
            0x05 => self.registers.dec_b(ByteRegister::B),
            0x0B => self.registers.dec_w(WordRegister::BC),
            0x1B => self.registers.dec_w(WordRegister::DE),
            0x2B => self.registers.dec_w(WordRegister::HL),
            0x3B => self.registers.dec_w(WordRegister::SP),
            0x0C => self.registers.inc_b(ByteRegister::C),
            0xC5 => self.push_w(self.registers.bc()),
            0xCD => {
                let word = self.fetch_word();
                self.call(word);
            }
            0xA8 => self.xor(self.registers.b),
            0xA9 => self.xor(self.registers.c),
            0xAA => self.xor(self.registers.d),
            0xAB => self.xor(self.registers.e),
            0xAC => self.xor(self.registers.h),
            0xAD => self.xor(self.registers.l),
            0xAF => self.xor(self.registers.a),
            0xCB => self.cb(),
            0xE0 => {
                let address = self.fetch_byte() as u16;
                self.mmu
                    .write_word(0xff00 + address, self.registers.a as u16);
            }
            0xE2 => self
                .mmu
                .write_word(0xff00 + self.registers.c as u16, self.registers.a as u16),
            _ => todo!("{:#x}", op),
        }
    }

    fn cb(&mut self) {
        let op = self.fetch_byte();

        match op {
            0x10 => self.registers.b = self.rotate(self.registers.b),
            0x11 => self.registers.c = self.rotate(self.registers.c),
            0x12 => self.registers.d = self.rotate(self.registers.d),
            0x13 => self.registers.e = self.rotate(self.registers.e),
            0x14 => self.registers.h = self.rotate(self.registers.h),
            0x15 => self.registers.l = self.rotate(self.registers.l),
            0x16 => {
                let mut data = self.mmu.read_byte(self.registers.hl());
                data = self.rotate(data);
                self.mmu.write_byte(self.registers.hl(), data);
            }
            0x17 => self.registers.a = self.rotate(self.registers.a),
            0x40 => self.bit(self.registers.b, 0),
            0x41 => self.bit(self.registers.c, 0),
            0x42 => self.bit(self.registers.d, 0),
            0x43 => self.bit(self.registers.e, 0),
            0x44 => self.bit(self.registers.h, 0),
            0x45 => self.bit(self.registers.l, 0),
            0x47 => self.bit(self.registers.a, 0),
            0x48 => self.bit(self.registers.b, 1),
            0x49 => self.bit(self.registers.c, 1),
            0x4A => self.bit(self.registers.d, 1),
            0x4B => self.bit(self.registers.e, 1),
            0x4C => self.bit(self.registers.h, 1),
            0x4D => self.bit(self.registers.l, 1),
            0x4F => self.bit(self.registers.a, 1),
            0x50 => self.bit(self.registers.b, 2),
            0x51 => self.bit(self.registers.c, 2),
            0x52 => self.bit(self.registers.d, 2),
            0x53 => self.bit(self.registers.e, 2),
            0x54 => self.bit(self.registers.h, 2),
            0x55 => self.bit(self.registers.l, 2),
            0x57 => self.bit(self.registers.a, 2),
            0x58 => self.bit(self.registers.b, 3),
            0x59 => self.bit(self.registers.c, 3),
            0x5A => self.bit(self.registers.d, 3),
            0x5B => self.bit(self.registers.e, 3),
            0x5C => self.bit(self.registers.h, 3),
            0x5D => self.bit(self.registers.l, 3),
            0x5F => self.bit(self.registers.a, 3),
            0x60 => self.bit(self.registers.b, 4),
            0x61 => self.bit(self.registers.c, 4),
            0x62 => self.bit(self.registers.d, 4),
            0x63 => self.bit(self.registers.e, 4),
            0x64 => self.bit(self.registers.h, 4),
            0x65 => self.bit(self.registers.l, 4),
            0x67 => self.bit(self.registers.a, 4),
            0x68 => self.bit(self.registers.b, 5),
            0x69 => self.bit(self.registers.c, 5),
            0x6A => self.bit(self.registers.d, 5),
            0x6B => self.bit(self.registers.e, 5),
            0x6C => self.bit(self.registers.h, 5),
            0x6D => self.bit(self.registers.l, 5),
            0x6F => self.bit(self.registers.a, 5),
            0x70 => self.bit(self.registers.b, 6),
            0x71 => self.bit(self.registers.c, 6),
            0x72 => self.bit(self.registers.d, 6),
            0x73 => self.bit(self.registers.e, 6),
            0x74 => self.bit(self.registers.h, 6),
            0x75 => self.bit(self.registers.l, 6),
            0x77 => self.bit(self.registers.a, 6),
            0x78 => self.bit(self.registers.b, 7),
            0x79 => self.bit(self.registers.c, 7),
            0x7A => self.bit(self.registers.d, 7),
            0x7B => self.bit(self.registers.e, 7),
            0x7C => self.bit(self.registers.h, 7),
            0x7D => self.bit(self.registers.l, 7),
            0x7F => self.bit(self.registers.a, 7),
            _ => todo!("{:#x}", op),
        }
    }

    fn call(&mut self, address: u16) {
        self.push_w(self.registers.pc);
        self.registers.pc = address;
    }

    fn push_w(&mut self, value: u16) {
        self.push_b((value >> 8) as u8);
        self.push_b(value as u8);
    }

    fn push_b(&mut self, value: u8) {
        let sp = self.registers.sp;
        self.registers.dec_w(WordRegister::SP);
        self.mmu.write_byte(sp, value);
    }

    fn bit(&mut self, reg: u8, bit: u8) {
        if (reg & (1 << bit)) == 0 {
            self.registers.set_flag(Flag::Z);
        } else {
            self.registers.clear_flag(Flag::Z);
        }
        self.registers.clear_flag(Flag::N);
        self.registers.set_flag(Flag::H);
    }

    fn rotate(&mut self, value: u8) -> u8 {
        let b7_set = (value & (1 << 7)) > 0;

        let mut result = value << 1;
        if b7_set {
            result += 1;
        }

        if result == 0 {
            self.registers.set_flag(Flag::Z);
        } else {
            self.registers.clear_flag(Flag::Z);
        }
        self.registers.clear_flag(Flag::N);
        self.registers.clear_flag(Flag::H);
        if b7_set {
            self.registers.set_flag(Flag::C);
        } else {
            self.registers.clear_flag(Flag::C);
        }

        return result;
    }
}
