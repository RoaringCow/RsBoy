use minifb::Scale;
use std::{thread, time};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;


#[derive(Debug)]
pub struct PPU {
    pub buffer: [u32; WIDTH * HEIGHT], // Change type to array
    pub vram: [u8; 0x2000], // Video RAM
    pub oam: [u8; 0xA0], // Object Attribute Memory


    pub cycle: u16,

    // Status Registers
    // FF40 - LCDC - LCD Control (R/W)
    // bit 7 - LCD Display Enable             (0=Off, 1=On)
    // bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    // bit 5 - Window Display Enable          (0=Off, 1=On)
    // bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
    // bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
    // bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
    // bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
    // bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
    pub lcdc: u8,

    // FF41 - STAT - LCDC Status (R/W)
    pub stat: u8,

    // FF42 - SCY - Scroll Y (R/W)
    pub scy: u8,
    // FF43 - SCX - Scroll X (R/W)
    pub scx: u8,

    // FF44 - LY - LCDC Y-Coordinate (Read only)
    pub ly: u8,


    // FF45 - LYC - (Read/Write)
    // this is compared with ly 
    // used for games that need to know 
    // when a specific line is reached
    pub lyc: u8,

    // FF47 - BGP - BG Palette Data (R/W)
    pub palette: u8,


    // Window Position
    // FF4A - WY - Window Y Position (R/W)
    pub wy: u8,
    // FF4B - WX - Window X Position minus 7 (R/W)
    pub wx: u8,



    pub state: Ppumode,

    pub fetcher_x: u8,

    pub fifo_x_coordinate: u8,

    pub current_line : u8,


    // background fifo stuff
    pub background_fifo: Vec<u8>,


    // fetching sprite or background
    pub tile_number : u8,
    pub tile_data: u16,
    pub is_there_data: bool,

    // Sprite fetching stuff

    pub fetcher_mode: Fetchmode,
    pub sprite_buffer: Vec<u32>,
    pub fifo_push: bool,
    // this is to simulate the two dots sizes of the fetcher operations
    pub fetcher_cycle: u8,




}



#[allow(dead_code)]
impl PPU {

    pub fn reset(&mut self) {
        self.buffer = [0; WIDTH * HEIGHT];
        self.cycle = 0;
        self.lcdc = 0;
        self.stat = 0;
        self.scy = 0;
        self.scx = 0;
        self.ly = 0;
        self.lyc = 0;
        self.palette = 0;
        self.wy = 0;
        self.wx = 0;
        self.state = Ppumode::OAM;
        self.fetcher_x = 0;
        self.current_line = 0;


    }
    // Create a new display
    pub fn new() -> Self {
        Self {
            buffer: [0xFFFFFF; WIDTH * HEIGHT],
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            cycle: 0,
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            palette: 0,
            wy: 0,
            wx: 0,
            state: Ppumode::OAM,
            fetcher_x: 0,

            background_fifo: Vec::new(),
            fifo_x_coordinate: 0,
            fifo_push: true,

            current_line: 0,

            tile_number: 0,
            tile_data: 0,
            is_there_data: false,

            sprite_buffer: Vec::new(),

            fetcher_cycle: 0,
            fetcher_mode: Fetchmode::Background,



        }
    }


