use crate::registers::Register;

struct CPU {
    registers: Register,
    memory: [u8; 0xFFFF],
    halted: bool,
    ei: bool,

}
impl CPU {


    fn new() -> CPU {
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

    fn fetch_instruction() {
        
    }
    
    fn run_instruction(&mut self, opcode: u8) {
        
        match opcode >> 6 {
            0b00 => {
                // I couldn't use a pattern in this part
                // so i will just make it manually
                todo!();

            },

            0b01 => {
                let first = (opcode & 0b00111000) >> 3;
                if opcode << 4 == 0x6 {
                    if opcode >> 4 == 0x7 {
                        // HALT
                    }
                    //Load register from HL
                }
                let second = opcode & 0b00000111;
                //Load register from register
            },

            0b10 => {
                match opcode >> 4 & 0b0011 {
                    //only get 4 and 5. bits 
                        
                    // ADD/ADC
                    0b00 => {
                        if opcode > 0x87 {
                            // It's 87 and not 86 because 0x87 is ADD A, A 
                            // without the carry
                            if opcode == 0x8E {
                                // Add from HL with carry
                            }
                            // Add from register with carry

                        }
                        if opcode == 0x86 {
                            // Add from carry
                        }
                        // Add from Register
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

