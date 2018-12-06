
use std::ops::Index;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;

mod instruction;

use instrution::Instruction::*;


type OpCode = u32;

struct Register {
    A: Box<u8>,
    B: Box<u8>,
    C: Box<u8>,
    D: Box<u8>,
    E: Box<u8>,
    F: Box<u8>,
    H: Box<u8>,
    L: Box<u8>,

    SP: Box<u16>,
    PC: Box<u16>
}

impl Register {

    // Special registers
    fn HL(&self) -> Box<u16> {
        box (self.H << 8) | self.L
    }

    fn BC(&self) -> Box<u16> {
        box (self.B << 8) | self.C
    }

    fn DE(&self) -> Box<u16> {
        box (self.D << 8) | self.E
    }
}



struct CPU {
    register: Register,
    memory_channel: MemorySpace,
    cycle: u16
}






impl CPU {

    fn read_memory(&self) -> byte {
        self.memory_channel.read()
    }

    fn fetch(&mut self) -> Opcode {

        let mut SP = self.register.SP;

        let data = channel.read(SP, 1);
        SP = SP + 8;

        data
    }

    fn decode(&mut self, op_code: OpCode, memory: &MemorySpace) -> Instruction {

        match op_code {

            // 8 bit LOAD

            // LD nn,n
            0x06 => LD ( self.register.B, self.read_memory()),
            0x0E => LD ( self.register.C, self.read_memory()),
            0x16 => LD ( self.register.D, self.read_memory()),
            0x1E => LD ( self.register.E, self.read_memory()),
            0x26 => LD ( self.register.H, self.read_memory()),
            0x2E => LD ( self.register.L, self.read_memory()),

            // LD r1,r2
            0x7F => LD ( self.register.A, self.register.A ),
            0x78 => LD ( self.register.A, self.register.B ),
            0x79 => LD ( self.register.A, self.register.C ),
            0x7A => LD ( self.register.A, self.register.D ),
            0x7B => LD ( self.register.A, self.register.E ),
            0x7C => LD ( self.register.A, self.register.H ),
            0x7D => LD ( self.register.A, self.register.L ),
            0x7E => LD ( self.register.A, self.register.HL()),
            0x40 => LD ( self.register.B, self.register.B ),
            0x41 => LD ( self.register.B, self.register.C ),
            0x42 => LD ( self.register.B, self.register.D ),
            0x43 => LD ( self.register.B, self.register.E ),
            0x44 => LD ( self.register.B, self.register.H ),
            0x45 => LD ( self.register.B, self.register.L ),
            0x46 => LD ( self.register.B, self.register.HL()),
            0x48 => LD ( self.register.C, self.register.B ),
            0x49 => LD ( self.register.C, self.register.C ),
            0x4A => LD ( self.register.C, self.register.D ),
            0x4B => LD ( self.register.C, self.register.E ),
            0x4C => LD ( self.register.C, self.register.H ),
            0x4D => LD ( self.register.C, self.register.L ),
            0x4E => LD ( self.register.C, self.register.HL()),
            0x50 => LD ( self.register.D, self.register.B ),
            0x51 => LD ( self.register.D, self.register.C ),
            0x52 => LD ( self.register.D, self.register.D ),
            0x53 => LD ( self.register.D, self.register.E ),
            0x54 => LD ( self.register.D, self.register.H ),
            0x55 => LD ( self.register.D, self.register.L ),
            0x56 => LD ( self.register.D, self.register.HL()),
            0x58 => LD ( self.register.E, self.register.B ),
            0x59 => LD ( self.register.E, self.register.C ),
            0x5A => LD ( self.register.E, self.register.D ),
            0x5B => LD ( self.register.E, self.register.E ),
            0x5C => LD ( self.register.E, self.register.H ),
            0x5D => LD ( self.register.E, self.register.L ),
            0x5E => LD ( self.register.E, self.register.HL()),
            0x60 => LD ( self.register.H, self.register.B ),
            0x61 => LD ( self.register.H, self.register.C ),
            0x62 => LD ( self.register.H, self.register.D ),
            0x63 => LD ( self.register.H, self.register.E ),
            0x64 => LD ( self.register.H, self.register.H ),
            0x65 => LD ( self.register.H, self.register.L ),
            0x66 => LD ( self.register.H, self.register.HL()),
            0x68 => LD ( self.register.L, self.register.B ),
            0x69 => LD ( self.register.L, self.register.C ),
            0x6A => LD ( self.register.L, self.register.D ),
            0x6B => LD ( self.register.L, self.register.E ),
            0x6C => LD ( self.register.L, self.register.H ),
            0x6D => LD ( self.register.L, self.register.L ),
            0x6E => LD ( self.register.L, self.register.HL()),
            0x70 => LD ( self.register.HL(), self.register.B ),
            0x71 => LD ( self.register.HL(), self.register.C ),
            0x72 => LD ( self.register.HL(), self.register.D ),
            0x73 => LD ( self.register.HL(), self.register.E ),
            0x74 => LD ( self.register.HL(), self.register.H ),
            0x75 => LD ( self.register.HL(), self.register.L ),
            0x36 => LD ( self.register.HL(), self.register.n ),

            // LD A,n
            0x7F => LD ( self.register.A,self.register.A ),
            0x78 => LD ( self.register.A,self.register.B ),
            0x79 => LD ( self.register.A,self.register.C ),
            0x7A => LD ( self.register.A,self.register.D ),
            0x7B => LD ( self.register.A,self.register.E ),
            0x7C => LD ( self.register.A,self.register.H ),
            0x7D => LD ( self.register.A,self.register.L ),
            0x0A => LD ( self.register.A,self.register.BC() ),
            0x1A => LD ( self.register.A,self.register.DE() ),
            0x7E => LD ( self.register.A,self.register.HL() ),
            0xFA => LD ( self.register.A,self.register.(nn) )
            0x3E => LD ( self.register.A,self.register.# )

















            _ => panic!()
        }
    }

    fn execute(&self, instruction: Instruction) {

    }

}

