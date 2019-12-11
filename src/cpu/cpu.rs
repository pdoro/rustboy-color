
use log::{trace, debug, info};
use std::fmt;
use super::instruction::Instruction;
use super::instruction::Operand;
use crate::memory::MemorySpace;

use crate::cpu::instruction::Instruction::*;
use super::instruction::Operand::*;
use super::register::*;
use crate::utils::{as_u16, lohi};
use std::borrow::Borrow;

type OpCode = u8;

#[derive(Debug)]
pub struct CPU {
    pub register: Registers,
    memory: MemorySpace,
    pub cycle: u32,
    pub halted: bool,
    pub stopped: bool,
    pub interrupts_enabled : bool
}

impl CPU {

    pub fn new(memory: MemorySpace) -> CPU {
        let cpu = CPU {
            register: Registers::default(),
            memory,
            cycle: 0,
            halted: false,
            stopped: false,
            interrupts_enabled: true
        };
        debug!("CPU initialized. {:#?}", cpu);
        cpu
    }

    pub fn run(&mut self) {
        debug!("Fetch-Decode-Execute loop starting");
        loop {
            let opcode = self.fetch();
            let instruction = self.decode( opcode );
            self.execute( instruction );

            if self.halted {
                break
            }
        }
    }

    fn fetch(&mut self) -> OpCode {
        trace!("Fetching next byte. PC: {:#?}", self.register.PC);
        let data = self.memory[ self.register.PC ];
        self.register.PC += 1;
        self.cycle += 4;

        data
    }

    fn decode(&mut self, opcode: OpCode) -> Instruction {
        trace!("Decoding opcode {:#X}", opcode);
        match opcode {
            // Special instructions always start with 0XCB
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
                self.execute(LD8(op1, op2));
                self.execute(DEC16(HL));
            },
            LDI(op1, op2) => {
                self.execute(LD8(op1, op2));
                self.execute(INC16(HL));
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
                let data: u16 = self.read(op);
                let (lo, hi) = lohi(data);
                self.push(lo);
                self.push(hi);
            },
            POP(op) => {
                let data = as_u16(self.pop(), self.pop() );
                self.write(op, data)
            },
            ADD8(op1, op2) => {
                let n: u8 = self.read(op2);
                let (result, overflow) = self.register[A].overflowing_add(n);
                self.register[A] = result;

                if result == 0 {
                    self.register.set_flag(Flags::Zero);
                }

                if overflow {
                    self.register.set_flag(Flags::Carry);
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

                if self.register[A] == 0 {
                    self.register.set_flag(Flags::Zero );
                }
                self.register.set_flag(Flags::HalfCarry );
            },
            OR(op) => {
                let n: u8 = self.read(op);
                self.register[A] |= n;

                if self.register[A] == 0 {
                    self.register.set_flag( Flags::Zero );
                }
            },
            XOR(op) => {
                let n: u8 = self.read(op);
                self.register[A] ^= n;

                if self.register[A] == 0 {
                    self.register.set_flag( Flags::Zero );
                }
            },
            CP(op) => {
                let n: u8 = self.read(op);
                let result: u8 = self.register.A - n;

                // TODO WRITE FLAGS
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

                if n == 0 {
                    self.register.set_flag( Flags::Zero )
                }
            },
            DAA => {},
            CPL => {
                self.register.A = self.register.A.reverse_bits();
                // TODO flags
            },
            CCF => {
                // TODO flags
            },
            SCF => {
                // TODO flags
            },
            NOP => (),
            HALT => {
                self.halted = true;
            },
            STOP => {
                self.stopped = true;
            },
            DI => {
                self.interrupts_enabled = false;
            },
            EI => {
                self.interrupts_enabled = true;
            },
            RLCA => {
                let old_bit = self.register.A & 0b10000000;
                self.register.A = self.register.A.rotate_left(1);

                if self.register.A == 0 {
                    self.register.set_flag( Flags::Zero )
                }
                if old_bit != 0 {
                    self.register.set_flag( Flags::Carry )
                }
            },
            RLA => {},
            RRCA => {
                self.register.A = self.register.A.rotate_right(1);
                // TDDO Old bit 0 to Carry flag
            },
            RRA => {},
            JP1(op) => {
                let address: u16 = self.read(op);
                self.register.PC = address;
                self.cycle += 12;
            },
            JP(op1, op2) => {
                if self.jump_allowed(op1) {
                    let address: u16 = self.read(op2);
                    self.register.PC = address;
                    self.cycle += 12;
                }
            },
            JR1(op) => {
                let offset: u8 = self.read(op);
                self.register.PC += offset as u16;
                self.cycle += 12;
            },
            JR(cc, nn) => {
                if self.jump_allowed(cc) {
                    let offset: u8 = self.read(nn);
                    self.register.PC += offset as u16;
                    self.cycle += 12;
                }
            },
            CALL1(op) => {

                let (lo, hi) = lohi(self.register.PC + 1);
                self.push( lo );
                self.push( hi );

                let address: u16 = self.read(op);
                self.register.PC = address;
                self.cycle += 12;
            },
            CALL(op1, op2) => {

                if self.jump_allowed(op1) {
                    // Push current Stack Pointer
                    let (lo, hi) = lohi(self.register.SP);
                    self.push( lo );
                    self.push( hi );
                    // Jump to address by replacing Stack Pointer with value
                    let address: u16 = self.read(op2);
                    self.register.PC = address;
                    self.cycle += 12;
                }
            },
            RST(op) => {

            },
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
            BIT(op1, op2) => {
                let nth_bit: u8 = self.read(op1);
                let value: u8 = self.read(op2.clone());
                let is_zero = ((value >> nth_bit) & 1) == 0 ;

                // https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit

                if is_zero { self.register.set_flag( Flags::Zero); }
                self.register.set_flag( Flags::HalfCarry);
                self.register.reset_flag( Flags::Subtract);
            },
            SET(op1, op2) => {
                let nth_bit: u8 = self.read(op1);
                let value: u8 = self.read(op2.clone());
                let result: u8 = value | 1 << nth_bit;
                self.write(op2, value);
            },
            RES(op1, op2) => {
                let nth_bit: u8 = self.read(op1);
                let value: u8 = self.read(op2.clone());
                let result: u8 = value & !(1 << flag as u8);
                self.write(op2, value);
            },
        }
    }

    fn jump_allowed(&self, operand: Operand) -> bool {
        match operand {
            Zero => self.register.read_flag(Flags::Zero),
            NoZero => ! self.register.read_flag(Flags::Zero),
            Carry => self.register.read_flag(Flags::Carry),
            NoCarry => ! self.register.read_flag(Flags::Carry),
            _ => panic!("Invalid operand {:?} for JP instructions", operand)
        }
    }

    fn push(&mut self, data: u8) {
        self.write( Memory(Box::new(SP), 0x0), data );
        self.cycle += 8;
        self.register.SP -= 1;
    }

    fn pop(&mut self) -> u8 {
        let data = self.read(Memory(Box::new(SP), 0x0) );
        self.cycle += 8;
        self.register.SP += 1;
        data
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
                let address: u16 = self.read(*addr);
                self.memory[ address + offset ]
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
                let address: u16 = self.read(*addr);
                self.memory[ address + offset ] = data;
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
            DWord => as_u16(self.read(Word),self.read(Word)),
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