pub struct Timer {

    // ------ registers --------
    // FF04 DIV: Divider register
    div: u8,
    // FF05 TIMA: Timer counter
    tima: u8,
    // FF06 TMA: timer modulo
    tma: u8,
    // FF07 TAC: timer control
    tac: u8,
    // -------------------------
    
    // for increasing tac every n cycles 
    counter: u8,

    tac_increment_cycle: u8,
    tac_enable: bool,

}

impl Timer {
    pub fn power_up(&mut self) -> Self {
        Timer {}
    }

    pub fn tick(&mut self) {
        self.div += 1;



        self.counter += 1;
        if self.counter > self.tac_increment_cycle {
            self.tima += 1;
            self.counter = 0
        }
    }
}
