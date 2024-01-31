use crate::registers::Register;
use crate::ppu::PPU;
use crate::cartridge::Cartridge;
#[allow(dead_code)]




pub struct CPU {
    pub registers: Register,
    pub memory: [u8; 0x2000],
    halted: bool,
    ei: bool,
    ppu: PPU,
    rom: Cartridge,
}
impl CPU {

    #[allow(dead_code)]
    pub fn new(game_rom: &str) -> CPU {
        CPU {

            registers: Register {
                a: 0x0,
                f: 0x0,
                b: 0x0,
                c: 0x0,
                d: 0x0,
                e: 0x0,
                h: 0x0,
                l: 0x0,
                sp: 0xFFFE, // not sure about this
                pc: 0x0000, //only to test
            },
            memory: [0; 0x2000],
            ppu: PPU::new(),
            halted: false,
            ei: false,
            rom: Cartridge::new(game_rom),
        }
    }

    // to make the compiler shut the fuck up
    #[allow(dead_code)]
    pub fn decode_register(&mut self, register: u8) -> &mut u8{
        return match register {
            0b000 => &mut self.registers.b,
            0b001 => &mut self.registers.c,
            0b010 => &mut self.registers.d,
            0b011 => &mut self.registers.e,
            0b100 => &mut self.registers.h,
            0b101 => &mut self.registers.l,
            0b111 => &mut self.registers.a,
            _ => {panic!("this register does not exist!")}
        };

    }

    #[allow(dead_code)]
    pub fn fetch_instruction(&self) -> u8{
        self.read_memory(self.registers.pc)
    }

    fn jump_8bitoffset(&mut self) {
        let offset = self.memory[(self.registers.pc + 1) as usize] as i16;
        self.registers.pc = (self.registers.pc as i16 + offset) as u16 + 1;
        // + 1 is there for value reading. It reads the next address after
        // the jump instruction to get the offset.
    }
    fn jump_16bitaddress(&mut self) {
        let lsb_address = self.memory[(self.registers.pc + 1) as usize] as u16;
        let msb_address = self.memory[(self.registers.pc + 2) as usize] as u16;
        self.registers.pc = msb_address << 8 | lsb_address;
    }

    fn call(&mut self) {
        self.registers.sp -= 1;
        self.memory[self.registers.sp as usize] = ((self.registers.pc + 3) & 0xFF) as u8;
        self.registers.sp -= 1;
        self.memory[self.registers.sp as usize] = ((self.registers.pc + 3) >> 8) as u8;
        self.jump_16bitaddress();
    }

