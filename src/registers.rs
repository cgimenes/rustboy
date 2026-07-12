pub enum Flag {
    /*
     * Zero Flag
     * This bit is set when the result of a math operation
     * is zero or two values match when using the CP
     * instruction.
     */
    Z,

    /*
     * Subtract Flag
     * This bit is set if a subtraction was performed in the
     * last math instruction.
     */
    N,

    /*
     * Half Carry Flag
     * This bit is set if a carry occurred from the lower
     * nibble in the last math operation.
     */
    H,

    /*
     * Carry Flag
     * This bit is set if a carry occurred from the last
     * math operation or if register A is the smaller value
     * when executing the CP instruction.
     */
    C,
}

pub enum ByteRegister {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum WordRegister {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8, // accumulator
    pub f: u8, // flags
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16, // stack pointer
    pub pc: u16, // program counter
}

macro_rules! register_pair {
    ($get:ident, $set:ident, $hi:ident, $lo:ident) => {
        pub fn $get(&self) -> u16 {
            ((self.$hi as u16) << 8) | self.$lo as u16
        }

        pub fn $set(&mut self, v: u16) {
            self.$hi = (v >> 8) as u8;
            self.$lo = v as u8;
        }
    };
}

impl Registers {
    register_pair!(af, set_af, a, f);
    register_pair!(bc, set_bc, b, c);
    register_pair!(de, set_de, d, e);
    register_pair!(hl, set_hl, h, l);

    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => (self.f & 0x80) != 0,
            Flag::N => (self.f & 0x40) != 0,
            Flag::H => (self.f & 0x20) != 0,
            Flag::C => (self.f & 0x10) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f |= 0x80,
            Flag::N => self.f |= 0x40,
            Flag::H => self.f |= 0x20,
            Flag::C => self.f |= 0x10,
        }
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f &= !0x80,
            Flag::N => self.f &= !0x40,
            Flag::H => self.f &= !0x20,
            Flag::C => self.f &= !0x10,
        }
    }

    pub fn clear_flags(&mut self) {
        self.f = 0;
    }

    pub fn inc_b(&mut self, register: ByteRegister) {
        let mut value = match register {
            ByteRegister::A => self.a,
            ByteRegister::F => self.f,
            ByteRegister::B => self.b,
            ByteRegister::C => self.c,
            ByteRegister::D => self.d,
            ByteRegister::E => self.e,
            ByteRegister::H => self.h,
            ByteRegister::L => self.l,
        };

        // Set if carry from bit 3
        if value & 0x0f == 0x0f {
            self.set_flag(Flag::H);
        }

        // Handles overflows
        value = value.wrapping_add(1);
        match register {
            ByteRegister::A => self.a = value,
            ByteRegister::F => self.f = value,
            ByteRegister::B => self.b = value,
            ByteRegister::C => self.c = value,
            ByteRegister::D => self.d = value,
            ByteRegister::E => self.e = value,
            ByteRegister::H => self.h = value,
            ByteRegister::L => self.l = value,
        };

        // TODO is an else here necessary?
        if value == 0 {
            self.set_flag(Flag::Z);
        }
        self.clear_flag(Flag::N);
    }

    // TODO improve this
    pub fn inc_w(&mut self, register: WordRegister) {
        let mut value = match register {
            WordRegister::AF => self.af(),
            WordRegister::BC => self.bc(),
            WordRegister::DE => self.de(),
            WordRegister::HL => self.hl(),
            WordRegister::SP => self.sp,
            WordRegister::PC => self.pc,
        };

        // Set if carry from bit 3
        if value & 0x0f == 0x0f {
            self.set_flag(Flag::H);
        }

        // Handles overflows
        value = value.wrapping_add(1);
        match register {
            WordRegister::AF => self.set_af(value),
            WordRegister::BC => self.set_bc(value),
            WordRegister::DE => self.set_de(value),
            WordRegister::HL => self.set_hl(value),
            WordRegister::SP => self.sp = value,
            WordRegister::PC => self.pc = value,
        };

        // TODO is an else here necessary?
        if value == 0 {
            self.set_flag(Flag::Z);
        }
        self.clear_flag(Flag::N);
    }

    pub fn dec_b(&mut self, register: ByteRegister) {
        let mut value = match register {
            ByteRegister::A => self.a,
            ByteRegister::F => self.f,
            ByteRegister::B => self.b,
            ByteRegister::C => self.c,
            ByteRegister::D => self.d,
            ByteRegister::E => self.e,
            ByteRegister::H => self.h,
            ByteRegister::L => self.l,
        };

        // TODO check if I need to change Flag::H
        // TODO what if it's already zero?
        value -= 1;
        match register {
            ByteRegister::A => self.a = value,
            ByteRegister::F => self.f = value,
            ByteRegister::B => self.b = value,
            ByteRegister::C => self.c = value,
            ByteRegister::D => self.d = value,
            ByteRegister::E => self.e = value,
            ByteRegister::H => self.h = value,
            ByteRegister::L => self.l = value,
        };

        if value == 0 {
            self.set_flag(Flag::Z);
        } else {
            self.clear_flag(Flag::Z);
        }
        self.clear_flag(Flag::N);
    }

    pub fn dec_w(&mut self, register: WordRegister) {
        let mut value = match register {
            WordRegister::AF => self.af(),
            WordRegister::BC => self.bc(),
            WordRegister::DE => self.de(),
            WordRegister::HL => self.hl(),
            WordRegister::SP => self.sp,
            WordRegister::PC => self.pc,
        };

        // TODO check if I need to change Flag::H
        // TODO what if it's already zero?
        value -= 1;
        match register {
            WordRegister::AF => self.set_af(value),
            WordRegister::BC => self.set_bc(value),
            WordRegister::DE => self.set_de(value),
            WordRegister::HL => self.set_hl(value),
            WordRegister::SP => self.sp = value,
            WordRegister::PC => self.pc = value,
        };

        if value == 0 {
            self.set_flag(Flag::Z);
        } else {
            self.clear_flag(Flag::Z);
        }
        self.clear_flag(Flag::N);
    }
}
