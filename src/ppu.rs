use minifb::Scale;


const WIDTH: usize = 160;
const HEIGHT: usize = 144;


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
    // TO BE EXPLAINED
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

    pub background_fifo: Vec<u8>,

    pub tile_number : u8,
    pub tile_data: u16,
    pub is_there_data: bool,
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
            state: Ppumode::VRAM,
            fetcher_x: 0,
            background_fifo: Vec::new(),
            fifo_x_coordinate: 0,

            current_line: 0,

            tile_number: 0,
            tile_data: 0,
            is_there_data: false,

        }
    }


    pub fn tick(&mut self) {
        match self.state {
            Ppumode::OAM => {

                // OAM Search

                /*
                // set stat interrupt
                if self.ly == self.lyc {
                    self.stat |= 0x04;
                } else {
                    self.stat &= !0x04;
                }

                if self.ticks >= 80 {
                    self.state = Ppumode::VRAM;
                    self.ticks = 0;
                } else {

                    self.ticks += 2;
                }
                */
                println!("OAM");
                self.cycle = 80;
                self.state = Ppumode::VRAM;
            }
            Ppumode::VRAM => {

                if self.fifo_x_coordinate >= 160 {
                    self.fifo_x_coordinate = 0;
                    self.background_fifo.clear();
                    self.fetcher_x = 0;
                    self.state = Ppumode::HBlank;
                }
                match self.cycle % 8 {
                    0 => {
                        // read_tile
                        self.tile_number = self.get_tile();
                    }
                    // Artificially simulating the cycles
                    4 => {
                        // read data 1
                        self.tile_data = self.get_tile_data(self.tile_number);
                        self.is_there_data = true;
                    }
                    _ => {}
                }
                println!("background fifo: {:?}", self.background_fifo);
                println!("Cycle: {}", self.cycle);
                self.print_to_screen();
                self.push_to_fifo();
                


                




                
            }
            Ppumode::HBlank => {
                println!("HBlank");
                //cycle
                println!("Cycle: {}", self.cycle);
                // HBlank
                if self.cycle >= 456 {
                    self.cycle = 0;
                    self.current_line += 1;
                    if self.current_line == 144 {
                        self.state = Ppumode::VBlank;
                    } else {
                        self.state = Ppumode::OAM;
                    }
                }
            }
            Ppumode::VBlank => {
                // VBlank
            }
        }

        self.cycle += 1;
    }

    fn get_tile(&mut self) -> u8 {
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
    }

    fn get_tile_data(&self, tile_number: u8) -> u16 {
        let mut tile_id = tile_number as u16;
        let base_address = match self.lcdc >> 4 & 1 == 0 {
            true => {
                tile_id = tile_id.wrapping_add(128);
                0x8800
            },
            false => 0x8000,
        };
        let tile_offset = 2 * (self.ly.wrapping_add(self.scy)) & 7; // & 7 is mod 8
        let tile_address = base_address + (tile_id as u16 * 16) + tile_offset as u16;
        println!("tile number: {:X}", tile_number);
        println!("\x1b[38;2;200;100;0m Address to get tile data: \x1b[0m: {:X}", tile_address);
        ((self.vram[tile_address as usize - 0x8000] as u16) << 8) | self.vram[tile_address as usize - 0x8000 + 1] as u16
    }

    fn push_to_fifo(&mut self) {
        if self.is_there_data {
            if self.background_fifo.len() <= 8 {
                let first_byte: u8 = (self.tile_data >> 8) as u8;
                let second_byte: u8 = self.tile_data as u8;

                for i in 0..8 {
                    // split and get the representing color
                    // lsb becomes the msb for the pixel and msb becomes the lsb
                    let color = (first_byte >> (7 - i) & 1) | ((second_byte >> (7 - i)) & 1) << 1;
                    println!("Color: {}", color);
                    self.background_fifo.push(color as u8);
                }
                self.tile_data = 0;
                self.is_there_data = false;
            }
        }
    }
    fn print_to_screen(&mut self) {
        // print to screen
        if self.background_fifo.len() > 8 {
            println!("-------------------------------------------------------");
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


}

pub enum Ppumode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}

