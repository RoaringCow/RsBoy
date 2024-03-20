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

    fetcher_x: u8,
    sprite_buffer: Vec<(u8, u8, u8, u8)>,
    tile_number: u8,
    tile_data: u16,
    fifo_push: bool,
    fetcher_cycle: u8,

}



#[allow(dead_code)]
impl PPU {

    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            cycle: 0,
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
            ly: 0, // scanline
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

            fetcher_x: 0,
            sprite_buffer: Vec::new(),
            tile_number: 0,
            tile_data: 0,
            fifo_push: true,
            fetcher_cycle: 0,
            
        }
    }


    pub fn tick(&mut self) {
        if !self.lcd_control.lcd_enable {
            return;
        }


        match self.ppu_mode {
            Ppumode::OAM => {
                if self.cycle >= 80 {
                    self.ppu_mode = Ppumode::VRAM;
                }

                if self.cycle % 2 == 0 {
                    let sprite_number = self.cycle / 2;


                    let sprite_y = self.oam[sprite_number as usize * 4];
                    let sprite_x = self.oam[sprite_number as usize * 4 + 1];
                    let tile_number = self.oam[sprite_number as usize * 4 + 2];
                    let flags = self.oam[sprite_number as usize * 4 + 3];

            // ------------- sprite conditions ----------------
                    //Sprite X-Position must be greater than 0
                    //LY + 16 must be greater than or equal to Sprite Y-Position
                    //LY + 16 must be less than Sprite Y-Position + Sprite Height (8 in Normal Mode, 16 in Tall-Sprite-Mode)
                    //The amount of sprites already stored in the OAM Buffer must be less than 10
            // ------------------------------------------------

                    if sprite_x >0 && self.ly + 16 >= sprite_y && self.ly + 16 < sprite_y + 8 && self.sprite_buffer.len() < 10 {
                        self.sprite_buffer.push((sprite_y, sprite_x, tile_number, flags));
                    }
                }
            }
            Ppumode::VRAM => {
                if self.fetcher_x >= 160 {
                    self.ppu_mode = Ppumode::HBlank;
                    self.fetcher_x = 0;
                }

                if self.fetch_mode == Fetchmode::Background {
                    if self.sprite_buffer.len() > 0 && (self.sprite_buffer[0].1 as u16) - 8 == self.fetcher_x as u16 {
                        self.fetch_mode = Fetchmode::Sprite;
                        self.fifo_push = true;
                        self.fetcher_cycle = 0;
                    }
                }

                match self.fetch_mode {
                    Fetchmode::Background => {
                        match self.fetcher_cycle {
                            0 => {
                                self.get_tile();
                            },
                            5 => {
                                self.get_tile_data(self.tile_number);
                            },
                            _ => {}
                        }
                        self.fetcher_cycle = self.fetcher_cycle.wrapping_add(1);

                        self.push_to_background_fifo();
                    }
                    Fetchmode::Sprite => {
                        self.get_tile();
                        self.get_tile_data(self.tile_number);
                        self.sprite_background_mix();
                    }
                    Fetchmode::Window => {
                        self.get_tile();
                        self.get_tile_data(self.tile_number);
                        self.push_to_background_fifo();
                        self.print_to_screen();
                    }
                }
                
            }
            Ppumode::HBlank => {
                if self.cycle >= 456 {
                    self.cycle = 0;
                    self.ly += 1;
                    self.ppu_mode = Ppumode::OAM;
                    if self.ly == 144 {
                        self.ppu_mode = Ppumode::VBlank;
                    }
                }
            }
            Ppumode::VBlank => {
            }
        }


        self.cycle += 1;
    }

    fn get_tile(&mut self){
        // these informations were taken from https://hacktix.github.io/GBEDG/ppu/
        match self.fetch_mode {
            Fetchmode::Background => {
                let tilemap_offset = match self.lcd_control.bg_tile_map {
                    true => {
                        // 0x9C00 - 0x9FFF
                        0x9C00
                    }
                    false => {
                        // 0x9800 - 0x9BFF
                        0x9800
                    }
                };
                let tile_number_address = (self.scx / 8) & 0x1F + ((self.ly + self.scy) / 8) * 32;
                self.tile_number = self.vram[tilemap_offset as usize + tile_number_address as usize];
            }
            Fetchmode::Sprite => {
                let tilemap_offset = 0x8000;
                todo!();

            }
            Fetchmode::Window => {
                let tilemap_offset = match self.lcd_control.window_tile_map {
                    true => {
                        // 0x9C00 - 0x9FFF
                        0x9C00
                    }
                    false => {
                        // 0x9800 - 0x9BFF
                        0x9800
                    }
                };
                todo!();

            }
        }
    }

    fn get_tile_data(&mut self, tile_number: u8){
        match self.fetch_mode {
            Fetchmode::Background => {
                let tile_data_offset = match self.lcd_control.bg_window_tile_data {
                    true => {
                        // 0x8000 - 0x8FFF
                        0x8000
                    }
                    false => {
                        // 0x8800 - 0x97FF
                        0x8800
                    }
                };
                let tile_data_address = tile_data_offset as usize + tile_number as usize * 16 + (2 * ((self.ly + self.scy) as usize % 8));
                let tile_data_low = self.vram[tile_data_address - 0x8000] as u16;
                let tile_data_high = self.vram[tile_data_address + 1 - 0x8000] as u16;
                let mut tile_data: u16 = 0;
                for i in 0..8 {
                    let bit = ((tile_data_low >> (7 - i)) & 1) << 1 | ((tile_data_high >> (7 - i)) & 1);
                    tile_data |= bit << (i * 2);
                }
                self.tile_data = tile_data;

            },
            Fetchmode::Sprite => {
                todo!();
            },
            Fetchmode::Window => {
                todo!();
            }
        }
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
    Window,
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
