
use log::{trace, debug, info};
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
    register: Registers,
    memory: MemorySpace,
    cycle: u32,
}

impl CPU {

    pub fn new(memory: MemorySpace) -> CPU {
        let cpu = CPU {
            register: Registers::new(),
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

    fn fetch(&mut self) -> OpCode {
        trace!("Fetching next byte. SP: {:#?}", self.register.SP);
        let data = self.memory[ Address(self.register.SP) ];
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




        // Unificar read_word y read_dword asi como write para que ambas usen u16. Cuando
	// haya que escribir un u16 en un registro castear a u8 o dar error.
	// Asi se unifica mucha logica aunque puede dar errores no previstos




        match instruction {
            LD8(op1, op2) => {
                let data = self.read_word(op2);
                self.write_word(op1, data);
            },
            LD16(op1, op2) => {
                let data = self.read_dword(op2);
                self.write_dword(op1, data);
            },
            LDD(op1, op2) => {
                let data = self.read_word(op2);
                self.write_word(op1, data);
                // DEC self.register.HL()
            },
            LDI(op1, op2) => {
                let data = self.read_word(op2);
                self.write_word(op1, data);
                // INC self.register.HL()
            },
            LDH(op1, op2) => {
                let data = self.read_word(op2);
                self.write_word(op1, data);
            },
            LDHL(_, op2) => {
                let offset = self.read_word(op2) as u16;
                let addr = self.register.SP + offset;
                //self.write_dword(op2, addr);
                // Write addr to HL
            },
            PUSH(op) => {
                let data = self.read_word(op);
                //self.memory[ Address(self.register.SP) ] = data;
                self.register.SP -= 2;
            },
            POP(op) => {
                let data = self.memory[ Address(self.register.SP) ] as u16;
                self.write_dword(op, data);
                self.register.SP += 2;
            },
            ADD(op1, op2) => {
                let n = self.read_word(op2);
                self.register[A] += n;
                // TODO flags
            },
            ADC(op1, op2) => {

            },
            SUB(op) => {
                let n = self.read_word(op);
                self.register[A] -= n;
                // TODO flags
            },
            SBC(op1, op2) => {},
            AND(op) => {
                let n = self.read_word(op);
                self.register[A] &= n;
                // TODO flags
            },
            OR(op) => {
                let n = self.read_word(op);
                self.register[A] |= n;
                // TODO flags
            },
            XOR(op) => {
                let n = self.read_word(op);
                self.register[A] ^= n;
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
            SWAP(op) => {
                let n = self.read_word(op.clone());
                self.write_word(op, n.swap_bytes());
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
                //self[PC] = self.read_dword(op);
            },
            JP(op1, op2) => {},
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

    fn write_word(&mut self, operand: Operand, data: u8) {
        trace!("Writing word into operand {:?}", operand);

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
                let addr = Address(self.read_dword(*addr) as u16 + offset);
                //self.memory[addr] = data;
            },
            _ => panic!("Invalid operand {:?} to write word", operand)
        }
    }

    fn read_word(&mut self, operand: Operand) -> u8 {
        trace!("Reading word from operand {:?}", operand);

        match operand {
            A | B | C | D | E | F | H | L => {
                // reading registers does not consume cycles
                self.register[operand]
            },
            Memory(addr, offset) => {
                trace!("Reading word from memory");
                self.cycle += 4;
                let address = Address(self.read_dword(*addr) + offset);
                self.memory[address]
            },
            Word => {
                self.cycle += 4;
                self.fetch()
            },
            _ => panic!("Invalid operand {:?} to read word", operand)
        }
    }

    fn read_dword(&mut self, operand: Operand) -> u16 {
        trace!("Reading double word from operand {:?}", operand);

        match operand {
            HL => self.register.HL(),
            AF => self.register.AF(),
            BC => self.register.BC(),
            DE => self.register.DE(),
            SP => self.register.SP,
            PC => self.register.PC,
            DWord => {
                as_u16(
                    self.read_word(Word),
                    self.read_word(Word)
                )
            },
            _ => panic!("Invalid operand {:?} to read double word", operand)
        }
    }

    fn write_dword(&mut self, operand: Operand, data: u16)  {
        trace!("Writing double word into operand {:?}", operand);

        match operand {
            HL => {},
            AF => {},
            BC => {},
            DE => {},
            SP => {},
            PC => {},
            Memory(addr, offset) => {},
            _ => panic!("Invalid operand {:?} to write word", operand)
        }
    }
}
