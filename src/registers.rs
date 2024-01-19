


pub struct Register {
    pub a: u8,
    pub f: u8, // The F register is indirectly accessible by the programer.
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}
impl Register {

    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }
    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }
    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }
    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }
    pub fn set_af(&mut self, input: u16) {
        self.a = (input >> 8) as u8;
        self.f = (input & 0xF0) as u8; // f register doesnt allow writing
                                       // to bits 0-3
    }
    pub fn set_bc(&mut self, input: u16) {
        self.b = (input >> 8) as u8;
        self.c = (input & 0xFF) as u8;
    }
    pub fn set_de(&mut self, input: u16) {
        self.d = (input >> 8) as u8;
        self.e = (input & 0xFF) as u8;
    }
    pub fn set_hl(&mut self, input: u16) {
        self.h = (input >> 8) as u8;
        self.l = (input & 0xFF) as u8;
    }
}
