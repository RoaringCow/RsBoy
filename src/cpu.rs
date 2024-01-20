use crate::registers::Register;

pub struct CPU {
    pub registers: Register,
    memory: [u8; 0xFFFF],
    halted: bool,
    ei: bool,

}
impl CPU {


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

    fn fetch_instruction() {
        
    }
     
    pub fn run_instruction(&mut self, opcode: u8) {
        
        match opcode >> 6 {
            0b00 => {
                // I couldn't use a pattern in this part
                // so i will just make it manually
                todo!();

            },

            // Load / Halt
            0b01 => {
                // first register
                let first = (opcode & 0b00111000) >> 3;
                if opcode & 0xF == 0x6 || opcode & 0xF == 0xE {
                    if opcode >> 4 == 0x7 {
                        // HALT
                        self.halted = true;
                    }else {
                        //Load register from HL
                        *self.decode_register(first) = self.memory[self.registers.get_hl() as usize];  
                    }
                }
                else {
                    let second = opcode & 0b00000111;
                    //Load register from register
                    *self.decode_register(first) = *self.decode_register(second);
                }
            },

            0b10 => {
                match opcode >> 4 & 0b0011 {
                    //only get 4 and 5. bits to identify aritmetic operation 
                        
                    // ADD/ADC
                    0b00 => {
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
                            }

                        }else if opcode == 0x86 {
                            // Add from HL
                            let value = self.memory[self.registers.get_hl() as usize];
                            sum = value as u16 + self.registers.a as u16;
                    
                            if value & 0x0F + self.registers.a & 0x0F > 0x0F {
                                flag |= 0b001;
                            }


                        } else {
                            // Add from Register
                            let value = *self.decode_register(opcode & 0x0F);
                            sum = self.registers.a as u16 + value as u16;

                            if  & 0x0F + self.registers.a & 0x0F > 0x0F {
                                // Set the half carry flag if the first 4 bits overflow.
                                flag |= 0b001;
                            }
                        }


                        if sum > 0xFF {
                            flag |= 0b0001;
                        }else if sum == 0x0 {
                            flag |= 0b1;
                        }

                        self.registers.a = sum as u8;
                        self.registers.f = flag;
                    

                    },

                    // SUB/SBC
                    0b01 => {
                        if opcode > 0x97 {
                            if opcode == 0x9E {
                                // subtract from HL with carry
                            }
                            // subtract from register with carry
                        } 
                        if opcode == 0x96 {
                            // subtract from HL 
                        }
                        // subtract from register
                    },

                    // AND/XOR
                    0b10 => {
                        if opcode > 0xA7 {
                            if opcode == 0xAE {
                                // XOR HL
                            }
                            //XOR REGİSTER
                        }
                        if opcode == 0xA6 {
                            // AND HL
                        }
                        // AND REGİSTER

                    },
                    //OR/CP
                    0b11 => {
                        if opcode > 0xB7 {
                            if opcode == 0xBE {
                                // CP Reg
                            }
                            // CP HL
                        }
                        if opcode == 0xB6 {
                            // OR HL
                        }
                        // Or Reg
                    }


                    _ => (),
                }
            },

            0b11 => {
                todo!();
            },


            _ => {
            }

        }

        }



    }

