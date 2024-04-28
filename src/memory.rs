use std::fs::File;
use std::io::{self, Read};
use super::ppu::PPU;

pub struct Memory {
    pub rom: Cartridge, // rom
    pub ppu: PPU, // ppu
    wram: [u8; 0x2000], // work ram
    hram: [u8; 0x7F], // high ram
    ie: u8, // interrupt enable register
}
impl Memory {
    pub fn reset(&mut self) {
        //self.ppu.reset();
        self.wram = [0; 0x2000];
        self.hram = [0; 0x7F];
        self.ie = 0;
    }
    pub fn new(file: &str) -> Memory {
        Memory {
            rom: Cartridge::new(file),
            ppu: PPU::new(),
            wram: [0; 0x2000],
            hram: [0; 0x7F],
            ie: 0,
        }

    }
    pub fn read_memory(&self, address: u16) -> u8 {
       // print!("\x1b[38;2;0;255;0mreading memory\x1b[0m at address: \x1b[38;2;255;0;0m{:2X}\x1b[0m", address);
        match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize], // ROM
            0x8000..=0x9FFF => self.ppu.vram[address as usize - 0x8000] as u8, // VRAM
            0xA000..=0xBFFF => 0xFF, // External RAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000], // RAM
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000], // Echo RAM
            0xFE00..=0xFE9F => self.ppu.oam[address as usize - 0xFE00],//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => 0,// not usable
            0xFF00..=0xFF7F => { // IO
                match address {
                    // ------------------------------------------------------
                    //FF40 - FF4B lcd control, status, position .... registers
                    0xFF40 => self.ppu.lcd_control,
                    0xFF41 => self.ppu.stat,
                    0xFF42 => self.ppu.scy,
                    0xFF43 => self.ppu.scx,
                    0xFF44 => self.ppu.ly,
                    0xFF45 => self.ppu.lyc,
                    0xFF46 => self.ppu.dma_address,
                    0xFF47 => {
                        (self.ppu.bgp.color3 << 6) |
                        (self.ppu.bgp.color2 << 4) |
                        (self.ppu.bgp.color1 << 2) |
                        (self.ppu.bgp.color0)
                    },
                    0xFF48 => {
                        (self.ppu.obp0.color3 << 6) |
                        (self.ppu.obp0.color2 << 4) |
                        (self.ppu.obp0.color1 << 2) |
                        (self.ppu.obp0.color0)
                    },
                    0xFF49 => {
                        (self.ppu.obp1.color3 << 6) |
                        (self.ppu.obp1.color2 << 4) |
                        (self.ppu.obp1.color1 << 2) |
                        (self.ppu.obp1.color0)
                    },
                    0xFF4A => self.ppu.wy,
                    0xFF4B => self.ppu.wx,
                    // ------------------------------------------------------
                    // ------------------------------------------------------



                    // TODO implement the rest of the registers
                    _ => 0xFF
                }
            },
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80], // High RAM
            0xFFFF => self.ie, // Interrupt Enable Register
            #[allow(unreachable_patterns)]
            _ => panic!("address out of range"), // cant get here

        }
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        ////println!("\x1b[38;2;255;255;0mwriting memory\x1b[0m at address: \x1b[38;2;255;0;255m{:4X}\x1b[0m    value: \x1b[38;2;255;0;255m{:2X}\x1b[0m", address, value);
        match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize] = value, // ROM
            0x8000..=0x9FFF => self.ppu.vram[address as usize - 0x8000] = value, // VRAM
            0xA000..=0xBFFF => (), // External RAM
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000] = value, // RAM
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000] = value, // Echo RAM
            0xFE00..=0xFE9F => self.ppu.oam[address as usize - 0xFE00] = value,//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => (),// not usable
            0xFF00..=0xFF7F => {
                match address {
                    // ------------------------------------------------------
                    //FF40 - FF4B lcd control, status, position .... registers
                    0xFF40 => self.ppu.lcd_control = value,
                    0xFF41 => self.ppu.stat = value,
                    0xFF42 => self.ppu.scy = value,
                    0xFF43 => self.ppu.scx = value,
                    0xFF44 => self.ppu.ly = value,
                    0xFF45 => self.ppu.lyc = value,
                    0xFF46 => {
                        self.ppu.dma_address = value;
                        self.dma_transfer();
                    }
                    0xFF47 => {
                        self.ppu.bgp.color3 = (value >> 6) & 0b11;
                        self.ppu.bgp.color2 = (value >> 4) & 0b11;
                        self.ppu.bgp.color1 = (value >> 2) & 0b11;
                        self.ppu.bgp.color0 = value & 0b11;
                    },
                    0xFF48 => {
                        self.ppu.obp0.color3 = (value >> 6) & 0b11;
                        self.ppu.obp0.color2 = (value >> 4) & 0b11;
                        self.ppu.obp0.color1 = (value >> 2) & 0b11;
                        self.ppu.obp0.color0 = value & 0b11;
                    },
                    0xFF49 => {
                        self.ppu.obp1.color3 = (value >> 6) & 0b11;
                        self.ppu.obp1.color2 = (value >> 4) & 0b11;
                        self.ppu.obp1.color1 = (value >> 2) & 0b11;
                        self.ppu.obp1.color0 = value & 0b11;
                    },
                    0xFF4A => self.ppu.wy = value,
                    0xFF4B => self.ppu.wx = value,
                    // ------------------------------------------------------
                    // ------------------------------------------------------
                


                    _ => (),
                }
            } ,
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value, // High RAM
            0xFFFF => self.ie = value, // Interrupt Enable Register
            _ => panic!("address out of range"),
        };
    }


    pub fn dma_transfer(&mut self) {
        let source_address = (self.read_memory(0xFF46) as u16) << 8;
        for x in 0..160 {
            // transfer sprite data
            self.write_memory(0xFE00 + x, self.read_memory(source_address + x));
        }
    }

}


pub struct Cartridge {
    pub rom: [u8; 0x8000],
    pub banks: Vec<[u8; 0x4000]> // 16kb banks
}
#[allow(dead_code)]
impl Cartridge {
    pub fn new(file: &str) -> Cartridge {
        let cartridge = Cartridge::parse_file_to_vector(file).unwrap();
        Cartridge {
            rom: cartridge[0..0x8000].try_into().unwrap(),
            banks: cartridge.chunks(0x4000).map(|chunk| chunk.try_into().unwrap()).collect(), 
        }
    
    }

    fn parse_file_to_vector(file_path: &str) -> io::Result<Vec<u8>> {
        // Open the file
        let mut file = File::open(file_path)?;

        // Read all bytes from the file
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;

        if file_bytes.len() < 0x8000 {
            // Calculate the number of zeros to fill
            let zeros_to_fill = 0x8000 - file_bytes.len();

            // Extend the vector with zeros
            file_bytes.extend(std::iter::repeat(0).take(zeros_to_fill));
            println!("file size: {}", file_bytes.len());
        }

        Ok(file_bytes)
    }

    pub fn switch_bank(&mut self, bank: u8) {
        // copy the bank into the second half of the rom
        self.rom[0x4000..0x8000].copy_from_slice(&self.banks[bank as usize]);
    }
}
