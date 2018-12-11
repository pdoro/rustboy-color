
use crate::memory::Address;
use std::fmt;

pub enum Operand {
    // Cpu registers
    A, B, C, D, E, F, H, L,
    // Cpu double registers
    HL, AF, BC, DE,
    // Special Registers
    SP, PC,

    Word,
    DWord,
    Memory(Box<Operand>),
    OffsetMemory(u16, Box<Operand>)
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::A => write!(f, "A"),
            Operand::B => write!(f, "B"),
            Operand::C => write!(f, "C"),
            Operand::D => write!(f, "D"),
            Operand::E => write!(f, "E"),
            Operand::F => write!(f, "F"),
            Operand::H => write!(f, "H"),
            Operand::L => write!(f, "L"),
            Operand::HL => write!(f, "HL"),
            Operand::AF => write!(f, "AF"),
            Operand::BC => write!(f, "BC"),
            Operand::DE => write!(f, "DE"),
            Operand::SP => write!(f, "SP"),
            Operand::PC => write!(f, "PC"),
            Operand::Word => write!(f, "#"),
            Operand::DWord => write!(f, "##"),
            Operand::Memory(addr) => write!(f, "({:?})", addr),
            Operand::OffsetMemory(offset, addr) => write!(f, "(0x{:X} + {:?})", offset, addr),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    LD8(Operand, Operand),
    LD16(Operand, Operand),
    LDD(Operand, Operand),
    LDI(Operand, Operand),
    LDH(Operand, Operand),
    LDHL(Operand, Operand),
    PUSH(Operand),
    POP(Operand),
    ADD(Operand, Operand),
    ADC(Operand, Operand),
    SUB(Operand),
    SBC(Operand, Operand),
    AND(Operand),
    OR(Operand),
    XOR(Operand),
    CP(Operand),
    INC(Operand),
    DEC(Operand),

