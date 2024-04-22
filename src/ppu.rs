// This version of ppu is not real world accurate.
// An other branch was left unfinished that was planned to be accurate
// wasted so much time that i stopped(fuck you pixel fifo)


use minifb::Scale;
use std::{iter::Enumerate, thread, time};

const WIDTH: usize = 256;//160;
const HEIGHT: usize = 256;//144;


#[derive(Debug)]
pub struct PPU {
    pub buffer: [u32; WIDTH * HEIGHT], // Change type to array
    pub display: [u32; WIDTH * HEIGHT],
    pub vram: [u8; 0x2000], // Video RAM
    pub oam: [u8; 0xA0], // Object Attribute Memory

    ppu_mode: Ppumode,
    pub cycle: u16,

    // 0xFF40 LCDC
    pub lcd_control: LcdControl,

    // 0xFF44 LY
    pub ly: u8,

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
            display: [0; WIDTH * HEIGHT],
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            cycle: 0,
            ppu_mode: Ppumode::OAM,
            lcd_control: LcdControl{
                lcd_enable: true,
                window_tile_map: false,
                window_enable: false,
                bg_window_tile_data: true,
                bg_tile_map: false,
                sprite_size: false,
                sprite_enable: true,
                bg_enable: true,
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
        }
    }


    pub fn update_display(&mut self) {

        // Background stuff
        let background_tilemap_offset = match self.lcd_control.bg_tile_map {
            true => 0x9C00,
            false => 0x9800
        };
        // loop through all the tiles
        for address in background_tilemap_offset..background_tilemap_offset + 1024{
            let tilemap_number = address - background_tilemap_offset;
            // number of tiles in a line / slice of tile width/ tile height
            let offset_y: usize = (tilemap_number / 32) as usize * 32 * 8 * 8;
            let offset_x: usize = (tilemap_number % 32) as usize * 8;
            for y in 0..8 {
                // Get a slice of the tile
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

                //println!("address: {:x}", address);
                let tile_number = self.vram[address as usize - 0x8000];
                let tile_data_address = tile_data_offset as usize + tile_number as usize * 16 + 2 * y;
                let tile_data_low = self.vram[tile_data_address - 0x8000] as u16;
                let tile_data_high = self.vram[tile_data_address + 1 - 0x8000] as u16;


                for x in 0..8 {
                    // map the color code to a value that minifb can use
                    let color = match ((tile_data_low >> (7 - x)) & 1) << 1 | ((tile_data_high >> (7 - x)) & 1) {
                        0 => 0x000000,
                        1 => 0x555555,
                        2 => 0xAAAAAA,
                        3 => 0xFFFFFF,
                        _ => 0x000000,
                    };
                    self.buffer[(y * 32 * 8) + x + offset_y + offset_x] = color;
                }
            }
        }


        // Sprite stuff
        //
        //
        if self.lcd_control.sprite_enable {
            for sprite_number in 0..40 {
                let sprite_y = self.oam[sprite_number as usize * 4];
                let sprite_x = self.oam[sprite_number as usize * 4 + 1];
                let tile_number = self.oam[sprite_number as usize * 4 + 2];
                let flags = self.oam[sprite_number as usize * 4 + 3];


                // ---- Check if Sprite is on screen ------
                // if out of screen on x
                if sprite_x == 0 || sprite_x >= 168 {
                    break;
                }
                // if out of screen on y
                let sprite_size = 8 + (8 * self.lcd_control.sprite_size as u8);
                if sprite_y == sprite_size - 8 || sprite_y >= 160 {
                    break;
                }

                // Getting the sprite data
                let mut sprite_data: [[u32; 8]; 8] = [[0; 8]; 8];
                for y in 0..8 {
                    // 0x8000 is not added because it will be subtracted otherwise
                    let address = 16 * tile_number + y * 2;
                    let data_low = self.vram[address as usize];
                    let data_high = self.vram[address as usize + 1];
                    for x in 0..8 {
                        // map the color code to a value that minifb can use
                        let color = match ((data_low >> (7 - x)) & 1) << 1 | ((data_high >> (7 - x)) & 1) {
                            0 => 0x000000,
                            1 => 0x555555,
                            2 => 0xAAAAAA,
                            3 => 0xFFFFFF,
                            _ => 0x000000,
                        };
                        sprite_data[y as usize][x as usize] = 0xCCCCCC;
                    }
                }


                for y in 0..8 {
                    if sprite_y + y >= 160 {break;}
                    for x in 0..8 {
                        if sprite_x + x >= 168 {break;}
                        todo!("fix division by zero");
                        let offset_y: usize = ((((sprite_y as usize - 16 + y as usize) % 144) + self.scy as usize) % 256) * 256;
                        let offset_x: usize = ((sprite_x as usize - 8 + x as usize) % 160 + self.scx as usize) % 256;
                        self.buffer[offset_y + offset_x];
                        println!("offsets: {}   x: {},    y: {}", offset_y % offset_x, offset_x, offset_y / 256);
                    }
                }


            }
        }
    }

}

#[derive(Debug)]
pub enum Ppumode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
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
