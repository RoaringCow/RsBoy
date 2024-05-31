use std::fs::File;
use std::io::{self, Read};
use super::ppu::PPU;



// boot rom to be mapped at boot
const DMG_BOOT_ROM: [u8; 0x100] = [
    0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26, 0xff, 0x0e,
    0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e, 0xfc, 0xe0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95, 0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b,
    0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06, 0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9,
    0x3e, 0x19, 0xea, 0x10, 0x99, 0x21, 0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20,
    0xf9, 0x2e, 0x0f, 0x18, 0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04,
    0x1e, 0x02, 0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
    0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64, 0x20, 0x06,
    0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15, 0x20, 0xd2, 0x05, 0x20,
    0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17,
    0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e,
    0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
    0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
    0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe, 0x34, 0x20,
    0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe, 0x3e, 0x01, 0xe0, 0x50,
];


pub struct Memory {
    pub rom: Cartridge, // rom
    pub ppu: PPU, // ppu
    wram: [u8; 0x2000], // work ram
    hram: [u8; 0x7F], // high ram
                      //interrupt enable
    ienable: u8,
    //interrupt flag
    iflag: u8,
}
impl Memory {
    pub fn reset(&mut self) {
        //self.ppu.reset();
        self.wram = [0; 0x2000];
        self.hram = [0; 0x7F];
        self.ienable = 0;
        self.iflag = 0;
    }
    pub fn new(file: &str) -> Memory {
        Memory {
            rom: Cartridge::new(file),
            ppu: PPU::new(),
            wram: [0; 0x2000],
            hram: [0; 0x7F],

            ienable: 0,
            iflag: 0,
        }

    }
    pub fn read_memory(&self, address: u16) -> u8 {
        let value = match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize], // ROM
            0x8000..=0x9FFF => self.ppu.vram[address as usize - 0x8000] as u8, // VRAM
            0xA000..=0xBFFF => 0xFF, // External RAM1
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000], // RAM
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000], // Echo RAM
            0xFE00..=0xFE9F => self.ppu.oam[address as usize - 0xFE00],//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => 0,// not usable
            0xFF00..=0xFF7F => { // IO
                match address {
                    0xFF0F => self.iflag, // interrupt flag register
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
            0xFFFF => self.ienable, // Interrupt Enable Register
            #[allow(unreachable_patterns)]
            _ => panic!("address out of range"), // cant get here

        };
        //println!("\x1b[38;2;0;255;0mreading memory\x1b[0m at address: \x1b[38;2;255;0;0m{:2X}\x1b[0m     value: \x1b[38;2;255;0;255m{:2X}\x1b[0m",address, value);
        value
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        println!("\x1b[38;2;255;255;0mwriting memory\x1b[0m at address: \x1b[38;2;255;0;255m{:4X}\x1b[0m    value: \x1b[38;2;255;0;255m{:2X}\x1b[0m", address, value);
        match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize] = value, // ROM
            0x8000..=0x9FFF => self.ppu.vram[address as usize - 0x8000] = value, // VRAM
            0xA000..=0xBFFF => (), // External RAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value, // RAM
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000] = value, // Echo RAM
            0xFE00..=0xFE9F => self.ppu.oam[address as usize - 0xFE00] = value,//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => (),// not usable
            0xFF00..=0xFF7F => {
                match address {
                    0xFF0F => self.iflag = value, // interrupt flag register
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
            0xFFFF => self.ienable= value, // Interrupt Enable Register
            _ => panic!("address out of range: {:x}", address),
        };
    }


    pub fn dma_transfer(&mut self) {
        println!("HOLY SHİT DMA HAPPENNİNG _______---__--__---___---");
        std::thread::sleep(std::time::Duration::from_millis(1000));
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
