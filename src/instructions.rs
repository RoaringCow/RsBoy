pub type Address = u16;
pub type Register = usize;

#[derive(Debug)]
pub enum Instruction { 
    LoadRegfromReg(Register, Register), // check if its reg
    LoadRegfromValue(Register, u8),
    
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
                let first: usize = ((opcode & 0b00111000) >> 3) as usize;
                let second: usize = (opcode & 0b00000111) as usize;
                if first == second && first == 0b110 {
                    return Some(Instruction::Halt);
                }
                return Some(Instruction::LoadRegfromReg(first, second));

                
            },
            0b10 => {
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
