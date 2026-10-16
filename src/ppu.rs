use crate::mmu::MMU;

#[derive(Debug)]
pub struct PPU {}

pub struct Tile {
    pub pixels: [u8; 64],
}

pub struct Palettes {
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_tile(&self, id: u16, mmu: &MMU) -> Result<Tile, String> {
        let start: u16 = 0x8000 + (id * 16);

        let mut pixels: Vec<u8> = Vec::with_capacity(64);
        for i in 0..8 {
            let left = mmu.read_byte(start + i)?;
            let right = mmu.read_byte(start + i + 8)?;

            let bytes = interleave_bits(left, right);
            for j in 0..8 {
                pixels.push(bytes[j as usize]);
            }
        }

        Ok(Tile {
            pixels: *pixels.as_array().unwrap(),
        })
    }

    pub fn get_tiles(&self, mmu: &MMU) -> Result<Vec<Tile>, String> {
        // FIX check LCDC bit 4: https://gbdev.io/pandocs/LCDC.html#lcdc4--bg-and-window-tile-data-area
        let mut tiles: Vec<Tile> = Vec::with_capacity(384);

        for i in 0..384 {
            tiles.push(self.get_tile(i, mmu)?);
        }
        Ok(tiles)
    }

    pub fn get_palettes(&self, mmu: &MMU) -> Result<Palettes, String> {
        Ok(Palettes {
            bgp: mmu.read_byte(0xFF47)?,
            obp0: mmu.read_byte(0xFF48)?,
            obp1: mmu.read_byte(0xFF49)?,
        })
    }
}
fn interleave_bits(a: u8, b: u8) -> [u8; 8] {
    let mut result: [u8; 8] = [0; 8];
    for i in (0..8).rev() {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        result[i] = (bit_a << 1) + bit_b;
    }
    result
}
