use crate::registers::Register;

#[allow(dead_code)]
pub struct CPU {
    pub registers: Register,
    memory: [u8; 0xFFFF],
    halted: bool,
    ei: bool,

}
impl CPU {

    #[allow(dead_code)]
    pub fn new() -> CPU {
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
                sp: 0xFFFe,
                pc: 0x0,
            },
            memory: [0; 0xFFFF],
            halted: false,
            ei: false,
        }
    }

    // to make the compiler shut the fuck up
    #[allow(dead_code)]
    fn decode_register(&mut self, register: u8) -> &mut u8{
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
    fn fetch_instruction() {
        todo!();
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

    #[allow(dead_code)]
    pub fn run_instruction(&mut self, opcode: u8) {
        // TODO? I might make these individual functions that will get called
        // from a hashmap but i dont think this part is going to affect the 
        // overall performance. I will certainly do it if the performance is
        // shit but if it isn't i won't bother. I dont know if hashmap would
        // be faster.
        //


        let cycles: u8 = match opcode >> 6 {
            // I couldn't use a pattern in this part
            // so i will just make it manually
            0b00 => match opcode {
                0x00 => 4,
                0x10 => todo!(),

                // JUMPS
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


                // CALLS
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

                // RETURN
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

                // Other 8bit arithmetic operations
                0x03 => {
                    self.registers.set_bc(self.registers.get_bc() + 1);
                    todo!();
                },

                _ => panic!("opcode doesn't exist") 
            },

            // Load / Halt
            0b01 => {
                if opcode >> 4 == 0x7 {
                    // HALT
                    self.halted = true;
                    4
                }else {
                    // first register
                    let first = (opcode & 0b00111000) >> 3;
                    if opcode & 0xF == 0x6 || opcode & 0xF == 0xE {
                        //Load register from HL
                        *self.decode_register(first) = self.memory[self.registers.get_hl() as usize]; 
                        8
                    }else {
                        let second = opcode & 0b00000111;
                        if opcode & 0xF < 0x8 {
                            //Load register from immediate value
                            self.memory[self.registers.get_hl() as usize] = *self.decode_register(second);
                            8
                        }else {
                            //Load register from register
                            *self.decode_register(first) = *self.decode_register(second);
                            4
                        }
                    }
                }
            },

            0b10 => match opcode >> 4 & 0b0011 {
                //only get 4 and 5. bits to identify aritmetic operation 

                // ADD/ADC
                0b00 => {
                let op_cycles;
                let mut flag = 0;
                let sum: u16;
                if opcode > 0x87 {
                    // It's 87 and not 86 because 0x87 is ADD A, A 
                    // without the carry
                    if opcode == 0x8E {
                        // Add from HL with carry
                        let value = self.memory[self.registers.get_hl() as usize];
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
                    let value = self.memory[self.registers.get_hl() as usize];
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

            // SUB/SBC
            0b01 => {
                let op_cycles;
                let mut flag = 0b01000000;
                let value: u16;
                if opcode > 0x97 {
                    if opcode == 0x9E {
                        // subtract from HL with carry
                        value = self.memory[self.registers.get_hl() as usize] as u16;
                        op_cycles = 8;
                    } else {
                        // subtract from register with carry
                        value = (*self.decode_register(opcode & 0x07) + (self.registers.f >> 4) & 0b1) as u16;
                        op_cycles = 4;
                    }
                }else {
                    if opcode == 0x96 {
                        // subtract from HL
                        value = self.memory[self.registers.get_hl() as usize] as u16;
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

            // AND/XOR
            0b10 => {
                let op_cycles;
                if opcode > 0xA7 {
                    self.registers.f = 0;
                    if opcode == 0xAE {
                        // XOR HL
                        self.registers.a = self.registers.a ^ self.memory[self.registers.get_hl() as usize];
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
                        self.registers.a = self.memory[self.registers.get_hl() as usize] & self.registers.a;
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


            //OR/CP
            0b11 => {
                let op_cycles;
                if opcode > 0xB7 {
                    self.registers.f = 0b01000000;
                    let value;

                    // Get the value. Register or [HL] in memory.
                    if opcode == 0xBE {
                        // CP HL
                        value = self.memory[self.registers.get_hl() as usize];
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
                        self.registers.a |= self.memory[self.registers.get_hl() as usize];
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

        0b11 => {
            todo!();
        },


        _ => 0

    };
    println!("opcode: {:x} cycles: {}", opcode, cycles);

}



}

