
const WIDTH: usize = 640;
const HEIGHT: usize = 360;


pub struct PPU {
    pub buffer: [u32; WIDTH * HEIGHT], // Change type to array
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
    }

    // Create a new display
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
            cycle: 0,
            lcdc: 0,
            stat: 0,
            scy: 0, 
            scx: 0,
            ly: 0, 
            lyc: 0,
        }
    }


}
