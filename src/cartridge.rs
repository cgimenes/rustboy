use std::fs;

#[derive(Debug)]
pub enum Cartridge {
    Empty,
    Loaded { rom: Vec<u8>, ram: Vec<u8> },
}

impl Cartridge {
    pub fn from_file(path: &str) -> Self {
        let data = fs::read(path).expect("Should have been able to read the file");

        Cartridge::Loaded {
            rom: data,
            ram: Vec::new(),
        }
    }

    pub fn read_rom(&self, address: u16) -> Result<u8, String> {
        match self {
            Self::Empty => Err("empty cartridge".to_string()),
            Self::Loaded { rom, ram: _ } => Ok(rom[address as usize]),
        }
    }

    pub fn read_ram(&self, address: u16) -> Result<u8, String> {
        match self {
            Self::Empty => Err("empty cartridge".to_string()),
            Self::Loaded { rom: _, ram } => Ok(ram[address as usize]),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        match self {
            Self::Empty => Err("empty cartridge".to_string()),
            Self::Loaded { rom: _, ram } => Ok(ram[address as usize] = value),
        }
    }
}
