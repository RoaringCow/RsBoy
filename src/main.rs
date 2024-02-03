#![allow(unused_imports)]
use minifb::Key;
mod cpu;
mod registers;
mod ppu;
mod memory;
mod gameboy_io;

use std::fs::File;
use std::io::{self, Read};


use std::{thread, time};

#[allow(dead_code)]
const ADDRESS: &str = "/home/ersan/İndirilenler/tetris.gb";

fn main() {

    //let mut cpu = cpu::CPU::new("/home/ersan/rs_boy/test_roms/cpu_instrs/cpu_instrs.gb");
    let mut cpu = cpu::CPU::new("/home/ersan/rs_boy/test_roms/my_test.gb");
    
    /*
    let mut display = ppu::PPU::new();

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
        
        cpu.run_instruction(cpu.fetch_instruction());
        println!("Registers a: {:X}, b:{:X}, c:{:X}, d:{:X}, e:{:X}, f:{:X}, h:{:X}, l:{:X}, ppc:{:X} sp:{:X}", cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.f, cpu.registers.h, cpu.registers.l,  cpu.registers.pc, cpu.registers.sp);
        display.update();
        thread::sleep(time::Duration::from_millis(1000));
    }
    */
    loop {
        cpu.run_instruction(cpu.fetch_instruction());
        println!("Registers a: {:X}, b:{:X}, c:{:X}, d:{:X}, e:{:X}, f:{:X}, h:{:X}, l:{:X}, sp:{:X} pc:{:X}", cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.f, cpu.registers.h, cpu.registers.l,  cpu.registers.sp, cpu.registers.pc);
        thread::sleep(time::Duration::from_millis(100));
    }
    


    

}






#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.set_hl(0xC000);
        cpu.memory.write_memory(cpu.registers.get_hl(), 0x01);
        cpu.run_instruction(0x7E);
        assert_eq!(cpu.registers.a, 0x01);
    }
    #[test]
    fn test_and() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10101010;
        cpu.registers.b = 0b01010101;
        cpu.run_instruction(0xA0);
        assert_eq!(0b00000000, cpu.registers.a);
        assert_eq!(cpu.registers.f >> 4, 0b1010);
    }

    #[test]
    fn test_or() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10101010;
        cpu.registers.c = 0b01010101;
        cpu.run_instruction(0xB1);
        assert_eq!(cpu.registers.a, 0b11111111);
        assert_eq!(cpu.registers.f >> 4 ,0);
    }

    #[test]
    fn test_xor() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10101010;
        cpu.registers.c = 0b11111111;
        cpu.run_instruction(0xA9);
        assert_eq!(cpu.registers.a, 0b01010101);
        assert_eq!(cpu.registers.f >> 4, 0);
    }

    #[test]
    fn test_cp() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10101010;
        cpu.registers.c = 0b11111111;
        cpu.run_instruction(0xB9);
        assert_eq!(cpu.registers.f >> 4, 0b0111);
    }
    
    #[test]
    fn test_rrc() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b00100100;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x0F);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b00010010);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b00100100;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x07);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b01001000);
    }

    #[test]
    fn test_rl() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b00100100;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x17);
        cpu.registers.f = 0b00010000;
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b01001001);
    }

    #[test]
    fn test_rr() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b00100100;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x1F);
        cpu.registers.f = 0b00010000;
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b10010010);
    }

    #[test]
    fn test_sla() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100100;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x27);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b01001000);
        assert_eq!(cpu.registers.f >> 4, 0b0001);
    }

    #[test]
    fn test_sra() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100101;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x2F);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b11010010);
        assert_eq!(cpu.registers.f >> 4, 0b0001);
    }

    #[test]
    fn test_swap() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100101;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x37);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b01011010);
        assert_eq!(cpu.registers.f >> 4, 0);
    }

    #[test]
    fn test_srl() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100101;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x3F);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b01010010);
        assert_eq!(cpu.registers.f >> 4, 1);
    }

    #[test]
    fn test_bit() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100101;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x47);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.f >> 4 & 1, 0);
    }

    #[test]
    fn test_res() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100101;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0x87);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b00100101);
    }

    #[test]
    fn test_set() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.registers.a = 0b10100101;
        cpu.memory.write_memory(cpu.registers.pc + 1, 0xC7);
        cpu.run_instruction(0xCB);
        assert_eq!(cpu.registers.a, 0b10100101);
    }



}
