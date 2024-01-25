#![allow(unused_imports)]
use minifb::Key;
mod cpu;
mod registers;
mod display;


fn main() {

    /*
    let mut display = display::Display::new();

    display.update();
    let mut y = false;
    while display.window.is_open() && !display.window.is_key_down(Key::Escape) {
        let mut x;
        if y {
            x = 10;
        } else {
            x = 5;
        }
        for i in display.buffer.iter_mut() {
            if x > 5{
                *i = 0x000000;
            } else {
                *i = 0x444444;
            }
            x -= 1;
            if x <= 0 {
                x = 10;
            }
        }
        y = !y;

        display.update();
    }
    */

    let mut cpu = cpu::CPU::new();
    cpu.registers.a = 0b00100100;
    println!("a: {:b}", cpu.registers.a);
    cpu.memory[(cpu.registers.pc + 1) as usize] = 0x0F;
    cpu.run_instruction(0xCB);
    println!("a: {:b}", cpu.registers.a);

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
