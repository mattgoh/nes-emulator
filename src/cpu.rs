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
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    /*
    CPU works in a constant cycle:
    - Fetch next exectuion instruction from the instruction memory
    - Decode instruction
    - Execute instruction
    - Repeat cycle
    */

    /*
    Memory
    NES implements von Neumann architecture. Both data and instructions are stored in memory.
    Executed code is data from CPU perspective. Any data can be interpreted as executable code
    The only mechanism the CPU has is a program counter register to track the position in instructions stream
    */
    fn mem_read(%self, addr: u16) -> u8 {
        self.memory[addr as usize
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    pub fn run(mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {
            //
            }
        }

    }
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            // status | 2
            // if status = 0, then 0 | 2 = 2
            self.status = self.status | 0b0000_0010;
        } else {
            // status | 253
            // if status = 0, then 0 & 253 = 0
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

        match opscode {
            // When LDA or TAX is called, the zero and negative flags are updated
            //
            // LDA (0xA9) opcode
            // (0xA90) = immediate addressing mode
            0xA9 => {
                let param = program[self.program_counter as usize];
                self.program_counter += 1;
                
                self.lda(param);
            }

            // TAX (0xAA) opcode
            // (0xAA) = implied addressing mode
            // Transfer value of A to X and updating zero and negative flags
            0xAA => self.tax(),

            // INX (0xe8)
            // Add 1 to register X
            // implied addressing mode
            0xe8 => self.inx(),

            // BRK (0x00) opcode
            // implied addressing mode
            0x00 => return,
            
            _ => todo!(),
        }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);

    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