    pub fn tick(&mut self) {
        let mut end_of_line = false;
        match self.state {
            Ppumode::OAM => {

                // OAM Search

                /*
                if self.sprite_buffer.len() > 0 {
                    println!("sprite ALERT!  {}", self.current_line);
                }
                for x in self.sprite_buffer.iter() {
                    println!("sprite buffer: {}", x >> 24);
                }
                println!("oam: {:?}", self.oam);
                */

                /*
                Sprite X-Position must be greater than 0
                LY + 16 must be greater than or equal to Sprite Y-Position
                LY + 16 must be less than Sprite Y-Position + Sprite Height (8 in Normal Mode, 16 in Tall-Sprite-Mode)
                The amount of sprites already stored in the OAM Buffer must be less than 10
                */

                if (self.cycle) % 2 == 0 {
                    // the OAM search is 80 cycles long and in each 2 cycle it checks for 1 sprite
                    // so self.cycle % 2 == 0 is to only do 40 sprite checks.
                    // and the self.cycle * 2 is to get the sprite number
                    let current_sprite_number = (self.cycle) * 2;
                    let sprite_y: u8= self.oam[(current_sprite_number) as usize];
                    let sprite_x: u8= self.oam[(current_sprite_number) as usize + 1];
                    let sprite_tile_index: u8 = self.oam[(current_sprite_number) as usize + 2];
                    let sprite_flags: u8 = self.oam[(current_sprite_number) as usize + 3];
                    //println!("{} {} {} {}", sprite_x > 0, self.current_line + 16 >= sprite_y, self.current_line + 16 < sprite_y + 8, self.sprite_buffer.len() < 10);
                    if sprite_x > 0 && self.current_line + 16 >= sprite_y && self.current_line + 16 < sprite_y + 8 && self.sprite_buffer.len() < 10 {
                        let sprite_that_fits: u32 = ((sprite_y as u32) << 24) | ((sprite_x as u32) << 16) | ((sprite_tile_index as u32) << 8) | (sprite_flags as u32);
                        self.sprite_buffer.push(sprite_that_fits);
                    }
                }

                if self.cycle >= 79 {
                    self.state = Ppumode::VRAM;
                }


            }
            Ppumode::VRAM => {

                if self.fifo_x_coordinate >= 160 {
                    self.fifo_x_coordinate = 0;
                    self.background_fifo.clear();
                    self.is_there_data = false;
                    self.state = Ppumode::HBlank;
                }

                /*
                If the X-Position of any sprite in the sprite buffer is less than or equal to
                the current Pixel-X-Position + 8, a sprite fetch is initiated.
                This resets the Background Fetcher to step 1 and temporarily pauses it,
                the pixel shifter which pushes pixels to the LCD is also suspended.
                The Sprite Fetcher works very similarly to the background fetcher:

                1) Fetch Tile No.: Same as the first Background Fetcher step, however, the tile number is simply read from the Sprite Buffer rather than VRAM.

                    2) Fetch Tile Data (Low): Same as the corresponding Background Fetcher step, however, Sprites always use 8000-addressing-mode, so this step is not affected by any LCDC bits.

                    3) Fetch Tile Data (High): Same as the corresponding Background Fetcher step.

                    4) Push to FIFO: The fetched pixel data is loaded into the FIFO on the first cycle of this step, allowing the first sprite pixel to be rendered in the same cycle. However, the check determining whether new sprite pixels should be rendered is done first, which can cause the PPU to not shift out any pixels at all between two sprite fetches, for example if both sprites have X-values below 8 or both sprites have the same X-value.

                    Note: During this step only pixels which are actually visible on the screen are loaded into the FIFO. A sprite with an X-value of 8 would have all 8 pixels loaded, while a sprite with an X-value of 7 would only have the rightmost 7 pixels loaded. Additionally, pixels can only be loaded into FIFO slots if there is no pixel in the given slot already. For example, if the Sprite FIFO contains one sprite pixel from a previously fetched sprite, the first pixel of the currently fetched sprite is discarded and only the last 7 pixels are loaded into the FIFO, while the pixel from the first sprite is preserved.
                */


                if self.fetcher_mode == Fetchmode::Background {
                    if self.sprite_buffer.len() > 0 && (self.sprite_buffer[0] >> 16) as u8 - 8 == self.fifo_x_coordinate {
                        self.fetcher_mode = Fetchmode::Sprite;
                        self.fifo_push = false;
                        self.fetcher_cycle = 0;
                    }
                }


                match self.fetcher_mode {
                    Fetchmode::Background => {
                        match self.fetcher_cycle % 8 {
                            0 => {
                                // read_tile
                                self.tile_number = self.get_tile();
                            }
                            // Artificially simulating the cycles
                            5 => {
                                // read data 1
                                self.tile_data = self.get_tile_data(self.tile_number);
                                self.is_there_data = true;
                            }
                            _ => {}
                        }
                        self.fetcher_cycle = self.fetcher_cycle.wrapping_add(1);


                        self.push_to_background_fifo();
                    }
                    Fetchmode::Sprite => {
                        if self.fetcher_x == 0 {
                            self.fetcher_mode = Fetchmode::Background;
                            self.tile_number = self.get_tile();
                            self.tile_data = self.get_tile_data(self.tile_number);
                            self.is_there_data = true;
                            self.push_to_background_fifo();
                            self.fetcher_mode = Fetchmode::Sprite;
                        }
                        match self.fetcher_cycle % 8 {
                            0 => {
                                // read_tile
                                self.tile_number = self.get_tile();
                            }
                            // Artificially simulating the cycles
                            5 => {
                                // read data 1
                                self.tile_data = self.get_tile_data(self.tile_number);
                                self.is_there_data = true;
                                // boş boş uğraşmamak için. Zaten + 1 olacak. Bir de üstüne kontrol mü edeceğim.
                                self.fetcher_cycle = u8::from(0).wrapping_sub(1); // 255

                                self.sprite_background_mix();

                                // THIS MIGHT BE A BAD WAY
                                self.sprite_buffer.remove(0);
                                //--------------------------------


                                self.fetcher_mode = Fetchmode::Background;
                                self.fifo_push = true;
                            }
                            _ => {}
                        }
                        self.fetcher_cycle = self.fetcher_cycle.wrapping_add(1);
                    }
                }

                if self.fifo_push {
                    self.print_to_screen();
                }
                //println!("fifo:  {:?}  {:?}        {}", self.fetcher_mode, self.background_fifo, self.fetcher_x);






            }
            Ppumode::HBlank => {
                //cycle
                // HBlank
                if self.cycle >= 456 {
                    self.sprite_buffer.clear();
                    self.cycle = 0;
                    self.current_line += 1;
                    if self.current_line == 144 {
                        self.state = Ppumode::VBlank;
                        end_of_line = true;
                    } else {
                        self.fetcher_x = 0;
                        self.state = Ppumode::OAM;
                        end_of_line = true;
                    }
                }
            }
            Ppumode::VBlank => {
                // VBlank
                if self.cycle >= 456 {
                    self.cycle = 0;
                    self.current_line += 1;
                    if self.current_line > 153 {
                        self.current_line = 0;
                        self.state = Ppumode::OAM;
                    }
                }
            }
        }

        // this prevents increasing cycles when switching to the next line
        if !end_of_line {
            self.cycle += 1;
        }
    }

