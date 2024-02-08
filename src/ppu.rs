
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


    // FF45 - LYC - LY Compare (Read/Write)
    pub lyc: u8,
    
    // FF47 - BGP - BG Palette Data (R/W)
    pub palette: u8,


    // FF4A - WY - Window Y Position (R/W)
    pub wy: u8,
    // FF4B - WX - Window X Position minus 7 (R/W)
    pub wx: u8,



   

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
        }
    }



}


struct FIFO {
   buffer: u32,
   current_tile_address : u8,
   
}
impl FIFO {
    pub fn new() -> Self {
        Self {
            buffer: 0,
            current_tile_address: 0,
        }
    }
    pub fn fetch(&mut self, ppu: &PPU) -> u32 {
        let address: u16;
        if ppu.lcdc & 0b00001000 == 0 {
            address = 0x9000 + self.current_tile_address as u16;
        } else {
            address = 0x8000 + self.current_tile_address as u16;
        }
        let value = ppu.buffer[address as usize];
        value as u32
    }

    fn is_in_window(&self, ppu: &PPU) -> bool {
        if ppu.lcdc & 0b00100000 == 0 {
            return false;
        }
        if ppu.scx <= ppu.wx {
            return false;
        }
        return true;
    }
}
