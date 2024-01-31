
pub struct PPU {
    vram: [u8; 0x2000],

}
impl PPU {
    pub fn new() -> PPU {
        PPU {
            vram: [0; 0x2000],
        }
    }
    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }
    pub fn write_vram(&mut self, addr: u16, data: u8) {
        self.vram[addr as usize] = data;
    }
}
