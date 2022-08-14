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
*/
