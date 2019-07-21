
use log::{trace, debug, info};
use std::fmt;
use super::instruction::Instruction;
use super::instruction::Operand;
use crate::memory::MemorySpace;

use crate::cpu::instruction::Instruction::*;
use super::instruction::Operand::*;
use super::register::*;
use crate::utils::as_u16;
use std::borrow::Borrow;

type OpCode = u8;

#[derive(Debug)]
pub struct CPU {
    register: Registers,
    memory: MemorySpace,
    cycle: u32,
    halted: bool,
    interrupts_enabled : bool
}

impl CPU {

    pub fn new(memory: MemorySpace) -> CPU {
        let cpu = CPU {
            register: Registers::new(),
            memory,
            cycle: 0,
            halted: false,
            interrupts_enabled: true
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

    fn fetch(&mut self) -> OpCode {
        trace!("Fetching next byte. SP: {:#?}", self.register.SP);
        let data = self.memory[ self.register.SP ];
        self.register.SP += 1;
        self.cycle += 4;

        data
    }

    fn decode(&mut self, opcode: OpCode) -> Instruction {
        trace!("Decoding opcode {:#X}", opcode);
        match opcode {
            // Special instructions
            0xCB => {
                let next_byte = self.fetch();
                let opcode = as_u16(opcode,next_byte);
                Instruction::from(opcode)
            }
            // Basic instructions
            _ => Instruction::from(opcode)
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        trace!("Executing {:?}. Cycle: {}", instruction, self.cycle);

        match instruction {
            LD8(op1, op2) => {
                let data: u8 = self.read(op2);
                self.write(op1, data);
            },
            LD16(op1, op2) => {
                let data: u16 = self.read(op2);
                self.write(op1, data);
            },
            LDD(op1, op2) => {
                let data: u8 = self.read(op2);
                self.write(op1, data);

                let hl = self.register.read_HL();
                let (hl, overflow) = hl.overflowing_sub(1);
                self.register.write_HL(hl);
                // DEC self.register.HL()
            },
            LDI(op1, op2) => {
                let data: u8 = self.read(op2);
                self.write(op1, data);

                let hl = self.register.read_HL();
                let (hl, overflow) = hl.overflowing_add(1);
                self.register.write_HL(hl);
                // INC self.register.HL()
            },
            LDH(op1, op2) => {
                let data: u8 = self.read(op2);
                self.write(op1, data);
            },
            LDHL(_, op2) => {
                let offset: u8 = self.read(op2);
                let addr = self.register.SP + offset as u16;
                //self.write(op2, addr);
                // Write addr to HL
            },
            PUSH(op) => {
                let data: u8 = self.read(op);
                self.write( Memory(Box::new(SP), 0), data );
                self.register.SP -= 2;
            },
            POP(op) => {
                let data = self.memory[ self.register.SP ];
                self.write(op, data);
                self.register.SP += 2;
            },
            ADD8(op1, op2) => {
                let n: u8 = self.read(op2);
                let (a, overflow) = self.register[A].overflowing_add(n);
                self.register[A] = a;

                if overflow {

                }
            },
            ADD16(op1, op2) => {
                let x: u16 = self.read(op1.clone());
                let y: u16 = self.read(op2);

                let (res, overflow) = x.overflowing_add(y);
                self.write(op1, res);

                if overflow {

                }
            },
            ADC(op1, op2) => {

            },
            SUB(op) => {
                let x: u8 = self.read(op.clone());

                let (res, overflow) = self.register[A].overflowing_sub(x);
                self.register[A] = res;

                if overflow {

                }
            },
            SBC(op1, op2) => {},
            AND(op) => {
                let n: u8 = self.read(op);
                self.register[A] &= n;
                // TODO flags
            },
            OR(op) => {
                let n: u8 = self.read(op);
                self.register[A] |= n;
                // TODO flags
            },
            XOR(op) => {
                let n: u8 = self.read(op);
                self.register[A] ^= n;
                // TODO flags
            },
            CP(op) => {

            },
            INC8(op) => {
                let n: u8 = self.read(op.clone());
                let (result, overflow) = n.overflowing_add(1);
                self.write(op, result);

                if overflow {
                    // TODO flags
                }
            },
            INC16(op) => {
                let n: u16 = self.read(op.clone());
                let (result, overflow) = n.overflowing_add(1);
                self.write(op, result);

                if overflow {
                    // TODO flags
                }
            },
            DEC8(op) => {
                let n: u8 = self.read(op.clone());
                let (result, overflow) = n.overflowing_sub(1);
                self.write(op, result);

                if overflow {
                    // TODO flags
                }
            },
            DEC16(op) => {
                let n: u16 = self.read(op.clone());
                let (result, overflow) = n.overflowing_sub(1);
                self.write(op, result);

                if overflow {
                    // TODO flags
                }
            },
            SWAP(op) => {
                let n: u8 = self.read(op.clone());
                self.write(op, n.swap_bytes());
            },
            DAA => {},
            CPL => {
                //self.register.A = self.register.A.reverse_bits();
                // TODO
            },
            CCF => {

            },
            SCF => {},
            NOP => (),
            HALT => {},
            DI => {},
            EI => {},
            RLCA => {
                self.register.A = self.register.A.rotate_left(1);
                // TDDO Old bit 7 to Carry flag
            },
            RLA => {},
            RRCA => {
                self.register.A = self.register.A.rotate_right(1);
                // TDDO Old bit 0 to Carry flag
            },
            RRA => {},
            JP1(op) => {
                //self[PC] = self.read(op);
            },
            JP(op1, op2) => {
                let address: u16 = self.read(op2);
                self.register.SP = address;
                self.cycle += 12;
            },
            JR1(op) => {},
            JR(op1, op2) => {},
            CALL1(op) => {},
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
}

trait ReadWrite<T> {
    fn read(&mut self, operand: Operand) -> T;
    fn write(&mut self, operand: Operand, data: T);
}

impl ReadWrite<u8> for CPU {
    fn read(&mut self, operand: Operand) -> u8 {
        let op = operand.clone();

        let word = match operand {
            A | B | C | D | E | F | H | L => {
                // reading registers does not consume cycles
                self.register[operand]
            },
            Memory(addr, offset) => {
                self.cycle += 4;
                let tmp: u16 = self.read(*addr);
                let address = tmp + offset;
                self.memory[address]
            },
            Word => {
                self.cycle += 4;
                self.fetch()
            },
            _ => panic!("Invalid operand {:?} to read word", operand)
        };

        trace!("Read word {:#X} from operand {:?}", word, op);
        word
    }

    fn write(&mut self, operand: Operand, data: u8) {
        trace!("Writing word {:#X} into {:?}", data, operand);

        match operand {
            A | B | C | D | E | F | H | L => {
                self.register[operand] = data;
            }
            HL => {},
            AF => {},
            BC => {},
            DE => {},
            DE => {},
            SP => {},
            PC => {},
            Memory(addr, offset) => {
                self.cycle += 4;
                let tmp: u16 = self.read(*addr);
                let addr = tmp + offset;
                self.memory[addr] = data;
            },
            _ => panic!("Invalid operand {:?} to write word", operand)
        }
    }
}

impl ReadWrite<u16> for CPU {
    fn read(&mut self, operand: Operand) -> u16 {
        let op = operand.clone();

        let dword = match operand {
            A | B | C | D | E | F | H | L => {
                self.register[operand] as u16
            },
            HL => self.register.read_HL(),
            AF => self.register.read_AF(),
            BC => self.register.read_BC(),
            DE => self.register.read_DE(),
            SP => self.register.SP,
            PC => self.register.PC,
            Word => as_u16(0, self.read(operand)),
            DWord => {
                as_u16(
                    self.read(Word),
                    self.read(Word)
                )
            },
            _ => panic!("Invalid operand {:?} to read double word", operand)
        };

        trace!("Read dword {:#X} from operand {:?}", dword, op);
        dword
    }

    fn write(&mut self, operand: Operand, data: u16) {
        trace!("Writing dword {:#X} into operand {:?}", data, operand);

        match &operand {
            HL => self.register.write_HL(data),
            AF => self.register.write_AF(data),
            BC => self.register.write_BC(data),
            DE => self.register.write_DE(data),
            SP => self.register.SP = data,
            PC => self.register.PC = data,
            Memory(addr, offset) => {

            },
            _ => panic!("Invalid operand {:?} to write word", operand)
        }
    }
}
