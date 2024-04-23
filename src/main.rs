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

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const SCALE: usize = 3;

#[allow(dead_code)]
// on linux
//const ADDRESS: &str = "/home/ersan/rs_boy/test_roms/emptyfortests.gb";
// on Mac
const ADDRESS: &str = "/Users/ersandemircan/rs_boy/test_roms/emptyfortests.gb";

fn main() {

    // last developed
    // on linux
    //let mut cpu = cpu::CPU::new("/home/ersan/rs_boy/test_roms/emptyfortests.gb");
    // on Mac
    let mut cpu = cpu::CPU::new(ADDRESS);


    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(time::Duration::from_micros(16740)));
    //window.limit_update_rate(Some(time::Duration::from_micros(1674)));





    // test background

    /*
       cpu.memory.write_memory(0x8000, 0b01010101);
       cpu.memory.write_memory(0x8002, 0b10101010);
       cpu.memory.write_memory(0x8004, 0b01010101);
       cpu.memory.write_memory(0x8006, 0b10101010);
       cpu.memory.write_memory(0x8008, 0b01010101);
       cpu.memory.write_memory(0x800A, 0b10101010);
       cpu.memory.write_memory(0x800C, 0b01010101);
       cpu.memory.write_memory(0x800E, 0b10101010);
       */

    // white
    cpu.memory.write_memory(0x8000, 0b11111111);
    cpu.memory.write_memory(0x8001, 0b11111111);
    cpu.memory.write_memory(0x8002, 0b11111111);
    cpu.memory.write_memory(0x8003, 0b11111111);
    cpu.memory.write_memory(0x8004, 0b11111111);
    cpu.memory.write_memory(0x8005, 0b11111111);
    cpu.memory.write_memory(0x8006, 0b11111111);
    cpu.memory.write_memory(0x8007, 0b11111111);
    cpu.memory.write_memory(0x8008, 0b11111111);
    cpu.memory.write_memory(0x8009, 0b11111111);
    cpu.memory.write_memory(0x800A, 0b11111111);
    cpu.memory.write_memory(0x800B, 0b11111111);
    cpu.memory.write_memory(0x800C, 0b11111111);
    cpu.memory.write_memory(0x800D, 0b11111111);
    cpu.memory.write_memory(0x800E, 0b11111111);
    cpu.memory.write_memory(0x800F, 0b11111111);

    // little less white

    cpu.memory.write_memory(0x8010, 0b00000000);
    cpu.memory.write_memory(0x8011, 0b11111111);
    cpu.memory.write_memory(0x8012, 0b00000000);
    cpu.memory.write_memory(0x8013, 0b11111111);
    cpu.memory.write_memory(0x8014, 0b00000000);
    cpu.memory.write_memory(0x8015, 0b11111111);
    cpu.memory.write_memory(0x8016, 0b00000000);
    cpu.memory.write_memory(0x8017, 0b11111111);
    cpu.memory.write_memory(0x8018, 0b00000000);
    cpu.memory.write_memory(0x8019, 0b11111111);
    cpu.memory.write_memory(0x801A, 0b00000000);
    cpu.memory.write_memory(0x801B, 0b11111111);
    cpu.memory.write_memory(0x801C, 0b00000000);
    cpu.memory.write_memory(0x801D, 0b11111111);
    cpu.memory.write_memory(0x801E, 0b00000000);
    cpu.memory.write_memory(0x801F, 0b11111111);


    // a little less white

    cpu.memory.write_memory(0x8020, 0b11111111);
    cpu.memory.write_memory(0x8021, 0b00000000);
    cpu.memory.write_memory(0x8022, 0b11111111);
    cpu.memory.write_memory(0x8023, 0b00000000);
    cpu.memory.write_memory(0x8024, 0b11111111);
    cpu.memory.write_memory(0x8025, 0b00000000);
    cpu.memory.write_memory(0x8026, 0b11111111);
    cpu.memory.write_memory(0x8027, 0b00000000);
    cpu.memory.write_memory(0x8028, 0b11111111);
    cpu.memory.write_memory(0x8029, 0b00000000);
    cpu.memory.write_memory(0x802A, 0b11111111);
    cpu.memory.write_memory(0x802B, 0b00000000);
    cpu.memory.write_memory(0x802C, 0b11111111);
    cpu.memory.write_memory(0x802D, 0b00000000);
    cpu.memory.write_memory(0x802E, 0b11111111);
    cpu.memory.write_memory(0x802F, 0b00000000);


    // black

    cpu.memory.write_memory(0x8030, 0b00000000);
    cpu.memory.write_memory(0x8031, 0b00000000);
    cpu.memory.write_memory(0x8032, 0b00000000);
    cpu.memory.write_memory(0x8033, 0b00000000);
    cpu.memory.write_memory(0x8034, 0b00000000);
    cpu.memory.write_memory(0x8035, 0b00000000);
    cpu.memory.write_memory(0x8036, 0b00000000);
    cpu.memory.write_memory(0x8037, 0b00000000);
    cpu.memory.write_memory(0x8038, 0b00000000);
    cpu.memory.write_memory(0x8039, 0b00000000);
    cpu.memory.write_memory(0x803A, 0b00000000);
    cpu.memory.write_memory(0x803B, 0b00000000);
    cpu.memory.write_memory(0x803C, 0b00000000);
    cpu.memory.write_memory(0x803D, 0b00000000);
    cpu.memory.write_memory(0x803E, 0b00000000);
    cpu.memory.write_memory(0x803F, 0b00000000);


    for x in 0x9800..0x9C00 {
        cpu.memory.write_memory(x, ((x+1) %4) as u8);
    }
    for x in 0x9800..0x9C00 {
        println!("{:x}, {}", x, cpu.memory.read_memory(x));
    }
    // -----------------

    cpu.memory.ppu.oam[0] = 16;
    cpu.memory.ppu.oam[1] = 40;
    cpu.memory.ppu.oam[2] = 1;

    let mut x = 0;
    let mut now = time::Instant::now();

    //let mut test = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.run_instruction(cpu.fetch_instruction());
        if x == 35112{
            cpu.memory.ppu.update_display();
            window.update_with_buffer(&cpu.memory.ppu.buffer, WIDTH, HEIGHT).unwrap();
            println!("display updated in: {:?}", now.elapsed());
            now = time::Instant::now();
            /*
            for a in 0x9800..0x9C00 {
                cpu.memory.write_memory(a, ((a+ test % 4) as u64 %4) as u8);
            }
            test += 1;
            */
            x = 0;
        }


        x += 1;
    }


    let mut cpu = cpu::CPU::new("/Users/ersandemircan/rs_boy/test_roms/emptyfortests.gb");
    cpu.memory.write_memory(0x9000, 0x31);
    cpu.memory.write_memory(0x9800, 0x00);   
}






#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /*
       #[test]
       fn test_fetch_coordinate() {
       let mut cpu = cpu::CPU::new(ADDRESS);
       cpu.memory.write_memory(0x9C00, 0x01);
       cpu.memory.ppu.lcdc = 0b10001000;
       cpu.memory.ppu.tick();
       }
       */


    #[test]
    fn test_tile_data_merge() {

        let tile_data_low: u16 = 0b01010101;
        let tile_data_high: u16 = 0b10101010;
        let mut tile_data: u16 = 0;
        for i in 0..8 {
            let bit = ((tile_data_low >> (7 - i)) & 1) << 1 | ((tile_data_high >> (7 - i)) & 1);
            tile_data |= bit << (i * 2);
        }
        assert_eq!(tile_data, 0b1001100110011001);
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
