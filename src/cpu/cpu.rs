
use std::ops::Index;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;
use super::instruction::Instruction;
use super::instruction::Operand;
use crate::memory::MemorySpace;
use crate::memory::Address;

use std::ops::IndexMut;
use crate::cpu::instruction::Instruction::*;

type OpCode = u8;

struct Register {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    H: u8,
    L: u8,

    SP: u16,
    PC: u16,
}

impl Register {
    // Special registers
    fn AF(&self) -> u16 {
        (self.A as u16) << 8 | self.F as u16
    }

    fn HL(&self) -> u16 {
        (self.H as u16) << 8 | self.L as u16
    }

    fn BC(&self) -> u16 {
        (self.B as u16) << 8 | self.C as u16
    }

    fn DE(&self) -> u16 {
        (self.D as u16) << 8 | self.E as u16
    }

    pub fn new() -> Register {
        Register {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            H: 0,
            L: 0,

            SP: 0,
            PC: 0xFFFE,
        }
    }
}


pub struct CPU {
    register: Register,
    memory: MemorySpace,
    cycle: u32,
}

impl CPU {

    pub fn new(memory: MemorySpace) -> CPU {
        CPU {
            register: Register::new(),
            memory,
            cycle: 0
        }
    }

    pub fn run(mut self) {
        loop {
            let opcode = self.fetch();
            let instruction = self.decode( opcode );
            self.execute( instruction );
        }
    }

    pub fn fetch(&mut self) -> OpCode {
        let data = self.memory[ self.register.SP ];
        self.register.SP += 1;

        data
    }

    pub fn decode(&mut self, opcode: OpCode) -> Instruction {
        match opcode {
            // Special instructions
            0xCB => {
                let nextByte = self.fetch();
                let opcode = (opcode as u16) << 8 | nextByte as u16;
                Instruction::from(opcode)
            }
            // Basic instructions
            _ => Instruction::from(opcode)
        }
    }

    fn execute(&mut self, instruction: Instruction) {

        println!("{:?}", instruction);

        match instruction {

            LD8( op1, op2 ) => {
                //let data = self.read_byte(op2);
                //self.write_byte(op1, data);//self.cycle += cycles as u32;
            }
            NOP => {},
            _ => println!("Unknown instruction"),
        }
    }


//    fn read_byte(&mut self, operand: Operand) -> u8 {
//        match &operand {
//            Operand::Register(reg) => self[reg],
//            Operand::Byte => self.fetch(),
//            Operand::DoubleByte => 2,
//            Operand::MemoryAddress(addr) => {
//                //self.read_byte(addr)
//                // self.bus.read( data )
//                1
//            }
//        }
//    }

//    fn write_byte(&mut self, operand: Operand, data: u8) {
//        match operand {
//            Operand::Register(reg) => self[reg] = data,
//            Operand::MemoryAddress(addr) => {},
//            _ => panic!("error"),
//        }
//    }
}

impl IndexMut<Operand> for CPU {

    fn index_mut(&mut self, register: Operand) -> &mut u8 {
        match register {
            Operand::A => &mut self.register.A,
            Operand::B => &mut self.register.B,
            Operand::C => &mut self.register.C,
            Operand::D => &mut self.register.D,
            Operand::E => &mut self.register.E,
            Operand::F => &mut self.register.F,
            Operand::H => &mut self.register.H,
            Operand::L => &mut self.register.L,
            _ => panic!(""),
        }
    }
}

impl Index<Operand> for CPU {
    type Output = u8;

    fn index(& self, register: Operand) -> & Self::Output {
        match register {
            Operand::A => & self.register.A,
            Operand::B => & self.register.B,
            Operand::C => & self.register.C,
            Operand::D => & self.register.D,
            Operand::E => & self.register.E,
            Operand::F => & self.register.F,
            Operand::H => & self.register.H,
            Operand::L => & self.register.L,
            _ => panic!(""),
        }
    }
}
