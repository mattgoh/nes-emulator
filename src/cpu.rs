/* 4 instructions. First instruction has a parameter
LDA #$c0 ; a9 c0
TAX      ; aa
INX      ; e8
BRK      ; 00

Opcodes can be found here:

https://www.nesdev.org/obelisk-6502-guide/reference.html 

LDA: load accumulator
     Loads a byte of memory into the accumulator setting the zero and negative flags as
     appropriate
TAX: transfer accumulator to X
     Copies current contents of accumulator into the X register
INX: increment X register
    Adds one to the X register setting the zero and negative flags as appropriate
BRK: force interrupt
    The BRK instruction forces the generation of an interupt request.  The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
*/

pub struct CPU {
    pub register_a: u8,
    pub status u8,
    pub program_counter: u16,
}

impl CPU{
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
    todo!("")
    }
}
