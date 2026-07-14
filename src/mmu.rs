use crate::cartridge::Cartridge;

const VRAM_SIZE: usize = 8 * 1024;
const WRAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 160;
const IO_SIZE: usize = 128;
const HRAM_SIZE: usize = 127;

const ROM_BANKN_START: u16 = 0x4000; // From cartridge, switchable bank via MBC (if any).
const VRAM_START: u16 = 0x8000;
const ERAM_START: u16 = 0xA000;
const WRAM_BANK0_START: u16 = 0xC000;
const WRAM_BANKN_START: u16 = 0xD000; // Only bank 1 in Non-CGB mode. Switchable bank 1~7 in CGB mode.
const ECHO_START: u16 = 0xE000;
const OAM_START: u16 = 0xFE00;
const UNUSABLE_START: u16 = 0xFEA0;
const IO_START: u16 = 0xFF00;
const HRAM_START: u16 = 0xFF80;
const IE: u16 = 0xFFFF;

#[derive(Debug)]
pub struct MMU {
    pub cartridge: Cartridge, // 0x0000 - 0x7FFF (Cartridge ROM)
    vram: [u8; VRAM_SIZE],    // 0x8000 - 0x9FFF
    // 0xA000 - 0xBFFF (Cartridge RAM)
    wram: [u8; WRAM_SIZE], // 0xC000 - 0xDFFF
    // 0xE000 - 0xFDFF (Echo RAM)
    oam: [u8; OAM_SIZE],   // 0xFE00 - 0xFE9F
    io: [u8; IO_SIZE],     // 0xFF00 - 0xFF7F
    hram: [u8; HRAM_SIZE], // 0xFF80 - 0xFFFE
    ie: u8,                // 0xFFFF
}

impl MMU {
    pub fn new() -> Self {
        Self {
            cartridge: Cartridge::Empty,
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let bootrom_enabled = self.io[(0xFF50 - IO_START) as usize] == 0;
        if bootrom_enabled && address <= 0x00FF {
            return BOOTSTRAP_ROM[address as usize];
        } else if address < VRAM_START {
            return self.cartridge.read_rom(address);
        } else if address >= VRAM_START && address < ERAM_START {
            return self.vram[(address - VRAM_START) as usize];
        } else if address >= ERAM_START && address < WRAM_BANK0_START {
            return self.cartridge.read_ram(address - ERAM_START);
        } else if address >= WRAM_BANK0_START && address < ECHO_START {
            return self.wram[(address - WRAM_BANK0_START) as usize];
        } else if address >= ECHO_START && address < OAM_START {
            return self.wram[(address - ECHO_START) as usize]; // TODO check echo ram
        } else if address >= OAM_START && address < UNUSABLE_START {
            return self.oam[(address - OAM_START) as usize];
        } else if address >= IO_START && address < HRAM_START {
            return self.io[(address - IO_START) as usize];
        } else if address >= HRAM_START && address < IE {
            return self.hram[(address - HRAM_START) as usize];
        } else if address == IE {
            return self.ie;
        } else {
            panic!("invalid address: {:#x}", address)
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let hi = self.read_byte(address) as u16;
        let lo = self.read_byte(address + 1) as u16;
        (hi << 8) | lo
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address < VRAM_START {
            panic!("writing to rom: {:#x}", address);
        } else if address >= VRAM_START && address < ERAM_START {
            return self.vram[(address - VRAM_START) as usize] = value;
        } else if address >= ERAM_START && address < WRAM_BANK0_START {
            return self.cartridge.write(address - ERAM_START, value);
        } else if address >= WRAM_BANK0_START && address < ECHO_START {
            return self.wram[(address - WRAM_BANK0_START) as usize] = value;
        } else if address >= ECHO_START && address < OAM_START {
            return self.wram[(address - ECHO_START) as usize] = value; // TODO check echo ram
        } else if address >= OAM_START && address < UNUSABLE_START {
            return self.oam[(address - OAM_START) as usize] = value;
        } else if address >= IO_START && address < HRAM_START {
            return self.io[(address - IO_START) as usize] = value;
        } else if address >= HRAM_START && address < IE {
            return self.hram[(address - HRAM_START) as usize] = value;
        } else if address == IE {
            return self.ie = value;
        } else {
            panic!("invalid address: {:#x}", address)
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value >> 8) as u8);
        self.write_byte(address + 1, value as u8);
    }
}

const BOOTSTRAP_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];
