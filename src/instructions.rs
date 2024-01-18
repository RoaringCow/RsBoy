use std::collections::HashMap;

pub type Address = u16;
pub type Register = u8;


//const INSTRUCTIN_MAP_1: HashMap<u8 ,Instruction>

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction { 
    NOP,
    Stop,
    JumpIFZreset,
    JumpIFZset,
    JumpIFCreset,
    JumpIFCset,

    LoadRegfromReg(Register, Register), // check if its reg
    LoadRegfromValue(Register, u8),
    LoadRegfromHL(Register),
    
    Load16bitRegfromInstant(Register),

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
    const INSTRUCTION_MAP_1: HashMap<i32, Instruction> = HashMap::from([
        (0x00, Instruction::NOP),
        (0x10, Instruction::Stop),
        (0x20, Instruction::JumpIFZreset),
        (0x30, Instruction::JumpIFCreset),

        (0x01, Instruction::Load16bitRegfromInstant(0b00)),
        (0x11, Instruction::Load16bitRegfromInstant(0b01)),
        (0x21, Instruction::Load16bitRegfromInstant(0b10)),
        (0x31, Instruction::Load16bitRegfromInstant(0b11)),

        (0x02, Instruction::),
        (0x12, Instruction::),
        (0x22, Instruction::),
        (0x32, Instruction::),


        (0x03, Instruction::),
        (0x13, Instruction::),
        (0x23, Instruction::),
        (0x33, Instruction::),


        (0x04, Instruction::),
        (0x14, Instruction::),
        (0x24, Instruction::),
        (0x34, Instruction::),

        (0x05, Instruction::),
        (0x15, Instruction::),
        (0x25, Instruction::),
        (0x35, Instruction::),

        (0x06, Instruction::),
        (0x16, Instruction::),
        (0x26, Instruction::),
        (0x36, Instruction::),

        (0x07, Instruction::),
        (0x17, Instruction::),
        (0x27, Instruction::),
        (0x37, Instruction::),

        (0x08, Instruction::),
        (0x18, Instruction::),
        (0x28, Instruction::),
        (0x38, Instruction::),

        (0x09, Instruction::),
        (0x19, Instruction::),
        (0x29, Instruction::),
        (0x39, Instruction::),
    ]);


    pub fn new(opcode: u8) -> Option<Instruction> {
        match opcode >> 6 {
            0b00 => {
                // I couldn't use a pattern in this part
                // so i will just use a hashmap
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
