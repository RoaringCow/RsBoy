mod instructions;
mod cpu;



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


    let a: u8 = 0x46;
    println!("{:?}", instructions::Instruction::new(a));

}





#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::instructions::Instruction; 

    #[test]
    fn test_instructions() {
        assert_eq!(instructions::Instruction::new(0x46), Some(Instruction::LoadRegfromHL(0)));
        //assert_eq!(instructions::Instruction::new(0x76), Some(Instruction::Halt));
        assert_eq!(instructions::Instruction::new(0x86), Some(Instruction::AddfromHL));
    }

}
