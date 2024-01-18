pub type Address = u16;
pub type Register = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction { 
    LoadRegfromReg(Register, Register), // check if its reg
    LoadRegfromValue(Register, u8),
    LoadRegfromHL(Register),

    AddfromReg(Register),
    AddfromHL,
    AddfromRegwithCarry(Register),
    AddfromHLwithCarry,

    SubfromReg(Register),
    SubfromHL,
    SubfromRegwithCarry(Register),
    SubfromHLwithCarry,

    AndReg(Register),
    AndHL,

    XorReg(Register),
    XorHL,

    OrReg(Register),
    OrHL,

    CpReg(Register),
    CpHL,

    Halt,
}
impl Instruction {
    /*
        000 - Register B
        001 - Register C
        010 - Register D
        011 - Register E
        100 - Register H
        101 - Register L
        110 - Register HL (H + L)
        111 - Register A
    */
    /*
        Register Pair ss
        BC 00
        DE 01
        HL 10
        SP 11
    */
    pub fn new(opcode: u8) -> Option<Instruction> {
        match opcode >> 6 {
            0b00 => {
                todo!();
            },

            0b01 => {
                println!("{:b}", opcode);
                let first = (opcode & 0b00111000) >> 3;
                if opcode << 4 == 0x6 {
                    if opcode >> 4 == 0x7 {
                        return Some(Instruction::Halt);
                    }
                    return Some(Instruction::LoadRegfromHL(first)); 
                }
                let second = opcode & 0b00000111;
                println!("{} {:b}", first, second);
                
                return Some(Instruction::LoadRegfromReg(first, second));
            }

            0b10 => {
                match opcode >> 4 & 0b0011 {
                    //only get 4 and 5. bits 
                        
                    // ADD/ADC
                    0b00 => {
                        if opcode > 0x87 {
                            // It's 87 and not 86 because 0x87 is ADD A, A 
                            // without the carry
                            if opcode == 0x8E {
                               return Some(Instruction::AddfromHLwithCarry); 
                            }
                            return Some(Instruction::AddfromRegwithCarry(opcode & 0b111));

                        }
                        if opcode == 0x86 {
                            return Some(Instruction::AddfromHL);
                        }
                        return Some(Instruction::AddfromReg(opcode & 0b111));
                    },

                    // SUB/SBC
                    0b01 => {
                        if opcode > 0x97 {
                            if opcode == 0x9E {
                                return Some(Instruction::SubfromHLwithCarry);
                            }
                            return Some(Instruction::SubfromRegwithCarry(opcode & 0b111));
                        } 
                        if opcode == 0x96 {
                            return Some(Instruction::SubfromHL);
                        }
                        return Some(Instruction::SubfromReg(opcode & 0b111));
                    },

                    // AND/XOR
                    0b10 => {
                        if opcode > 0xA7 {
                            if opcode == 0xAE {
                                return Some(Instruction::XorHL);
                            }
                            return Some(Instruction::XorReg(opcode & 0b111));
                        }
                        if opcode == 0xA6 {
                            return Some(Instruction::AndHL);
                        }

                        return Some(Instruction::AndReg(opcode & 0b111));

                    },
                    //OR/CP
                    0b11 => {
                        if opcode > 0xB7 {
                            if opcode == 0xBE {
                                return Some(Instruction::CpReg(opcode & 0b111));
                            }
                            return Some(Instruction::CpHL);
                        }
                        if opcode == 0xB6 {
                            return Some(Instruction::OrHL);
                        }
                        return Some(Instruction::OrReg(opcode & 0b111));
                    }


                    _ => (),
                }
                panic!("error with 8bit arithmetic");
            },

            0b11 => {
                todo!();
            },


            _ => {
                return None;
            }

        }
    }
}
