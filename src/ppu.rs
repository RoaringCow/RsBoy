use minifb::Scale;
use std::{thread, time};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;


#[derive(Debug)]
pub struct PPU {
    pub buffer: [u32; WIDTH * HEIGHT], // Change type to array
    pub vram: [u8; 0x2000], // Video RAM
    pub oam: [u8; 0xA0], // Object Attribute Memory


    ppu_mode: Ppumode,
    fetch_mode: Fetchmode,
    cycle: u16,
    scanline: u16,

    // 0xFF40 LCDC
    lcd_control: LcdControl,
    
    // 0xFF44 LY
    ly: u8,

    // 0xFF45 LYC
    lyc: u8,

    // 0xFF41 STAT
    stat: u8,


    // 0xFF42 SCY
    scy: u8,
    // 0xFF43 SCX
    scx: u8,

    //FF4A WY
    wy: u8,
    //FF4B WX
    wx: u8,

    // 0xFF47 BGP
    bgp: Pallette,

    // 0xFF48 OBP0
    obp0: Pallette,
    // 0xFF49 OBP1
    obp1: Pallette,

    

}



#[allow(dead_code)]
impl PPU {

    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            cycle: 0,
            scanline: 0,
            ppu_mode: Ppumode::OAM,
            fetch_mode: Fetchmode::Background,
            lcd_control: LcdControl{
                lcd_enable: true,
                window_tile_map: false,
                window_enable: false,
                bg_window_tile_data: false,
                bg_tile_map: false,
                sprite_size: false,
                sprite_enable: false,
                bg_enable: false,
            },
            ly: 0,
            lyc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            bgp: Pallette{
                color0: 0b00,
                color1: 0b01,
                color2: 0b10,
                color3: 0b11,
            },
            obp0: Pallette{
                color0: 0b00,
                color1: 0b01,
                color2: 0b10,
                color3: 0b11,
            },
            obp1: Pallette{
                color0: 0b00,
                color1: 0b01,
                color2: 0b10,
                color3: 0b11,
            },
        }
    }


    pub fn tick(&mut self) {
        match self.ppu_mode {
            Ppumode::OAM => {
                if self.cycle >= 80 {
                    self.ppu_mode = Ppumode::VRAM;
                    self.cycle = 0;
                }
            }
            Ppumode::VRAM => {
                if self.>= 172 {
                    self.ppu_mode = Ppumode::HBlank;
                    self.cycle = 0;
                }
            }
            Ppumode::HBlank => {
                if self.cycle >= 204 {
                    self.cycle = 0;
                    self.scanline += 1;
                    if self.scanline == 143 {
                        self.ppu_mode = Ppumode::VBlank;
                    } else {
                        self.ppu_mode = Ppumode::OAM;
                    }
                }
            }
            Ppumode::VBlank => {
                if self.cycle >= 456 {
                    self.cycle = 0;
                    self.scanline += 1;
                    if self.scanline > 153 {
                        self.scanline = 0;
                        self.ppu_mode = Ppumode::OAM;
                    }
                }
            }
        }
    }

    fn get_tile(&mut self){
    }

    fn get_tile_data(&self, tile_number: u8){
    }

    fn push_to_background_fifo(&mut self) {
    }
    fn print_to_screen(&mut self) {
    }
    fn sprite_background_mix(&mut self) {
    }


}

#[derive(Debug)]
pub enum Ppumode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}


#[derive(Debug, PartialEq)]
pub enum Fetchmode {
    Background,
    Sprite,
    //Window,
}

#[derive(Debug)]
pub struct LcdControl{
    pub lcd_enable: bool,
    pub window_tile_map: bool,
    pub window_enable: bool,
    pub bg_window_tile_data: bool,
    pub bg_tile_map: bool,
    pub sprite_size: bool,
    pub sprite_enable: bool,
    pub bg_enable: bool,
}

#[derive(Debug)]
pub struct Pallette {
    pub color0: u8,
    pub color1: u8,
    pub color2: u8,
    pub color3: u8,
}
