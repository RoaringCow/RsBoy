

pub struct IO {
    io_registers: [u8; 0x7f],
}
impl IO {
    pub fn new() -> IO {
        IO {
            io_registers: [0; 0x7f],
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.io_registers[addr as usize - 0xFF00]
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        self.io_registers[addr as usize - 0xFF00] = value;
    }
}