    fn get_tile(&mut self) -> u8 {
        match self.fetcher_mode {
            Fetchmode::Background => {
                // get tile index
                let tile_base_address = match self.lcdc >> 3 & 1 == 0 {
                    true => 0x9800,
                    false => 0x9C00,
                };
                // scx is scrolled to get it divided by 8 (tile size)

                let tile_x: u8 = ((self.scx >> 3) + self.fetcher_x) & 0x1F;
                let tile_y: u8 = self.current_line.wrapping_add(self.scy) >> 3;
                let tile_address = tile_base_address + (tile_y as u16 * 32) + tile_x as u16;
                self.fetcher_x += 1;
                self.vram[tile_address as usize - 0x8000]
            },
            Fetchmode::Sprite => {
                (self.sprite_buffer[0] >> 8 & 0xFF) as u8
            }
        }
    }

    fn get_tile_data(&self, tile_number: u8) -> u16 {
        let mut tile_id = tile_number as u16;
        let base_address = match self.fetcher_mode {
            Fetchmode::Background => {
                match self.lcdc >> 4 & 1 == 0 {
                    true => {
                        tile_id = tile_id.wrapping_add(128);
                        0x8800
                    },
                    false => 0x8000,
                }
            },
            Fetchmode::Sprite => {
                0x8000
            }
        };

        let tile_offset = 2 * (self.ly.wrapping_add(self.scy)) & 7; // & 7 is mod 8
        let tile_address = base_address + (tile_id as u16 * 16) + tile_offset as u16;
        ((self.vram[tile_address as usize - 0x8000] as u16) << 8) | self.vram[tile_address as usize - 0x8000 + 1] as u16
    }

    fn push_to_background_fifo(&mut self) {
        if self.is_there_data {
            if self.background_fifo.len() <= 8 {
                let first_byte: u8 = (self.tile_data >> 8) as u8;
                let second_byte: u8 = self.tile_data as u8;

                for i in 0..8 {
                    // split and get the representing color
                    // lsb becomes the msb for the pixel and msb becomes the lsb
                    let color = (first_byte >> (7 - i) & 1) | ((second_byte >> (7 - i)) & 1) << 1;
                    self.background_fifo.push(color);
                }
                self.tile_data = 0;
                self.is_there_data = false;
            }
        }
    }
    fn print_to_screen(&mut self) {
        // print to screen
        if self.background_fifo.len() > 8 {
            let address = self.fifo_x_coordinate as usize + self.current_line as usize * WIDTH;
            let value = self.background_fifo.remove(0);
            let color = match value {
                0 => 0x000000,
                1 => 0x555555,
                2 => 0xAAAAAA,
                3 => 0xFFFFFF,
                _ => 0x000000,
            };
            self.buffer[address] = color;
            self.fifo_x_coordinate += 1;
        }

    }
    fn sprite_background_mix(&mut self) {
        // mix the background and sprite fetchers

        let first_byte: u8 = (self.tile_data >> 8) as u8;
        let second_byte: u8 = self.tile_data as u8;

        for i in 0..8 {
            // split and get the representing color
            // lsb becomes the msb for the pixel and msb becomes the lsb
            let color = (first_byte >> (7 - i) & 1) | ((second_byte >> (7 - i)) & 1) << 1;

            if color == 0 {
                continue;
            }
            if self.sprite_buffer[0] >> 7 & 1 == 1 && self.background_fifo[0] != 0 {
                continue;
            }

            // ARA SIRA NEDENSE FIFODA VERİ OLMUYOR.
            self.background_fifo[i] = color;
        }
        self.tile_data = 0;

        /*
            1) If the color number of the Sprite Pixel is 0, the Background Pixel is pushed to the LCD.
                2) If the BG-to-OBJ-Priority bit is 1 and the color number of the Background Pixel is anything other than 0, the Background Pixel is pushed to the LCD.
                3) If none of the above conditions apply, the Sprite Pixel is pushed to the LCD.
            */

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