    // Special Instructions
    SWAP(Operand),
    HALT,
    STOP,
    NOP,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {

        use super::instruction::Instruction::*;
        use super::instruction::Operand::*;

        match opcode {

            // --------------- 8 bit LOAD ---------------

            // LD nn,n
            0x06 => LD8(A, Word),
            0x0E => LD8(C, Word),
            0x16 => LD8(D, Word),
            0x1E => LD8(E, Word),
            0x26 => LD8(H, Word),
            0x2E => LD8(L, Word),

            // LD r1,r2
            0x7F => LD8( A, A),
            0x78 => LD8( A, B),
            0x79 => LD8( A, C),
            0x7A => LD8( A, D),
            0x7B => LD8( A, E),
            0x7C => LD8( A, H),
            0x7D => LD8( A, L),
            0x7E => LD8( A, Memory( Box::new(HL) )),
            0x40 => LD8( B, B),
            0x41 => LD8( B, C),
            0x42 => LD8( B, D),
            0x43 => LD8( B, E),
            0x44 => LD8( B, H),
            0x45 => LD8( B, L),
            0x46 => LD8( B, Memory( Box::new(HL) )),
            0x48 => LD8( C, B),
            0x49 => LD8( C, C),
            0x4A => LD8( C, D),
            0x4B => LD8( C, E),
            0x4C => LD8( C, H),
            0x4D => LD8( C, L),
            0x4E => LD8( C, Memory( Box::new(HL) )),
            0x50 => LD8( D, B),
            0x51 => LD8( D, C),
            0x52 => LD8( D, D),
            0x53 => LD8( D, E),
            0x54 => LD8( D, H),
            0x55 => LD8( D, L),
            0x56 => LD8( D, Memory( Box::new(HL) )),
            0x58 => LD8( E, B),
            0x59 => LD8( E, C),
            0x5A => LD8( E, D),
            0x5B => LD8( E, E),
            0x5C => LD8( E, H),
            0x5D => LD8( E, L),
            0x5E => LD8( E, Memory( Box::new(HL) )),
            0x60 => LD8( H, B),
            0x61 => LD8( H, C),
            0x62 => LD8( H, D),
            0x63 => LD8( H, E),
            0x64 => LD8( H, H),
            0x65 => LD8( H, L),
            0x66 => LD8( H, Memory( Box::new(HL) )),
            0x68 => LD8( L, B),
            0x69 => LD8( L, C),
            0x6A => LD8( L, D),
            0x6B => LD8( L, E),
            0x6C => LD8( L, H),
            0x6D => LD8( L, L),
            0x6E => LD8( L, Memory( Box::new(HL) )),
            0x70 => LD8( Memory( Box::new(HL) ), B),
            0x71 => LD8( Memory( Box::new(HL) ), C),
            0x72 => LD8( Memory( Box::new(HL) ), D),
            0x73 => LD8( Memory( Box::new(HL) ), E),
            0x74 => LD8( Memory( Box::new(HL) ), H),
            0x75 => LD8( Memory( Box::new(HL) ), L),
            0x36 => LD8(Memory( Box::new(HL) ), Word),

            // LD A,n
            0x7F => LD8( A, A),
            0x78 => LD8( A, B),
            0x79 => LD8( A, C),
            0x7A => LD8( A, D),
            0x7B => LD8( A, E),
            0x7C => LD8( A, H),
            0x7D => LD8( A, L),
            0x0A => LD8( A, Memory( Box::new(BC) )),
            0x1A => LD8( A, Memory( Box::new(DE) )),
            0x7E => LD8( A, Memory( Box::new(HL) )),
            0xFA => LD8( A, Memory( Box::new(DWord) )),
            //0x3E => LD(A,self.register. # )

            // LD n,A
            0x7F => LD8( A, A),
            0x47 => LD8( B, A),
            0x4F => LD8( C, A),
            0x57 => LD8( D, A),
            0x5F => LD8( E, A),
            0x67 => LD8( H, A),
            0x6F => LD8( L, A),
            0x02 => LD8( Memory( Box::new(B) ), A),
            0x12 => LD8( Memory( Box::new(DE) ), A),
            0x77 => LD8( Memory( Box::new(HL) ), A),
            0xEA => LD8(Memory( Box::new(DWord) ), A),

            // LD A,(C)
            0xF2 => LD8( A,OffsetMemory(0xFF00, Box::new(C))),
            // LD (C),A
            0xE2 => LD8( OffsetMemory(0xFF00, Box::new(C)),A),

            // LDD A,(HL)
            0x3A => LDD(A, Memory( Box::new(HL) )),
            // LDD (HL),A
            0x32 => LDD(Memory( Box::new(HL) ), A),

            // LDI A,(HL)
            0x2A => LDI(A, Memory( Box::new(HL) )),
            // LDI (HL),A
            0x22 => LDI(Memory( Box::new(HL) ), A),

            // LDH (n),A
            0xE0 => LDH(OffsetMemory(0xFF00,  Box::new(Word) ), A),
            // LDH A,(n)
            0xF0 => LDH(A, OffsetMemory(0xFF00,  Box::new(Word) )),

            // --------------- 16 bit LOAD ---------------

            // LD n,nn
            0x01 => LD16(BC, DWord),
            0x11 => LD16(DE, DWord),
            0x21 => LD16(HL, DWord),
            0x31 => LD16(SP, DWord),

            // LD SP,HL
            0xF9 => LD16(SP, HL),

            // LDHL SP,n
            0xF8 => LDHL(SP, Word),

            // LD (nn),SP
            0x08 => LD16(Memory(Box::new(DWord)), SP),

            // --------------- PUSH & POP ---------------

            // PUSH nn
            0xF5 => PUSH(AF),
            0xC5 => PUSH(BC),
            0xD5 => PUSH(DE),
            0xE5 => PUSH(HL),

            // POP nn
            0xF1 => POP(AF),
            0xC1 => POP(BC),
            0xD1 => POP(DE),
            0xE1 => POP(HL),

            // --------------- 8 BIT ALU ---------------

            // ADD A,n
            0x80 => ADD(A, B),
            0x81 => ADD(A, C),
            0x82 => ADD(A, D),
            0x83 => ADD(A, E),
            0x84 => ADD(A, H),
            0x85 => ADD(A, L),
            0x86 => ADD(A, Memory(Box::new(HL))),
            0x87 => ADD(A, Word),
            0x87 => ADD(A, A),

            // ADC A,n
            0x8F => ADC(A,A),
            0x88 => ADC(A,B),
            0x89 => ADC(A,C),
            0x8A => ADC(A,D),
            0x8B => ADC(A,E),
            0x8C => ADC(A,H),
            0x8D => ADC(A,L),
            0x8E => ADC(A,Memory(Box::new(HL))),
            0xCE => ADC(A, Word),

            // SUB n
            0x97 => SUB(A),
            0x90 => SUB(B),
            0x91 => SUB(C),
            0x92 => SUB(D),
            0x93 => SUB(E),
            0x94 => SUB(H),
            0x95 => SUB(L),
            0x96 => SUB(Memory(Box::new(HL))),
            0xD6 => SUB(Word),

            // SBC A,n
            0x9F => SBC(A,A),
            0x98 => SBC(A,B),
            0x99 => SBC(A,C),
            0x9A => SBC(A,D),
            0x9B => SBC(A,E),
            0x9C => SBC(A,H),
            0x9D => SBC(A,L),
            0x9E => SBC(A,Memory(Box::new(HL))),
            //0x?? => SBC(A, Word),

            // AND n
            0xA7 => AND(A),
            0xA0 => AND(B),
            0xA1 => AND(C),
            0xA2 => AND(D),
            0xA3 => AND(E),
            0xA4 => AND(H),
            0xA5 => AND(L),
            0xA6 => AND(Memory(Box::new(HL))),
            0xE6 => AND(Word),

            // OR n
            0xB7 => OR(A),
            0xB0 => OR(B),
            0xB1 => OR(C),
            0xB2 => OR(D),
            0xB3 => OR(E),
            0xB4 => OR(H),
            0xB5 => OR(L),
            0xB6 => OR(Memory(Box::new(HL))),
            0xF6 => OR(Word),

            // XOR n
            0xAF => XOR(A),
            0xA8 => XOR(B),
            0xA9 => XOR(C),
            0xAA => XOR(D),
            0xAB => XOR(E),
            0xAC => XOR(H),
            0xAD => XOR(L),
            0xAE => XOR(Memory(Box::new(HL))),
            0xEE => XOR(Word),

            // CP n
            0xBF => CP(A),
            0xB8 => CP(B),
            0xB9 => CP(C),
            0xBA => CP(D),
            0xBB => CP(E),
            0xBC => CP(H),
            0xBD => CP(L),
            0xBE => CP(Memory(Box::new(HL))),
            0xFE => CP(Word),

            // INC n
            0x3C => INC(A),
            0x04 => INC(B),
            0x0C => INC(C),
            0x14 => INC(D),
            0x1C => INC(E),
            0x24 => INC(H),
            0x2C => INC(L),
            0x34 => INC(Memory(Box::new(HL))),

            // DEC n
            0x3D => DEC(A),
            0x05 => DEC(B),
            0x0D => DEC(C),
            0x15 => DEC(D),
            0x1D => DEC(E),
            0x25 => DEC(H),
            0x2D => DEC(L),
            0x35 => DEC(Memory(Box::new(HL))),

            // --------------- 16-Bit Arithmetic ---------------

            // ADD HL,n
            0x09 => ADD(HL,BC),
            0x19 => ADD(HL,DE),
            0x29 => ADD(HL,HL),
            0x39 => ADD(HL,SP),

            // ADD SP,n
            0xE8 => ADD(SP,Word),

            // INC nn
            0x03 => INC(BC),
            0x13 => INC(DE),
            0x23 => INC(HL),
            0x33 => INC(SP),

            // DEC nn
            0x0B => DEC(BC),
            0x1B => DEC(DE),
            0x2B => DEC(HL),
            0x3B => DEC(SP),

            _ => NOP
        }
    }
}

impl From<u16> for Instruction {
    fn from(opcode: u16) -> Self {
        use super::instruction::Instruction::*;
        use super::instruction::Operand::*;

        match opcode {

            // SWAP n
            0xCB37 => SWAP(A),
            0xCB30 => SWAP(B),
            0xCB31 => SWAP(C),
            0xCB32 => SWAP(D),
            0xCB33 => SWAP(E),
            0xCB34 => SWAP(H),
            0xCB35 => SWAP(L),
            0xCB36 => SWAP(Memory(Box::new(HL))),



            _ => NOP
        }
    }
}
