use crate::memory::MemorySpace;
use crate::soc::instruction::{Instruction, Instruction::*, Operand, Operand::*};
use crate::soc::register::{Flags, MathOps, Registers};
use crate::utils::{as_u16, hilo};
use log::{debug, info, trace};
use std::ops::{Range, RangeInclusive};

type OpCode = u8;
const HIGH_RAM: RangeInclusive<u16> = 0xFF80..=0xFFFE;

#[derive(Debug)]
pub struct CPU {
    pub register: Registers,
    pub memory: MemorySpace,
    pub cycle: u32,
    pub halted: bool,
    pub stopped: bool,

    pub high_ram: [u8; 126],
}

impl CPU {
    pub fn new(memory: MemorySpace) -> CPU {
        let cpu = CPU {
            register: Registers::default(),
            memory,
            cycle: 0,
            halted: false,
            stopped: false,
            high_ram: [0; 126]
        };
        debug!("CPU initialized");
        cpu
    }

    pub fn run(&mut self) {
        debug!("Fetch-Decode-Execute loop starting");
        loop {
            let opcode = self.fetch();
            let instruction = self.decode(opcode);
            self.execute(instruction);

            if self.halted {
                break;
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
                let opcode = as_u16(opcode, next_byte);
                Instruction::from(opcode)
            }
            // Basic instructions
            _ => Instruction::from(opcode),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        trace!("Executing {:?}. Cycle: {}", instruction, self.cycle);

        match instruction {
            LD8(op1, op2) => {
                let data: u8 = self.read(op2);
                self.write(op1, data);
            }
            LD16(op1, op2) => {
                let data: u16 = self.read(op2);
                self.write(op1, data);
            }
            LDD(op1, op2) => {
                self.execute(LD8(op1, op2));
                self.execute(DEC16(HL));
            }
            LDI(op1, op2) => {
                self.execute(LD8(op1, op2));
                self.execute(INC16(HL));
            }
            LDH(op1, op2) => {
                // identical to LD8 since the offset is set at instruction level
                let data: u8 = self.read(op2);
                self.write(op1, data);
            }
            LDHL(sp, op2) => {
                let offset: u8 = self.read(op2);
                let address = self.register.SP + offset as u16;

                // Write addr to HL
                self.write(HL, address);

                // TODO FLAGS
            }
            PUSH(op) => {
                let data: u16 = self.read(op);
                self.push(data);
            }
            POP(op) => {
                let data: u16 = self.pop();
                self.write(op, data)
            }
            ADD8(op1, op2) => {
                let n: u8 = self.read(op2);
                let result = self.register.carrying_add(self.register.A, n);
                self.register.A = result;
            }
            ADD16(op1, op2) => {
                let x: u16 = self.read(op1.clone());
                let y: u16 = self.read(op2);

                let result = self.register.carrying_add(x, y);
                self.write(op1, result);
            }
            ADC(op1, op2) => {
                let mut n: u8 = self.read(op2);
                if self.register.read_flag(Flags::Carry) {
                    n += 1;
                }

                let result = self.register.carrying_add(self.register.A, n);
                self.register.A = result;
            }
            SUB(op) => {
                let n: u8 = self.read(op);
                let result = self.register.borrowing_sub(self.register.A, n);
                self.register.A = result;
            }
            SBC(op1, op2) => {
                let mut n: u8 = self.read(op2);
                if self.register.read_flag(Flags::Carry) {
                    n += 1;
                }

                let result = self.register.borrowing_sub(self.register.A, n);
                self.register.A = result;
            }
            AND(op) => {
                let n: u8 = self.read(op);
                self.register.A &= n;

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                self.register.set_flag(Flags::HalfCarry);
            }
            OR(op) => {
                let n: u8 = self.read(op);
                self.register.A |= n;

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
            }
            XOR(op) => {
                let n: u8 = self.read(op);
                self.register.A ^= n;

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
            }
            CP(op) => {
                let n: u8 = self.read(op);
                self.register.borrowing_sub(self.register.A, n);
            }
            INC8(op) => {
                let n: u8 = self.read(op.clone());
                let result = self.register.carrying_add(n, 1);
                self.write(op, result);
            }
            INC16(op) => {
                let n: u16 = self.read(op.clone());
                let result = self.register.carrying_add(n, 1);
                self.write(op, result);
            }
            DEC8(op) => {
                let n: u8 = self.read(op.clone());
                let result = self.register.borrowing_sub(n, 1);
                self.write(op, result);
            }
            DEC16(op) => {
                let n: u16 = self.read(op.clone());
                let result = self.register.borrowing_sub(n, 1);
                self.write(op, result);
            }
            SWAP(op) => {
                let n: u8 = self.read(op.clone());
                let swap = (n & 0xF0) >> 4 | (n & 0xF) << 4;
                self.write(op, swap);

                if swap == 0 {
                    self.register.set_flag(Flags::Zero)
                }
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
                self.register.reset_flag(Flags::Carry);
            }
            DAA => {}
            CPL => {
                self.register.A ^= 0xFF;

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            CCF => {
                if self.register.read_flag(Flags::Carry) {
                    self.register.reset_flag(Flags::Carry);
                } else {
                    self.register.set_flag(Flags::Carry);
                }

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            SCF => {
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
                self.register.set_flag(Flags::Carry);
            }

            // ---------- JUMP INSTRUCTIONS ----------
            JP1(op) => {
                let address: u16 = self.read(op);
                self.register.PC = address;
            }
            JP(op1, op2) => {
                if self.jump_allowed(op1) {
                    let address: u16 = self.read(op2);
                    self.register.PC = address;
                }
            }
            JR1(op) => {
                let offset: u8 = self.read(op);
                self.register.PC += offset as u16;
            }
            JR(cc, nn) => {
                if self.jump_allowed(cc) {
                    let offset: u8 = self.read(nn);
                    self.register.PC += offset as u16;
                }
            }

            // ---------- CALL INSTRUCTIONS ----------
            CALL1(op) => {
                // Push address of next instruction
                self.push(self.register.PC + 1);
                // Jump to address by replacing Program Counter with value
                let address: u16 = self.read(op);
                self.jump(address);
            }
            CALL(op1, op2) => {
                if self.jump_allowed(op1) {
                    // Push current Stack Pointer
                    self.push(self.register.PC + 1);
                    // Jump to address by replacing Program Counter with value
                    let address: u16 = self.read(op2);
                    self.jump(address);
                }
            }
            RST(address) => {
                self.execute(PUSH(SP));
                self.jump(address);
                self.cycle += 20;
            }
            RET_ => {
                let address = self.pop();
                self.jump(address);
                // TODO review cycle accuracy
            }
            RET(cc) => {
                if self.jump_allowed(cc) {
                    self.execute(RET_);
                }
            }
            RETI => {
                self.execute(RET_);
                self.execute(EI);
            }

            // ---------- ROTATE INSTRUCTIONS ----------
            RLCA => {
                self.execute(RLC(A));
            }
            RLA => {
                self.execute(RL(A));
            }
            RRCA => {
                self.execute(RRC(A));
            }
            RRA => {
                self.execute(RR(A));
            }
            RLC(op) => {
                let old_bit = self.register.A & 0b10000000;
                // Shift 1 bit left and keep only the byte
                self.register.A = (self.register.A << 1) & 0xFF;

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                if old_bit != 0 {
                    self.register.set_flag(Flags::Carry);
                }

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            RL(op) => {
                let old_bit = self.register.A & 0b10000000;
                // Shift 1 bit left and keep only the byte
                self.register.A = (self.register.A << 1) & 0xFF;

                if self.register.read_flag(Flags::Carry) {
                    self.register.A |= 0x01;
                    self.register.reset_flag(Flags::Carry);
                }

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                if old_bit != 0 {
                    self.register.set_flag(Flags::Carry);
                }

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            RRC(op) => {
                let old_bit = self.register.A & 0b00000001;
                // Shift 1 bit left and keep only the byte
                self.register.A = (self.register.A >> 1) & 0xFF;

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                if old_bit != 0 {
                    self.register.set_flag(Flags::Carry);
                }

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            RR(op) => {
                let old_bit = self.register.A & 0b00000001;
                // Shift 1 bit left and keep only the byte
                self.register.A = (self.register.A >> 1) & 0xFF;

                if self.register.read_flag(Flags::Carry) {
                    self.register.A |= 0x80;
                    self.register.reset_flag(Flags::Carry);
                }

                if self.register.A == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                if old_bit != 0 {
                    self.register.set_flag(Flags::Carry);
                }

                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            // ---------- SHIFT INSTRUCTIONS ----------
            SLA(op) => {
                let mut value: u8 = self.read(op.clone());
                let old_bit = value & 0b10000000 == 0b10000000;
                // Shift 1 bit left and keep only the byte
                value = (value << 1) & 0xFF;

                self.write(op, value);

                if old_bit {
                    self.register.set_flag(Flags::Carry);
                } else {
                    self.register.reset_flag(Flags::Carry);
                }

                if value == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            SRA(op) => {
                let mut value: u8 = self.read(op.clone());
                let old_bit = value & 0x01 == 0x01;
                // Shift 1 bit right and keep MSB
                value = (value & 0x80) | (value >> 1);

                self.write(op, value);

                if old_bit {
                    self.register.set_flag(Flags::Carry);
                } else {
                    self.register.reset_flag(Flags::Carry);
                }

                if value == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }
            SRL(op) => {
                let mut value: u8 = self.read(op.clone());
                let old_bit = value & 0b00000001 == 0b00000001;
                // Shift 1 bit left and keep only the byte
                value = (value >> 1) & 0xFF;

                self.write(op, value);

                if old_bit {
                    self.register.set_flag(Flags::Carry);
                } else {
                    self.register.reset_flag(Flags::Carry);
                }

                if value == 0 {
                    self.register.set_flag(Flags::Zero);
                }
                self.register.reset_flag(Flags::Subtract);
                self.register.reset_flag(Flags::HalfCarry);
            }

            // ---------- BIT INSTRUCTIONS ----------
            BIT(nth_bit, op2) => {
                let value: u8 = self.read(op2.clone());
                let is_zero = value & (1 << nth_bit) == 0x00;

                // https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit

                if is_zero {
                    self.register.set_flag(Flags::Zero);
                }
                self.register.set_flag(Flags::HalfCarry);
                self.register.reset_flag(Flags::Subtract);
            }
            SET(nth_bit, op2) => {
                let value: u8 = self.read(op2.clone());
                let result: u8 = value | (1 << nth_bit);
                self.write(op2, result);
            }
            RES(nth_bit, op2) => {
                let value: u8 = self.read(op2.clone());
                let result: u8 = value & !(1 << nth_bit);
                self.write(op2, result);
            }

            NOP => {}
            HALT => {
                self.halted = true;
            }
            STOP => {
                self.stopped = true;
            }
            // TODO why unreachable pattern???
            DI => {
                // TODO
                // self.interrupts_enabled = false;
            }
            EI => {
                // TODO
                // self.interrupts_enabled = true;
            }
        }
    }

    fn jump_allowed(&self, operand: Operand) -> bool {
        match operand {
            Zero => self.register.read_flag(Flags::Zero),
            NoZero => !self.register.read_flag(Flags::Zero),
            Carry => self.register.read_flag(Flags::Carry),
            NoCarry => !self.register.read_flag(Flags::Carry),
            _ => panic!("Invalid operand {:?} for JP instructions", operand),
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
        let instruction = self.decode(opcode);
        self.execute(instruction);
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
                let address: u16 = self.read(*addr);
                let address = address + offset;

                match address {
                    address if HIGH_RAM.contains(&address) => {
                        self.high_ram[ (address - 0xFF80) as usize ]
                    }
                    0xFFFF => self.register.IR,
                    _ => {
                        self.cycle += 4;
                        self.memory[address + offset]
                    }
                }
            }
            Word => {
                let data = self.memory[self.register.PC];
                self.register.PC += 1;
                self.cycle += 4;
                data
            }
            _ => panic!("Cannot read word from operand {:?}", operand),
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
            Memory(addr, offset) => {
                let address: u16 = self.read(*addr);
                let address = address + offset;

                match address {
                    address if HIGH_RAM.contains(&address) => {
                        self.high_ram[ (address - 0xFF80) as usize ] = data
                    }
                    0xFFFF => self.register.IR = data,
                    _ => {
                        self.cycle += 4;
                        self.memory[address + offset] = data
                    }
                }
            }
            _ => panic!("Invalid operand {:?} to write word", operand),
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
            }
            HL => self.register.read_HL(),
            AF => self.register.read_AF(),
            BC => self.register.read_BC(),
            DE => self.register.read_DE(),
            SP => self.register.SP,
            PC => self.register.PC,
            Word => as_u16(0, self.read(operand)),
            DWord => as_u16(self.read(Word), self.read(Word)),
            _ => panic!("Invalid operand {:?} to read double word", operand),
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
            _ => panic!("Invalid operand {:?} to write word", operand),
        }
    }
}

impl PushPop<u8> for CPU {
    fn push(&mut self, data: u8) {
        self.write(Memory(Box::new(SP), 0x0), data);
        self.register.SP -= 1;
    }

    fn pop(&mut self) -> u8 {
        let data = self.read(Memory(Box::new(SP), 0x0));
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

    // #[test]
    // fn should_fetch_opcode() {
    //     let mut cpu = CPU::new(MemorySpace::new(&[0xFF]));
    //     let current_cycle = cpu.cycle;
    //     let current_program_counter = cpu.register.PC;
    //
    //     let opcode = cpu.fetch();
    //
    //     assert_eq!(opcode, 0xFF);
    //     assert_eq!(cpu.cycle, current_cycle + 4);
    //     assert_eq!(cpu.register.PC, current_program_counter + 1);
    // }
}
//
//     #[test]
//     fn should_decode_simple_instruction() {
//         let mut cpu = CPU::new(MemorySpace::new(&[0xFF]));
//         let instruction = cpu.decode(0x06);
//         assert_eq!(instruction, LD8(B, Word));
//     }
//
//     #[test]
//     fn should_decode_complex_instruction() {
//         let mut cpu = CPU::new(MemorySpace::new(&[0x37]));
//         let instruction = cpu.decode(0xCB);
//         assert_eq!(instruction, SWAP(A));
//     }
// }
