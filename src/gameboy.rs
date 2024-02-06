use super::cpu::CPU;
use super::memory::Memory;

pub struct Gameboy {
    pub cpu: CPU,
    pub memory: Memory,
}
impl Gameboy {
    pub fn powerup(rom: Vec<u8>) -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(rom),
        }
    }
    pub fn run(&mut self) {
        loop {
            let cycles = self.cpu.step(&mut self.mmu);
            self.mmu.step(cycles);
        }
    }
}
