
use log::{trace, debug, info};
use std::fmt;
use super::instruction::Instruction;
use super::instruction::Operand;
use crate::memory::MemorySpace;

use crate::cpu::instruction::Instruction::*;
use super::instruction::Operand::*;
use super::register::*;
use crate::utils::{as_u16, hilo};
use std::borrow::Borrow;

type OpCode = u8;

#[derive(Debug)]
pub struct CPU {
    pub register: Registers,
    pub memory: MemorySpace,
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
        trace!("Fetching next opcode. PC: {:#?}", self.register.PC);
        self.read(Word)
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
                // identical to LD8 since the offset is set at instruction level
                let data: u8 = self.read(op2);
                self.write(op1, data);
            },
            LDHL(sp, op2) => {
                let offset: u8 = self.read(op2);
                let address = self.register.SP + offset as u16;

                // Write addr to HL
                self.write(HL, address);

                // TODO FLAGS
            },
            PUSH(op) => {
                let data: u16 = self.read(op);
                self.push(data);
            },
            POP(op) => {
                let data: u16 = self.pop();
                self.write(op, data)
            },
            ADD8(op1, op2) => {
                let n: u8 = self.read(op2);
                println!("n {}", n);
                let result = self.register.carrying_add(self.register.A, n);
                println!("result {}", result);
                self.register.A = result;
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
                let mut n: u8 = self.read(op2);
                if self.register.read_flag(Flags::Carry) {
                    n += 1;
                }

                let result = self.register.carrying_add(self.register.A, n);
                self.register.A = result;
            },
            SUB(op) => {
                let n: u8 = self.read(op);
                let result = self.register.borrowing_sub(self.register.A, n);
                self.register.A = result;
            },
            SBC(op1, op2) => {
                let mut n: u8 = self.read(op2);
                if self.register.read_flag(Flags::Carry) {
                    n += 1;
                }

                let result = self.register.borrowing_sub(self.register.A, n);
                self.register.A = result;
            },
            AND(op) => {
                let n: u8 = self.read(op);
                self.register.A &= n;

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero );
                }
                self.register.set_flag(Flags::HalfCarry );
            },
            OR(op) => {
                let n: u8 = self.read(op);
                self.register.A |= n;

                if self.register.A == 0 {
                    self.register.set_flag( Flags::Zero );
                }
            },
            XOR(op) => {
                let n: u8 = self.read(op);
                self.register.A ^= n;

                if self.register.A == 0 {
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
                let result = self.register.carrying_add(n, 1);
                self.write(op, result);
            },
            INC16(op) => {
                let n: u16 = self.read(op.clone());
                let result = self.register.carrying_add(n, 1);
                self.write(op, result);
            },
            DEC8(op) => {
                let n: u8 = self.read(op.clone());
                let result = self.register.borrowing_sub(n, 1);
                self.write(op, result);
            },
            DEC16(op) => {
                let n: u16 = self.read(op.clone());
                let result = self.register.borrowing_sub(n, 1);
                self.write(op, result);
            },
            SWAP(op) => {
                let n: u8 = self.read(op.clone());
                self.write(op, n.swap_bytes());

                if n == 0 {
                    self.register.set_flag( Flags::Zero )
                }
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
                self.register.reset_flag(Flags::Carry);
            },
            DAA => {},
            CPL => {
                self.register.A = self.register.A.reverse_bits();

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            },
            CCF => {
                if self.register.read_flag(Flags::Carry) {
                    self.register.reset_flag(Flags::Carry);
                } else {
                    self.register.set_flag(Flags::Carry);
                }

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            },
            SCF => {
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
                self.register.set_flag(Flags::Carry);
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

            // ---------- JUMP INSTRUCTIONS ----------
            JP1(op) => {
                let address: u16 = self.read(op);
                self.register.PC = address;
            },
            JP(op1, op2) => {
                if self.jump_allowed(op1) {
                    let address: u16 = self.read(op2);
                    self.register.PC = address;
                }
            },
            JR1(op) => {
                let offset: u8 = self.read(op);
                self.register.PC += offset as u16;
            },
            JR(cc, nn) => {
                if self.jump_allowed(cc) {
                    let offset: u8 = self.read(nn);
                    self.register.PC += offset as u16;
                }
            },

            // ---------- CALL INSTRUCTIONS ----------
            CALL1(op) => {
                // Push address of next instruction
                self.push(self.register.PC + 1);
                // Jump to address by replacing Program Counter with value
                let address: u16 = self.read(op);
                self.jump(address);
            },
            CALL(op1, op2) => {

                if self.jump_allowed(op1) {
                    // Push current Stack Pointer
                    self.push(self.register.PC + 1);
                    // Jump to address by replacing Program Counter with value
                    let address: u16 = self.read(op2);
                    self.jump(address);
                }
            },
            RST(op) => {
                self.execute(PUSH(SP));

                let address = match op {
                    FixedValue(address) => address,
                    _ => panic!("Illegal operand {:?} for restart address", op)
                };
                self.jump(address);
                self.cycle += 20;
            },
            RET_ => {
                let address = self.pop();
                self.jump(address);
                // TODO review cycle accuracy
            },
            RET(cc) => {
                if self.jump_allowed(cc) {
                    self.execute(RET_);
                }
            },
            RETI => {
                self.execute(RET_);
                self.execute(EI);
            },

            // ---------- ROTATE INSTRUCTIONS ----------
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
            RLC(op) => {},
            RL(op) => {},
            RRC(op) => {},
            RR(op) => {},
            // ---------- SHIFT INSTRUCTIONS ----------
            SLA(op) => {},
            SRA(op) => {},
            SRL(op) => {},

            // ---------- BIT INSTRUCTIONS ----------

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
                let result: u8 = value & !(1 << nth_bit);
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

    fn jump(&mut self, address: u16) {
        self.register.PC = address;
        self.cycle += 8;
    }

    // USE FOR TESTING PURPOSES
    // TODO find a way to impl this in cpu_test
    pub fn exec_single_instruction(&mut self) {
        let opcode = self.fetch();
        let instruction = self.decode( opcode );
        self.execute( instruction );
    }
}

pub trait ReadWrite<T> {
    fn read(&mut self, operand: Operand) -> T;
    fn write(&mut self, operand: Operand, data: T);
}

trait PushPop<T> {
    fn push(&mut self, data: T);
    fn pop(&mut self) -> T;
}

impl ReadWrite<u8> for CPU {
    fn read(&mut self, operand: Operand) -> u8 {
        let op = operand.clone();

        let word = match operand {
            // reading registers does not consume cycles
            A => self.register.A,
            B => self.register.B,
            C => self.register.C,
            D => self.register.D,
            E => self.register.E,
            F => self.register.F,
            H => self.register.H,
            L => self.register.L,
            Memory(addr, offset) => {
                self.cycle += 4;
                let address: u16 = self.read(*addr);
                self.memory[ address + offset ]
            },
            Word => {
                let data = self.memory[ self.register.PC ];
                self.register.PC += 1;
                self.cycle += 4;
                data
            },
            _ => panic!("Invalid operand {:?} to read word", operand)
        };

        trace!("Read word {:#X} from operand {:?}", word, op);
        word
    }

    fn write(&mut self, operand: Operand, data: u8) {
        trace!("Writing word {:#X} into {:?}", data, operand);

        match operand {
            A => self.register.A = data,
            B => self.register.B = data,
            C => self.register.C = data,
            D => self.register.D = data,
            E => self.register.E = data,
            F => self.register.F = data,
            H => self.register.H = data,
            L => self.register.L = data,
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
                let data: u8 = self.read(operand);
                data as u16
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
            _ => panic!("Invalid operand {:?} to write word", operand)
        }
    }
}

impl PushPop<u8> for CPU {
    fn push(&mut self, data: u8) {
        self.write( Memory(Box::new(SP), 0x0), data );
        self.register.SP -= 1;
    }

    fn pop(&mut self) -> u8 {
        let data = self.read(Memory(Box::new(SP), 0x0) );
        self.register.SP += 1;
        data
    }
}

impl PushPop<u16> for CPU {
    fn push(&mut self, data: u16) {
        let (hi, lo) = hilo(data);
        self.push(lo);
        self.push(hi);
    }

    fn pop(&mut self) -> u16 {
        let (hi, lo) = (self.pop(), self.pop());
        as_u16(hi, lo)
    }
}

#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    fn should_fetch_opcode() {
        let mut cpu = CPU::new(MemorySpace::new(&[0xFF]));
        let current_cycle = cpu.cycle;
        let current_program_counter = cpu.register.PC;

        let opcode = cpu.fetch();

        assert_eq!(opcode, 0xFF);
        assert_eq!(cpu.cycle, current_cycle + 4);
        assert_eq!(cpu.register.PC, current_program_counter + 1);
    }

    #[test]
    fn should_decode_simple_instruction() {
        let mut cpu = CPU::new(MemorySpace::new(&[0xFF]));

        let instruction = cpu.decode(0x06);
        assert_eq!(instruction, LD8(B, Word));
    }

    #[test]
    fn should_decode_complex_instruction() {
        let mut cpu = CPU::new(MemorySpace::new(&[0x37]));

        let instruction = cpu.decode(0xCB);
        assert_eq!(instruction, SWAP(A));
    }
}

