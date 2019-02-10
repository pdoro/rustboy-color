
use log::{trace, debug, info};
use std::ops;
use std::fmt;
use super::instruction::Instruction;
use super::instruction::Operand;
use crate::memory::MemorySpace;
use crate::memory::Address;

use crate::cpu::instruction::Instruction::*;
use super::instruction::Operand::*;
use super::register::*;
use crate::utils::as_u16;

type OpCode = u8;

#[derive(Debug)]
pub struct CPU {
    register: Register,
    memory: MemorySpace,
    cycle: u32,
}

impl CPU {

    pub fn new(memory: MemorySpace) -> CPU {
        let cpu = CPU {
            register: Register::new(),
            memory,
            cycle: 0
        };
        debug!("CPU initialized. {:#?}", cpu);
        cpu
    }

    pub fn run(mut self) {
        debug!("Fetch-Decode-Execute loop starting");
        loop {
            let opcode = self.fetch();
            let instruction = self.decode( opcode );
            self.execute( instruction );
        }
    }

    pub fn fetch(&mut self) -> OpCode {
        trace!("Fetching next byte. SP : {:#?}", self.register.SP);
        let data = self.memory[ self.register.SP ];
        self.register.SP += 1;

        data
    }

    pub fn decode(&mut self, opcode: OpCode) -> Instruction {
        trace!("Decoding opcode {:#X}", opcode);
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

        trace!("Executing {:?}", instruction);

        match instruction {
            LD8(op1, op2) => {
                let data = self.read_word(op2);
                self.write_word(op1, data);
            },
            LD16(op1, op2) => {},
            LDD(op1, op2) => {},
            LDI(op1, op2) => {},
            LDH(op1, op2) => {},
            LDHL(op1, op2) => {},
            PUSH(op) => {},
            POP(op) => {},
            ADD(op1, op2) => {
                let n = self.read_word(op2);
                self[A] += n;
                // TODO flags
            },
            ADC(op1, op2) => {},
            SUB(op) => {
                let n = self.read_word(op);
                self[A] -= n;
                // TODO flags
            },
            SBC(op1, op2) => {},
            AND(op) => {
                let n = self.read_word(op);
                self[A] &= n;
                // TODO flags
            },
            OR(op) => {
                let n = self.read_word(op);
                self[A] |= n;
                // TODO flags
            },
            XOR(op) => {
                let n = self.read_word(op);
                self[A] ^= n;
                // TODO flags
            },
            CP(op) => {},
            INC(op) => {
                let n = self.read_word(op.clone());
                self.write_word(op, n + 1);
                // TODO flags
            },
            DEC(op) => {
                let n = self.read_word(op.clone());
                self.write_word(op, n - 1);
                // TODO flags
            },
            SWAP(op) => {},
            DAA => {},
            CPL => {},
            CCF => {},
            SCF => {},
            NOP => (),
            HALT => {},
            DI => {},
            EI => {},
            RLCA => {},
            RLA => {},
            RRCA => {},
            RRA => {},
            JP_(op) => {
                //self[PC] = self.read_dword(op);
            },
            JP(op1, op2) => {},
            JR_(op) => {},
            JR(op1, op2) => {},
            CALL_(op) => {},
            CALL(op1, op2) => {},
            RST(op) => {},
            RET_ => {},
            RET(op) => {},
            RETI => {},
            RLC(op) => {},
            RL(op) => {},
            RRC(op) => {},
            RR(op) => {},
            SLA(op) => {},
            SRA(op) => {},
            SRL(op) => {},
            BIT(op1, op2) => {},
            SET(op1, op2) => {},
            RES(op1, op2) => {},
        }
    }


    fn read_word(&mut self, operand: Operand) -> u8 {

        trace!("Reading word from operand {:?}", operand);

        match operand {
            A | B | C | D | E | F | H | L => {
                self[operand]
            }
            Memory(addr) => {0},
            OffsetMemory(offset, addr) => {0},
            Word => self.fetch(),
            _ => panic!("Invalid operand to read from {:?}", operand)
        }
    }

    fn write_word(&mut self, operand: Operand, data: u8) {

        trace!("Writing word into operand {:?}", operand);

        match operand {
            A | B | C | D | E | F | H | L => {
                self[operand] = data;
            }
            HL => {},
            AF => {},
            BC => {},
            DE => {},
            SP => {},
            PC => {},
            Memory(addr) => {
                let addr = self.read_word(*addr);

            },
            OffsetMemory(offset, addr) => {},
            _ => panic!("Invalid operand to write into {:?}", operand)
        }
    }

    fn read_dword(&mut self, operand: Operand) -> u16 {
        trace!("Reading double word from operand {:?}", operand);
        as_u16(
            self.read_word(operand.clone()),
            self.read_word(operand)
        )
    }
}

impl ops::IndexMut<Operand> for CPU {

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

impl ops::Index<Operand> for CPU {
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
