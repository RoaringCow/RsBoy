
const WIDTH: usize = 640;
const HEIGHT: usize = 360;


pub struct PPU {
    pub buffer: [u32; WIDTH * HEIGHT], // Change type to array
    pub vram: [u8; 0x2000], // Video RAM
    pub oam: [u8; 0xA0], // Object Attribute Memory


    pub cycle: u8,

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
    pub ticks: u8,
   
    // these are the fifos.
    // yep only u32's
    // They do the job so why bother make another struct
    pub background_fifo: u32,
    pub pixel_fifo: u32,
    
    pub fetcher_x: u8,

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
        self.ticks = 0;
        self.background_fifo = 0;
        self.pixel_fifo = 0;
        self.fetcher_x = 0;
    }

    // Create a new display
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
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
            ticks: 0,
            background_fifo: 0,
            pixel_fifo: 0,

            fetcher_x: 0,
        }
    }

    pub fn push_fifo(&mut self){
        todo!();
        self.buffer[self.ly as usize * WIDTH + self.scx as usize] = self.background_fifo >> 30;
        self.background_fifo <<= 2;
    }

    pub fn tick(&mut self) {
        match self.state {
            Ppumode::OAM => {

                // OAM Search
                
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
            
            }
            Ppumode::VRAM => {
                
                // fetch tile no
                // todo! window hanling
                let mut address: u16 = if self.lcdc & 0x08 == 0 {
                    0x9800
                } else {
                    0x9C00
                };
                let y_offset = 32 * (((self.ly as u16 + self.scy as u16) / 8) & 0xFF) as u16;
                let x_offset = ((((self.scx as u16) / 8) + self.fetcher_x as u16) & 0xFF) as u16;
                address += y_offset + x_offset;
                println!("address: {:X}, y_offset: {:X}, x_offset: {:X}", address, y_offset, x_offset);
                self.buffer[self.ly as usize * WIDTH + self.fetcher_x as usize] = self.background_fifo >> 30;
                // fetch tile data(Low)
                // fetch tile data(High)
                // push to fifo
                // Pixel Transfer
                self.fetcher_x += 1;
            }
            Ppumode::HBlank => {
                // HBlank
            }
            Ppumode::VBlank => {
                // VBlank
            }
        } 
    }


}

pub enum Ppumode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}


// FF40 - LCDC - LCD Control (R/W)
// bit 7 - LCD Display Enable             (0=Off, 1=On)
// bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
// bit 5 - Window Display Enable          (0=Off, 1=On)
// bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
// bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
// bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
// bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
// bit 0 - BG Display (for CGB see below) (0=Off, 1=On)

