pub type Address = u16;
pub type Register = u8;

#[derive(Debug)]
pub enum Instruction { 
    LoadRegfromReg(Register, Register), // check if its reg
    LoadRegfromValue(Register, u8),
    AddfromReg(Register),
    AddfromHL(),
    AddfromRegwithCarry(Register),
    AddfromAddrwithCarry(),
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
               return Some(Instruction::Halt); 
            },
            0b01 => {
                let first = (opcode & 0b00111000) >> 3;
                let second = opcode << 5;
                if first == second && first == 0b110 {
                    return Some(Instruction::Halt);
                }
                return Some(Instruction::LoadRegfromReg(first, second));

                
            },
            0b10 => {
                match opcode >> 4 & 0b0011 {
                    //only get 4 and 5. bits 
                        
                    // ADD/ADC
                    0b00 => {
                        if opcode > 0x87 {
                            // It's 87 and not 86 because 0x87 is ADD A, A 
                            // without the carry
                            
                            return Some(Instruction::AddfromRegwithCarry(opcode << 5));

                        }
                        if opcode == 0x86 {
                            return Some(Instruction::AddfromHL());
                        }
                        return Some(Instruction::AddfromReg(opcode << 5));
                        
                    }




                    _ => (),
                }
                return Some(Instruction::Halt); 
            },
            0b11 => {
               return Some(Instruction::Halt); 
            },
            _ => {
                return None;
            }

        }
    }
}
