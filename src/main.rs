#![allow(unused_imports)]
use minifb::{Key, Window, WindowOptions};
mod cpu;
mod registers;
mod ppu;
mod memory;
mod gameboy_io;

use std::fs::File;
use std::io::{self, Read};


use std::{thread, time};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

#[allow(dead_code)]
const ADDRESS: &str = "/home/ersan/rs_boy/test_roms/emptyfortests.gb";

fn main() {

    //let mut cpu = cpu::CPU::new("/home/ersan/rs_boy/test_roms/my_test.gb");
    let mut cpu = cpu::CPU::new("/Users/ersandemircan/rs_boy/test_roms/emptyfortests.gb");
    
    
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    window.limit_update_rate(Some(std::time::Duration::from_millis(1000/5)));
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //cpu.run_instruction(cpu.fetch_instruction());
        window.update_with_buffer(&cpu.memory.ppu.buffer, WIDTH, HEIGHT).unwrap();
        thread::sleep(time::Duration::from_millis(100));
        
        
        
    }

   /*
   let tile_data = 0b0000010010000000;
   let first_byte: u8 = (tile_data >> 8) as u8;
   let second_byte: u8 = tile_data as u8; 
   for i in 0..8 {
       let color = (first_byte >> (7 - i) & 1) | ((second_byte >> (7 - i)) & 1) << 1 ;
       println!("Color: {:b}", color);
    }
    */ 
    /*
    let mut cpu = cpu::CPU::new("/Users/ersandemircan/rs_boy/test_roms/emptyfortests.gb");
    cpu.memory.write_memory(0x9000, 0x31);
    cpu.memory.write_memory(0x9800, 0x00);   
    cpu.memory.ppu.tick();
    */
}






#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fetch_coordinate() {
        let mut cpu = cpu::CPU::new(ADDRESS);
        cpu.memory.write_memory(0x9C00, 0x01);
        cpu.memory.ppu.lcdc = 0b10001000;
        cpu.memory.ppu.tick();
    }
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
