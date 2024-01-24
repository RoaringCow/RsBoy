mod cpu;
mod registers;


// Framerate
// - 59.727500569606 Hz

// CPU
// - Custom 8-bit Sharp LR35902 (based on modified 8080 and Z80) at 4.19 MHz

// Memory
// - 64 KB address space including:
//   - 8 KB of built-in working RAM
//   - Up to sixteen 8 KB switchable working RAM pages (in game cartridge)
//   - 8 KB RAM for LCD display
//   - 32 KB external Game Pak ROM (16 KB switchable)
// - On-CPU-Die 256-byte bootstrap ROM
// - Support for 32 KB to 8 MB cartridges

// Resolution
// - 160 (w) × 144 (h) pixels (10:9 aspect ratio)

// Color Support
// - 2-bit (four shades of "gray": light to very dark olive green)
// - Original color scheme: 0x0 0x1 0x2 0x3
// - Pocket/Light color scheme: 0x0 0x1 0x2 0x3

// Sound
// - 2 pulse wave generators, 1 PCM 4-bit wave sample channel, 1 noise generator
// - Audio input from cartridge
// - One speaker, stereo output through headphone port

// Input
// - Eight-way control pad
// - Four action buttons (A, B, Start, Select)
// - Volume and contrast potentiometers
// - Power switch
// - Serial I/O ("Link cable"): 512 kbit/s with up to 4 connections in serial
// - Cartridge I/O



fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.registers.a = 0x05;
    cpu.registers.b = 0x21;
    println!("{:x} {:x}", cpu.registers.a, cpu.registers.b);
    cpu.run_instruction(0x90);
    println!("{:x}", cpu.registers.a);
    cpu.run_instruction(0x27);
    println!("{:x}", cpu.registers.a);
    println!("{:b}", cpu.registers.f);
    cpu.run_instruction(0x3F);
    println!("{:b}", cpu.registers.f);
}





#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_and() {
        let mut cpu = cpu::CPU::new();
        cpu.registers.a = 0b10101010;
        cpu.registers.b = 0b01010101;
        cpu.run_instruction(0xA0);
        assert_eq!(0b00000000, cpu.registers.a);
        assert_eq!(cpu.registers.f >> 4, 0b1010);
    }
    
    #[test]
    fn test_or() {
        let mut cpu = cpu::CPU::new();
        cpu.registers.a = 0b10101010;
        cpu.registers.c = 0b01010101;
        cpu.run_instruction(0xB1);
        assert_eq!(cpu.registers.a, 0b11111111);
        assert_eq!(cpu.registers.f >> 4 ,0);
    }

    #[test]
    fn test_xor() {
        let mut cpu = cpu::CPU::new();
        cpu.registers.a = 0b10101010;
        cpu.registers.c = 0b11111111;
        cpu.run_instruction(0xA9);
        assert_eq!(cpu.registers.a, 0b01010101);
        assert_eq!(cpu.registers.f >> 4, 0);
    }

    #[test]
    fn test_cp() {
        let mut cpu = cpu::CPU::new();
        cpu.registers.a = 0b10101010;
        cpu.registers.c = 0b11111111;
        cpu.run_instruction(0xB9);
        assert_eq!(cpu.registers.f >> 4, 0b0111);
    }

    // Todo test addition

}