    fn return_instruction(&mut self) {
        self.registers.sp += 1;
        let mut return_address: u16;
        return_address = self.memory[self.registers.sp as usize] as u16;
        self.registers.sp += 1;
        return_address |= (self.memory[self.registers.sp as usize] as u16) << 8;
        self.registers.sp += 1;
        self.registers.pc = return_address;
    }
    fn inc_flag_check(&mut self, value: u8) {
        let mut flag = 0;
        if value == 0 {
            flag |= 0b10000000;
        }
        if value & 0x0F == 0 {
            flag |= 0b00100000;
        }
        self.registers.f = flag;
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize], // ROM
            0x8000..=0x9FFF => self.ppu.read_vram(address), // VRAM
            0xA000..=0xBFFF => 0, // External RAM
            0xC000..=0xCFFF => self.memory[address as usize - 0xC000], // RAM
            0xE000..=0xFDFF => self.memory[address as usize - 0xE000], // Echo RAM
            0xFE00..=0xFE9F => todo!(),//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => 0,// not usable
            0xFF00..=0xFF7F => todo!(),// IO
            0xFF80..=0xFFFE => self.memory[address as usize - 0xFF80], // High RAM
            0xFFFF => 0, // Interrupt Enable Register
            _ => panic!("address out of range"),
        }
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        println!("reading memory at address: {:X}", address);
        match address {
            0x0000..=0x7FFF => self.rom.switch_bank(value), // ROM
            0x8000..=0x9FFF => self.ppu.write_vram(address, value), // VRAM
            0xA000..=0xBFFF => todo!(), // External RAM
            0xC000..=0xCFFF => self.memory[address as usize - 0xC000] = value, // RAM
            0xE000..=0xFDFF => self.memory[address as usize - 0xE000] = value, // Echo RAM
            0xFE00..=0xFE9F => todo!(),//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => todo!(),// not usable
            0xFF00..=0xFF7F => todo!(),// IO
            0xFF80..=0xFFFE => self.memory[address as usize - 0xFF80] = value, // High RAM
            0xFFFF => todo!(), // Interrupt Enable Register
            _ => panic!("address out of range"),
        };
    }


    #[allow(dead_code)]
    pub fn run_instruction(&mut self, opcode: u8) {
        // TODO? I might make these individual functions that will get called
        // from a hashmap but i dont think this part is going to affect the 
        // overall performance. I will certainly do it if the performance is
        // shit but if it isn't i won't bother. I dont know if hashmap would
        // be faster.

        println!("instruction: {}", opcode);
        let cycles: u8 = match opcode >> 6 {
            // I couldn't use a pattern in this part
            // so i will just make it manually


            // --------------------------------------------------------------------
            //                          0x00 --- 0x3F
            // --------------------------------------------------------------------
            0b00 => match opcode {
                0x00 => 4,
                0x10 => todo!(),

                // ----------------- Jumps ----------------------
                //Jumps with offset 8bit
                0x18 => {
                    self.jump_8bitoffset();
                    12
                },
                0x20 => {
                    // if Zero flag reset
                    if self.registers.f >> 7 == 0 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }

                },
                0x28 => {
                    // if zero flag set
                    if self.registers.f >> 7 == 1 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }
                },
                0x30 => {
                    // if carry flag reset
                    if self.registers.f >> 4 & 1 == 0 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }

                },
                0x38 => {
                    // if carry flag set
                    if self.registers.f >> 4 & 1 == 1 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }

                },

                // ------------------ ALU 8bit ------------------
                // INC r8
                0x04 => {
                    self.registers.b += 1;
                    self.inc_flag_check(self.registers.b);
                    4
                },
                0x0C => {
                    self.registers.c += 1;
                    self.inc_flag_check(self.registers.c);
                    4
                },
                0x14 => {
                    self.registers.d += 1;
                    self.inc_flag_check(self.registers.d);
                    4
                },
                0x1C => {
                    self.registers.e += 1;
                    self.inc_flag_check(self.registers.e);
                    4
                },
                0x24 => {
                    self.registers.h += 1;
                    self.inc_flag_check(self.registers.h);
                    4
                },
                0x2C => {
                    self.registers.l += 1;
                    self.inc_flag_check(self.registers.l);
                    4
                },
                0x3C => {
                    self.registers.a += 1;
                    self.inc_flag_check(self.registers.a);
                    4
                },
                0x34 => {
                    let value = self.read_memory(self.registers.get_hl());
                    self.write_memory(self.registers.get_hl(), value + 1);
                    self.inc_flag_check(value + 1);
                    12
                },

                // ------------------ ALU 16bit ------------------ 
                // INC
                0x03 => {
                    self.registers.set_bc(self.registers.get_bc() + 1);
                    8
                },
                0x13 => {
                    self.registers.set_de(self.registers.get_de() + 1);
                    8
                },
                0x23 => {
                    self.registers.set_hl(self.registers.get_hl() + 1);
                    8
                },
                0x33 => {
                    self.registers.sp += 1;
                    8
                },

                // DEC
                0x0B => {
                    self.registers.set_bc(self.registers.get_bc() - 1);
                    8
                },
                0x1B => {
                    self.registers.set_de(self.registers.get_de() - 1);
                    8
                },
                0x2B => {
                    self.registers.set_hl(self.registers.get_hl() - 1);
                    8
                },
                0x3B => {
                    self.registers.sp -= 1;
                    8
                },

                // ------------------ ADD HL, r16 ------------------
                // lots of repetition here but i dont care
                0x09 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.get_bc();
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },
                0x19 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.get_de();
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },
                0x29 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.get_hl();
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },
                0x39 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.sp;
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },


                // --------------------------------------------------------------
                // some load operations that are outside of the 0x40 - 0x7F range
                // --------------------------------------------------------------

                // ------------------ LD [a16], SP ------------------
                0x08 => {
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    let address = msb << 8 | lsb;
                    self.write_memory(address, self.registers.sp as u8);
                    self.write_memory(address + 1, (self.registers.sp >> 8) as u8);
                    20
                },

                // ------------------ LD r8, d8 ------------------
                0x06 => {
                    // load to B
                    self.registers.b = self.read_memory(self.registers.pc + 1);
                    8
                },
                0x0E => {
                    // add to C
                    self.registers.c = self.read_memory(self.registers.pc + 1);
                    8
                },
                0x16 => {
                    // add to D
                    self.registers.d = self.read_memory(self.registers.pc + 1);
                    8
                },
                0x1E => {
                    // add to E
                    self.registers.e = self.read_memory(self.registers.pc + 1);
                    8
                },
                0x26 => {
                    // add to H
                    self.registers.h = self.read_memory(self.registers.pc + 1);
                    8
                },
                0x2E => {
                    // add to L
                    self.registers.l = self.read_memory(self.registers.pc + 1);
                    8
                },
                0x36 => {
                    // add to address HL
                    let value = self.read_memory(self.registers.pc + 1);
                    self.write_memory(self.registers.get_hl(), value);
                    12
                },
                // ------------------ LD [r16], A ------------------
                0x02 => {
                    // load to BC
                    self.write_memory(self.registers.get_bc(), self.registers.a);
                    8
                },
                0x12 => {
                    // load to DE
                    self.write_memory(self.registers.get_de(), self.registers.a);
                    8
                },
                0x22 => {
                    // hl increment
                    self.write_memory(self.registers.get_hl(), self.registers.a);
                    self.registers.set_hl(self.registers.get_hl() + 1);
                    8
                },
                0x32 => {
                    // HL decrement
                    self.write_memory(self.registers.get_hl(), self.registers.a);
                    self.registers.set_hl(self.registers.get_hl() - 1);
                    8
                },

                // ------------------ LD A, [r16] ------------------
                0x0A => {
                    self.registers.a = self.read_memory(self.registers.get_bc());
                    8
                },
                0x1A => {
                    self.registers.a = self.read_memory(self.registers.get_de());
                    8
                },
                0x2A => {
                    // HL increment
                    self.registers.a = self.read_memory(self.registers.get_hl());
                    self.registers.set_hl(self.registers.get_hl() + 1);
                    8
                },
                0x3A => {
                    // HL decrement
                    self.registers.a = self.read_memory(self.registers.get_hl());
                    self.registers.set_hl(self.registers.get_hl() - 1);
                    8
                },


                // ------------------ LD r16, d16 ------------------
                0x01 => {
                    // load to BC
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    self.registers.set_bc(msb << 8 | lsb);
                    12
                },
                0x11 => {
                    // load to DE
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    self.registers.set_de(msb << 8 | lsb);
                    12
                },
                0x21 => {
                    // load to HL
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    self.registers.set_hl(msb << 8 | lsb);
                    12
                },
                0x31 => {
                    // load to SP
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    self.registers.sp = msb << 8 | lsb;
                    12
                },

                // ------------------ ROTATE ------------------
                // RLCA
                // Rotate A left. Carry flag is set to the bit that is shifted out
                // and the rightmost bit is set to the shifted out bit
                0x07 => {
                    let mut flag = 0;
                    if self.registers.a >> 7 == 1 {
                        flag |= 0b00010000;
                    }
                    self.registers.a = self.registers.a << 1 | self.registers.a >> 7;
                    self.registers.f = flag;
                    4
                },
                // RLA
                // Rotate A left through carry flag
                // Carry flag is set to the bit that is shifted out
                // and the bit that is shifted in is set to the carry flag
                0x17 => {
                    let mut flag = 0;
                    if self.registers.a >> 7 == 1 {
                        flag |= 0b00010000;
                    }
                    self.registers.a = self.registers.a << 1 | (self.registers.f >> 4) & 1;
                    self.registers.f = flag;
                    4
                },
                // RRCA
                // Rotate A right. Carry flag is set to the bit that is shifted out
                // and the leftmost bit is set to the shifted out bit
                0x0F => {
                    let mut flag = 0;
                    if self.registers.a & 1 == 1 {
                        flag |= 0b00010000;
                    }
                    self.registers.a = self.registers.a >> 1 | self.registers.a << 7;
                    self.registers.f = flag;
                    4
                },
                // RRA
                // Rotate A right through carry flag
                // Carry flag is set to the bit that is shifted out
                // and the bit that is shifted in is set to the carry flag
                0x1F => {
                    let mut flag = 0;
                    if self.registers.a & 1 == 1 {
                        flag |= 0b00010000;
                    }
                    self.registers.a = self.registers.a >> 1 | (self.registers.f >> 4) & 1 << 7;
                    self.registers.f = flag;
                    4
                },

                // ------------------ DAA ------------------
                // Decimal adjust register A
                // This instruction adjusts register A so that the
                // correct representation of Binary Coded Decimal (BCD)
                // is obtained.
                // Some weird stuff
                // -----------------------------------------
                0x27 => {
                    let mut offset = 0_u8;
                    let mut should_carry:u8 = 0;

                    let a_value = self.registers.a;
                    let half_carry = self.registers.f >> 5 & 1; 
                    let carry = self.registers.f >> 4 & 1;
                    let subtract = self.registers.f >> 6 & 1; 

                    if (subtract == 0 && a_value & 0xF > 0x09) || half_carry == 1 {
                        offset |= 0x06;
                    }

                    if (subtract == 0 && a_value > 0x99) || carry == 1 {
                        offset |= 0x60;
                        should_carry = 1;
                    }


                    let output = if subtract == 0 {
                        a_value.wrapping_add(offset)
                    } else {
                        a_value.wrapping_sub(offset)
                    };

                    self.registers.a = output;
                    self.registers.f = if output == 0 {0b10000000} else {0} 
                    | (should_carry << 4) | (self.registers.f & 0b01000000);
                    4 
                },

                // ------------------ SET CARRY FLAG ------------------
                0x37 => {
                    self.registers.f = (self.registers.f & 0b10000000) | 0b00010000;
                    4
                },

                // ------------------ COMPLEMENT CARRY FLAG ------------------
                0x3F => {
                    self.registers.f = (self.registers.f & 0b10000000) 
                        | ((self.registers.f & 0b00010000) ^ 0b00010000);
                    4
                },

                // ------------------ CPL ------------------
                0x2F => {
                    self.registers.a = !self.registers.a;
                    self.registers.f |= 0b01100000;
                    4
                },


                _ => panic!("opcode doesn't exist") 
            },





            // --------------------------------------------------------------------
            //                          0x40 --- 0x7F
            // --------------------------------------------------------------------
            // Load / Halt
            0b01 => {
                if opcode & 0x0F == 0x7 {
                    // HALT
                    self.halted = true;
                    4
                }else {
                    // first register
                    let first = (opcode & 0b00111000) >> 3;
                    if opcode & 0xF == 0x6 || opcode & 0xF == 0xE {
                        //Load register from HL
                        println!("load register from HL");
                        *self.decode_register(first) = self.read_memory(self.registers.get_hl()); 
                        8
                    }else {
                        let second = opcode & 0b00000111;
                        if opcode & 0xF < 0x8 {
                            //Load register from immediate value
                            let value = *self.decode_register(second);
                            self.write_memory(self.registers.get_hl(), value);
                            8
                        }else {
                            //Load register from register
                            *self.decode_register(first) = *self.decode_register(second);
                            4
                        }
                    }
                }
            },



            // --------------------------------------------------------------------
            //                          0x80 --- 0xBF
            // --------------------------------------------------------------------
            0b10 => match opcode >> 4 & 0b0011 {
                //only get 4 and 5. bits to identify aritmetic operation 

                // ------------------ ADD/ADC ------------------
                0b00 => {
                    let op_cycles;
                    let mut flag = 0;
                    let sum: u16;
                    if opcode > 0x87 {
                        // It's 87 and not 86 because 0x87 is ADD A, A 
                        // without the carry
                        if opcode == 0x8E {
                            // Add from HL addr with carry
                            let value = self.read_memory(self.registers.get_hl());
                            sum = value as u16 + self.registers.a as u16 + (self.registers.f as u16 & 0b00010000);

                            if value & 0x0F + self.registers.a & 0x0F > 0x0F {
                                // Set the half carry flag if the first 4 bits overflow.
                                flag |= 0b001;
                            }
                            op_cycles = 8;

                        }
                        else {
                            // get the register value to be added
                            let value = *self.decode_register(opcode & 0x07);
                            // add them up with a bigger size in order to see the carry
                            sum = self.registers.a as u16 + value as u16 + ((self.registers.f >> 4) & 1) as u16;
                            if (value & 0x0F) + self.registers.a & 0x0F + self.registers.f & 0b00010000 > 0x0F {
                                // Set the half carry flag if the first 4 bits overflow.
                                flag |= 0b001;
                            }
                            op_cycles = 4;
                        }

                    }else if opcode == 0x86 {
                        // Add from HL
                        let value = self.read_memory(self.registers.get_hl());
                        sum = value as u16 + self.registers.a as u16;

                        if value & 0x0F + self.registers.a & 0x0F > 0x0F {
                            flag |= 0b001;
                        }
                        op_cycles = 8;

                    } else {
                        // Add from Register
                        let value = *self.decode_register(opcode & 0x07);
                        sum = self.registers.a as u16 + value as u16;

                        if  & 0x0F + self.registers.a & 0x0F > 0x0F {
                            // Set the half carry flag if the first 4 bits overflow.
                            flag |= 0b001;
                        }
                        op_cycles = 4;
                    }


                    if sum > 0xFF {
                        flag |= 0b0001;
                    }else if sum == 0x0 {
                        flag |= 0b1;
                    }

                    self.registers.a = sum as u8;
                    self.registers.f = flag;

                    op_cycles

                },

                // ------------------ SUB/SBC ------------------
                0b01 => {
                    let op_cycles;
                    let mut flag = 0b01000000;
                    let value: u16;
                    if opcode > 0x97 {
                        if opcode == 0x9E {
                            // subtract from HL with carry
                            value = self.read_memory(self.registers.get_hl()) as u16;
                            op_cycles = 8;
                        } else {
                            // subtract from register with carry
                            value = (*self.decode_register(opcode & 0x07) + (self.registers.f >> 4) & 0b1) as u16;
                            op_cycles = 4;
                        }
                    }else {
                        if opcode == 0x96 {
                            // subtract from HL
                            value = self.read_memory(self.registers.get_hl()) as u16;
                            op_cycles = 8;
                        } else {
                            // subtract from register
                            value = *self.decode_register(opcode & 0x07) as u16;
                            op_cycles = 4;
                        }
                    }

                    if value > self.registers.a as u16 {
                        // set carry flag
                        flag |= 0b00010000;
                    } else if value as u8 == self.registers.a {
                        // set zero flag
                        flag |= 0b10000000;
                    }
                    if value as u8 & 0x0F > self.registers.a & 0x0F {
                        // set half carry flag
                        flag |= 0b00100000;
                    }

                    // update flag
                    self.registers.f = flag;
                    self.registers.a -= value as u8;

                    op_cycles
                },

                // ------------------ AND/XOR ------------------
                0b10 => {
                    let op_cycles;
                    if opcode > 0xA7 {
                        self.registers.f = 0;
                        if opcode == 0xAE {
                            // XOR HL
                            self.registers.a = self.registers.a ^ self.read_memory(self.registers.get_hl());
                            op_cycles = 8;
                        }else {
                            //XOR REGİSTER
                            self.registers.a = self.registers.a ^ *self.decode_register(opcode & 0x07);
                            op_cycles = 4;
                        }
                    }else {
                        self.registers.f = 0b00100000;
                        if opcode == 0xA6 {
                            // AND HL
                            self.registers.a = self.read_memory(self.registers.get_hl()) & self.registers.a;
                            op_cycles = 8;
                        }else {
                            // AND REGİSTER
                            self.registers.a = self.registers.a & *self.decode_register(opcode & 0x07);
                            op_cycles = 4;
                        }
                    }

                    if self.registers.a == 0 {
                        // set zero flag if zero
                        self.registers.f |= 0b10000000;
                    }

                    op_cycles
                },


                // ------------------ OR/CP ------------------
                0b11 => {
                    let op_cycles;
                    if opcode > 0xB7 {
                        self.registers.f = 0b01000000;
                        let value;

                        // Get the value. Register or [HL] in memory.
                        if opcode == 0xBE {
                            // CP HL
                            value = self.read_memory(self.registers.get_hl());
                            op_cycles = 8;

                        }else {
                            // CP Reg
                            value = *self.decode_register(opcode & 0x07);
                            op_cycles = 4;
                        }

                        if self.registers.a < value {
                            // set carry flag
                            self.registers.f |= 0b00010000;
                        }
                        // It will check zero even if the carry flag part was true
                        // but i dont really care there wont be any performance difference
                        if self.registers.a & 0x0f < value {
                            // set half carry flag
                            self.registers.f |= 0b00100000;
                        }else if self.registers.a == value{
                            // Set zero flag
                            self.registers.f |= 0b10000000;
                        }

                    }else {
                        self.registers.f = 0;
                        if opcode == 0xB6 {
                            // OR HL
                            self.registers.a |= self.read_memory(self.registers.get_hl());
                            op_cycles = 8;
                        }else {    
                            // Or Reg
                            self.registers.a |= *self.decode_register(opcode & 0x07);
                            op_cycles = 4;
                        }
                        if self.registers.a == 0 {
                            self.registers.f = 0b10000000;
                        }
                    }
                    op_cycles
                }


                _ => 0,
            },



            // --------------------------------------------------------------------
            //                          0xC0 --- 0xFF
            // --------------------------------------------------------------------
            0b11 => match opcode {

                // -------------------------------------------------------
                // arithmetic operations with A with 8 bit immediate value
                // -------------------------------------------------------
                // i would have made a function for this if i knew these would
                // take up so much space
                // ------------------ ADD A, d8 ------------------
                0xC6 => {
                    let value = self.read_memory(self.registers.pc + 1);
                    let mut flag = 0;
                    let sum = self.registers.a as u16 + value as u16;
                    if sum > 0xFF {
                        flag |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.a & 0x0F > 0x0F {
                        flag |= 0b00100000;
                    }
                    if sum == 0 {
                        flag |= 0b10000000;
                    }
                    self.registers.a = sum as u8;
                    self.registers.f = flag;
                    8
                },
                // ------------------ ADC A, d8 ------------------
                0xCE => {
                    let value = self.read_memory(self.registers.pc + 1);
                    let mut flag = 0;
                    let sum = self.registers.a as u16 + value as u16 + ((self.registers.f >> 4) & 1) as u16;
                    if sum > 0xFF {
                        flag |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.a & 0x0F + (self.registers.f >> 4) & 1 > 0x0F {
                        flag |= 0b00100000;
                    }
                    if sum == 0 {
                        flag |= 0b10000000;
                    }
                    self.registers.a = sum as u8;
                    self.registers.f = flag;
                    8
                },
                // ------------------ SUB A, d8 ------------------
                0xD6 => {
                    let value = self.read_memory(self.registers.pc + 1);
                    let mut flag = 0b01000000;
                    if value > self.registers.a {
                        flag |= 0b00010000;
                    }
                    if value & 0x0F > self.registers.a & 0x0F {
                        flag |= 0b00100000;
                    }
                    if value == self.registers.a {
                        flag |= 0b10000000;
                    }
                    self.registers.a -= value;
                    self.registers.f = flag;
                    8
                },
                // ------------------ SBC A, d8 ------------------
                0xDE => {
                    let value = self.read_memory(self.registers.pc + 1) + (self.registers.f >> 4) & 1;
                    let mut flag = 0b01000000;
                    if value > self.registers.a {
                        flag |= 0b00010000;
                    }
                    if value & 0x0F > self.registers.a & 0x0F {
                        flag |= 0b00100000;
                    }
                    if value == self.registers.a {
                        flag |= 0b10000000;
                    }
                    self.registers.a -= value;
                    self.registers.f = flag;
                    8

                },
                // ------------------ AND A, d8 ------------------
                0xE6 => {
                    let value = self.read_memory(self.registers.pc + 1);
                    self.registers.a &= value;
                    self.registers.f = 0b00100000;
                    if self.registers.a == 0 {
                        self.registers.f |= 0b10000000;
                    }
                    8
                },
                // ------------------ XOR A, d8 ------------------
                0xEE => {
                    let value = self.read_memory(self.registers.pc + 1);
                    self.registers.a ^= value;
                    self.registers.f = 0;
                    if self.registers.a == 0 {
                        self.registers.f = 0b10000000;
                    }
                    8
                },
                // ------------------ OR A, d8 ------------------
                0xF6 => {
                    let value = self.read_memory(self.registers.pc + 1);
                    self.registers.a |= value;
                    self.registers.f = 0;
                    if self.registers.a == 0 {
                        self.registers.f = 0b10000000;
                    }
                    8
                },
                // ------------------ CP A, d8 ------------------
                0xFE => {
                    let value = self.read_memory(self.registers.pc + 1);
                    let mut flag = 0b01000000;
                    if value > self.registers.a {
                        flag |= 0b00010000;
                    }
                    if value & 0x0F > self.registers.a & 0x0F {
                        flag |= 0b00100000;
                    }
                    if value == self.registers.a {
                        flag |= 0b10000000;
                    }
                    self.registers.f = flag;
                    8
                },


                // ------------------ RST ------------------

                // -----ADD SP with 8 bit immediate value-----
                0xE8 => {
                    let value = self.read_memory(self.registers.pc + 1);
                    self.registers.f = self.registers.f & 0b10000000;
                    let sum = self.registers.sp as i16 + value as i16;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.sp as u8 & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.sp = sum as u16;
                    16
                },

                // ------------------ JP [a16] ------------------
                // Jump to address 16 bit immediate value
                0xC2 => {
                    // if zero flag reset
                    if self.registers.f >> 7 == 0 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                },
                0xCA => {
                    // if zero flag set 
                    if self.registers.f >> 7 == 1 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                },
                0xD2 => {
                    if self.registers.f >> 4 & 1 == 0 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                },
                0xDA => {
                    if self.registers.f >> 4 & 1 == 1 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                }
                0xC3 => { // şişko kalp
                    self.jump_16bitaddress();
                    16
                },

                // Jump to address in HL
                0xE9 => {
                    self.registers.pc = self.registers.get_hl();
                    4
                },


                // ------------------ CALL ------------------
                // call 16 bit immediate value
                0xC4 => {
                    if self.registers.f >> 7 == 0 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xCC => {
                    if self.registers.f >> 7 == 1 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xD4 => {
                    if self.registers.f >> 4 & 1 == 0 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xDC => {
                    if self.registers.f >> 4 & 1 == 1 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xCD => {
                    self.call();
                    24
                },

                // ------------------ RETURN ------------------
                0xC9 => {
                    // return without condition
                    self.return_instruction();
                    16
                },
                0xC0 => {
                    // if zero flag reset
                    if self.registers.f >> 7 == 0 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                0xC8 => {
                    // if zero flag set
                    if self.registers.f >> 7 == 1 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                0xD0 => {
                    // if carry flag reset
                    if self.registers.f >> 4 & 1 == 0 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                0xD8 => {
                    // if carry flag set
                    if self.registers.f >> 4 & 1 == 1 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },

                // some load operations

                // ------------------ LDH [a8], A ------------------
                0xE0 => {
                    let address = self.read_memory(self.registers.pc + 1) as u16;
                    self.write_memory(0xFF00 + address, self.registers.a);
                    12
                },
                // ------------------ LDH A, [a8] ------------------
                0xF0 => {
                    let address = self.read_memory(self.registers.pc + 1) as u16;
                    self.registers.a = self.read_memory(0xFF00 + address);
                    12
                },

                // ------------------ LD [C], A ------------------
                0xE2 => {
                    self.write_memory(0xFF00 + self.registers.c as u16, self.registers.a);
                    8
                },
                // ------------------ LD A, [C] ------------------
                0xF2 => {
                    self.registers.a = self.read_memory(0xFF00 + self.registers.c as u16);
                    8
                },
                // ------------------ LD [a16], A ------------------
                0xEA => {
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    let address = msb << 8 | lsb;
                    self.write_memory(address, self.registers.a);
                    16
                },
                // ------------------ LD A, [a16] ------------------
                0xFA => {
                    let lsb = self.read_memory(self.registers.pc + 1) as u16;
                    let msb = self.read_memory(self.registers.pc + 2) as u16;
                    let address = msb << 8 | lsb;
                    self.registers.a = self.read_memory(address);
                    16
                },

                // ------------------ LD HL, SP+r8 ------------------
                0xF8 => {
                    let mut flag = 0;
                    let value = self.read_memory(self.registers.pc + 1);
                    self.registers.f = self.registers.f & 0b10000000;
                    let sum = self.registers.sp as i16 + value as i16;
                    if sum > 0xFF {
                        flag |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.sp as u8 & 0x0F > 0x0F {
                        flag |= 0b00100000;
                    }
                    self.registers.f = flag;
                    self.registers.set_hl(sum as u16);
                    12
                },

                // ------------------ LD SP, HL ------------------
                0xF9 => {
                    self.registers.sp = self.registers.get_hl();
                    8
                },

                // ------------------ PUSH -----------------
                0xC5 => {
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.c);
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.b);
                    16
                },
                0xD5 => {
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.e);
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.d);
                    16
                },
                0xE5 => {
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.l);
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.h);
                    16
                },
                0xF5 => {
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.f);
                    self.registers.sp -= 1;
                    self.write_memory(self.registers.sp as u16, self.registers.a);
                    16
                },


                // ------------------ POP ------------------
                0xC1 => {
                    self.registers.b = self.read_memory(self.registers.sp);
                    self.registers.c = self.read_memory(self.registers.sp + 1);
                    self.registers.sp += 2;
                    12
                },
                0xD1 => {
                    self.registers.d = self.read_memory(self.registers.sp);
                    self.registers.e = self.read_memory(self.registers.sp + 1);
                    self.registers.sp += 2;
                    12
                },
                0xE1 => {
                    self.registers.h = self.read_memory(self.registers.sp);
                    self.registers.l = self.read_memory(self.registers.sp + 1);
                    self.registers.sp += 2;
                    12
                },
                0xF1 => {
                    self.registers.a = self.read_memory(self.registers.sp);
                    self.registers.f = self.read_memory(self.registers.sp + 1);
                    self.registers.sp += 2;
                    12
                },

                // Miscellaneous
                // ------------------ DI ------------------
                0xF3 => {
                    self.ei = false;
                    4
                },
                // ------------------ EI ------------------
                0xFB => {
                    self.ei = true;
                    4
                },


                // ------------------ PREFIX CB ------------------
                0xCB => {
                    let cb_opcode = self.read_memory(self.registers.pc + 1);
                    let cycles_cb = self.run_cb_prefix(cb_opcode);

                    4 + cycles_cb
                },

                _ => 4
            },


            _ => 4

        };
        println!("opcode: {:x}, cycles: {}", opcode, cycles);
        self.registers.pc += cycles as u16;
    }


    fn run_cb_prefix(&mut self, cb_opcode: u8) -> u8 {


        let cycles_cb = match cb_opcode >> 6 {

            0b00 => match cb_opcode & 0xF0 {
                // ------------------ RLC/RRC ------------------
                // Rotate left/right. Carry flag is set to the bit that is shifted out
                // and the rightmost/leftmost bit is set to the shifted out bit
                // -----------------------------------------
                0x00 => {
                    let mut flag = 0;
                    let cycles_cb;
                    if cb_opcode < 0x08 {
                        //------------------ RLC ------------------
                        if cb_opcode & 0x0F == 0x06 {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value >> 7 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value << 1 | value >> 7);
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register >> 7 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register << 1 | *register >> 7;
                            cycles_cb = 8;
                        }
                        self.registers.f = flag;

                    }else {
                        //------------------ RRC ------------------
                        if cb_opcode & 0x0F == 0x0E {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value >> 1 | value << 7);
                            cycles_cb = 16; 
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register >> 1 | *register << 7;
                            cycles_cb = 8;
                        }
                    }
                    self.registers.f = flag;
                    cycles_cb
                },

                // ------------------ RL/RR ------------------
                // Rotate left/right through carry flag
                // Carry flag is set to the bit that is shifted out
                // and the bit that is shifted in is set to the carry flag
                // -----------------------------------------
                0x10 => {
                    let old_flag = self.registers.f;
                    let mut flag = 0;
                    let cycles_cb;
                    if cb_opcode < 0x18 {
                        //------------------ RL ------------------
                        if cb_opcode & 0x0F == 0x06 {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value >> 7 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value << 1 | (old_flag >> 4) & 1);
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register >> 7 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register << 1 | (old_flag >> 4) & 1;
                            cycles_cb = 8;
                        }
                    }else {
                        //------------------ RR ------------------
                        if cb_opcode & 0x0F == 0x0E {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value >> 1 | ((old_flag >> 4) & 1) << 7);
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register >> 1 | ((old_flag >> 4) & 1) << 7;
                            cycles_cb = 8;
                        }
                    }
                    self.registers.f = flag;
                    cycles_cb
                },

                // ------------------ SLA/SRA ------------------
                0x20 => {
                    let mut flag = 0;
                    let cycles_cb;
                    if cb_opcode < 0x28 {
                        //------------------ SLA ------------------
                        // Shift left. Carry flag is set to the bit that is shifted out
                        // and the rightmost bit is set to 0
                        // -----------------------------------------
                        if cb_opcode & 0x0F == 0x06 {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value >> 7 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value << 1);
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register >> 7 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register << 1;
                            cycles_cb = 8;
                        }
                    }else {
                        //------------------ SRA ------------------
                        // Shift right. Carry flag is set to the bit that is shifted out
                        // and the leftmost bit is not changed
                        // -----------------------------------------
                        if cb_opcode & 0x0F == 0x0E {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value >> 1 | value & 0b10000000);
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register >> 1 | *register & 0b10000000; 
                            cycles_cb = 8;
                        }
                    }
                    self.registers.f = flag;
                    cycles_cb

                },

                0x30 => {
                    if cb_opcode < 0x38 {
                        // ------------------ SWAP ------------------
                        // Swap the upper and lower nibbles of a register
                        // -----------------------------------------
                        let mut flag = 0;
                        let cycles_cb;
                        if cb_opcode & 0x0F == 0x06 {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            self.write_memory(self.registers.get_hl(), (value & 0x0F) << 4 | (value & 0xF0) >> 4);
                            if self.read_memory(self.registers.get_hl()) == 0 {
                                flag |= 0b10000000;
                            }
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            *register = (*register & 0x0F) << 4 | (*register & 0xF0) >> 4;
                            if *register == 0 {
                                flag |= 0b10000000;
                            }
                            cycles_cb = 8;
                        }
                        self.registers.f = flag; 
                        cycles_cb
                    }else {
                        // ------------------ SRL ------------------
                        // Shift right. Carry flag is set to the bit that is shifted out
                        // and the leftmost bit is set to 0
                        // -----------------------------------------
                        let mut flag = 0;
                        let cycles_cb;
                        if cb_opcode & 0x0F == 0x0E {
                            // address HL
                            let value = self.read_memory(self.registers.get_hl());
                            if value & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            self.write_memory(self.registers.get_hl(), value >> 1);
                            cycles_cb = 16;
                        }else {
                            // Register
                            let register = self.decode_register(cb_opcode & 0x07);
                            if *register & 1 == 1 {
                                flag |= 0b00010000;
                            }
                            *register = *register >> 1;
                            cycles_cb = 8;
                        }
                        self.registers.f = flag;
                        cycles_cb
                    }
                },

                _=> 4
            },


            // ------------------ BIT ------------------
            // Test bit in register
            // -----------------------------------------
            0b01 => {
                println!("BIT");
                self.registers.f = self.registers.f | 0b00100000;

                let cycles: u8;
                let bit_to_test = cb_opcode >> 3 & 0x07;
                let value_to_test = {
                    if cb_opcode & 0x0F == 0b110 {
                        // address HL
                        cycles = 16;
                        self.read_memory(self.registers.get_hl())
                    }else {
                        // Register
                        cycles = 8;
                        *self.decode_register(cb_opcode & 0x07)
                    }
                };
                // value
                if value_to_test >> bit_to_test & 1 == 0 {
                    self.registers.f |= 0b10000000;
                }
                cycles
            },

            // ------------------ RES ------------------
            // Reset bit in register
            // -----------------------------------------
            0b10 => {
                println!("RES");
                let cycles: u8;
                let bit_to_reset = cb_opcode >> 3 & 0x07;
                if cb_opcode & 0x0F == 0b110 {
                    // address HL
                    cycles = 16;
                    let value = self.read_memory(self.registers.get_hl()) & !(0b10000000 >> bit_to_reset);
                    self.write_memory(self.registers.get_hl(), value);
                }else {
                    // Register
                    cycles = 8;
                    *self.decode_register(cb_opcode & 0x07) &= !(0b10000000 >> bit_to_reset);
                }
                // register
                cycles
            },

            // ------------------ SET ------------------
            // Set bit in register
            // -----------------------------------------
            0b11 => {
                println!("SET");
                let cycles: u8;
                let bit_to_set = cb_opcode >> 3 & 0x07;
                if cb_opcode & 0x0F == 0b110 {
                    // address HL
                    cycles = 16;
                    let value = self.read_memory(self.registers.get_hl()) | 0b10000000 >> bit_to_set;
                    self.write_memory(self.registers.get_hl(), value);
                }else {
                    // Register
                    cycles = 8;
                    *self.decode_register(cb_opcode & 0x07) |= 0b10000000 >> bit_to_set;
                }
                cycles
            },


            _=> 4
        };
        cycles_cb
    }



}

