



struct CPU {
    registers: [u8; 16], 
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
    flags: u8,
    // flag register structure
    // 7 6 5 4 3 2 1 0
    // Z N H C 0 0 0 0

/*
     *Accumulator: A
        An 8-bit register for storing data and the results 
        of arithmetic and logical operations
     

    Auxiliary registers: B, C, D, E, F, H, and L
        These serve as auxiliary registers to the accumulator. 
        As register pairs (BC, DE, HL), they are 8-bit
        registers that function as data pointers


     Program counter: PC
            A 16-bit register that holds the address data of the program to be executed next.
            Usually incremented automatically according to the byte count of the fetched instructions. When an
            instruction with branching is executed, however, immediate data and register contents are loaded
    

    Stack pointer: SP
            A 16-bit register that holds the starting address of the stack area of memory.
            The contents of the stack pointer are decremented when a subroutine CALL instruction or PUSH
            instruction is executed or when an interrupt occurs and incremented when a return instruction or pop
            instruction is executed.


    Flag Register: F
        Consists of 4 flags that are set and reset according to the results of instruction execution.
        Flags CY and Z are tested by various conditional branch instructions.
        Z: Set to 1 when the result of an operation is 0; otherwise reset.
        N: Set to 1 following execution of the substruction instruction, regardless of the result.
        H: Set to 1 when an operation results in carrying from or borrowing to bit 3.
        CY: Set to 1 when an operation results in carrying from or borrowing to bit 7.


*/


}
