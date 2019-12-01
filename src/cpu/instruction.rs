
use std::fmt;

#[derive(Clone)]
pub enum Operand {
    // Cpu registers
    A, B, C, D, E, F, H, L,
    // Cpu double registers
    HL, AF, BC, DE,
    // Special Registers
    SP, PC,
    // Flags
    Zero, NoZero, Carry, NoCarry,

    Bit,
    Word,
    DWord,
    Memory(Box<Operand>, u16),
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
            Operand::Word => write!(f, "Imm Word"),
            Operand::DWord => write!(f, "Imm DWord"),
            Operand::Memory(addr, 0) => write!(f, "({:?})", addr),
            Operand::Memory(addr, offset) => write!(f, "({:?} + 0x{:X})", addr, offset),
            _ => write!(f, "XXX")
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
    ADD8(Operand, Operand),
    ADD16(Operand, Operand),
    ADC(Operand, Operand),
    SUB(Operand),
    SBC(Operand, Operand),
    AND(Operand),
    OR(Operand),
    XOR(Operand),
    CP(Operand),
    INC8(Operand),
    DEC8(Operand),
    INC16(Operand),
    DEC16(Operand),

    // Special Instructions
    SWAP(Operand),
    DAA,
    CPL,
    CCF,
    SCF,
    NOP,
    HALT,
    DI,
    EI,

    RLCA,
    RLA,
    RRCA,
    RRA,
    JP1(Operand),
    JP(Operand, Operand),
    JR1(Operand),
    JR(Operand, Operand),
    CALL1(Operand),
    CALL(Operand, Operand),
    RST(Operand),
    RET_,
    RET(Operand),
    RETI,

    RLC(Operand),
    RL(Operand),
    RRC(Operand),
    RR(Operand),
    SLA(Operand),
    SRA(Operand),
    SRL(Operand),
    BIT(Operand, Operand),
    SET(Operand, Operand),
    RES(Operand, Operand),
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
            0x7E => LD8( A, Memory( Box::new(HL), 0 )),
            0x40 => LD8( B, B),
            0x41 => LD8( B, C),
            0x42 => LD8( B, D),
            0x43 => LD8( B, E),
            0x44 => LD8( B, H),
            0x45 => LD8( B, L),
            0x46 => LD8( B, Memory( Box::new(HL), 0 )),
            0x48 => LD8( C, B),
            0x49 => LD8( C, C),
            0x4A => LD8( C, D),
            0x4B => LD8( C, E),
            0x4C => LD8( C, H),
            0x4D => LD8( C, L),
            0x4E => LD8( C, Memory( Box::new(HL), 0 )),
            0x50 => LD8( D, B),
            0x51 => LD8( D, C),
            0x52 => LD8( D, D),
            0x53 => LD8( D, E),
            0x54 => LD8( D, H),
            0x55 => LD8( D, L),
            0x56 => LD8( D, Memory( Box::new(HL), 0 )),
            0x58 => LD8( E, B),
            0x59 => LD8( E, C),
            0x5A => LD8( E, D),
            0x5B => LD8( E, E),
            0x5C => LD8( E, H),
            0x5D => LD8( E, L),
            0x5E => LD8( E, Memory( Box::new(HL), 0 )),
            0x60 => LD8( H, B),
            0x61 => LD8( H, C),
            0x62 => LD8( H, D),
            0x63 => LD8( H, E),
            0x64 => LD8( H, H),
            0x65 => LD8( H, L),
            0x66 => LD8( H, Memory( Box::new(HL), 0 )),
            0x68 => LD8( L, B),
            0x69 => LD8( L, C),
            0x6A => LD8( L, D),
            0x6B => LD8( L, E),
            0x6C => LD8( L, H),
            0x6D => LD8( L, L),
            0x6E => LD8( L, Memory( Box::new(HL), 0 )),
            0x70 => LD8( Memory( Box::new(HL), 0 ), B),
            0x71 => LD8( Memory( Box::new(HL), 0 ), C),
            0x72 => LD8( Memory( Box::new(HL), 0 ), D),
            0x73 => LD8( Memory( Box::new(HL), 0 ), E),
            0x74 => LD8( Memory( Box::new(HL), 0 ), H),
            0x75 => LD8( Memory( Box::new(HL), 0 ), L),
            0x36 => LD8(Memory( Box::new(HL), 0), Word),

            // LD A,n
            0x7F => LD8( A, A),
            0x78 => LD8( A, B),
            0x79 => LD8( A, C),
            0x7A => LD8( A, D),
            0x7B => LD8( A, E),
            0x7C => LD8( A, H),
            0x7D => LD8( A, L),
            0x0A => LD8( A, Memory( Box::new(BC), 0 )),
            0x1A => LD8( A, Memory( Box::new(DE), 0 )),
            0x7E => LD8( A, Memory( Box::new(HL), 0 )),
            0xFA => LD8( A, Memory( Box::new(DWord), 0 )),
            //0x3E => LD(A,self.register. # )

            // LD n,A
            0x7F => LD8( A, A),
            0x47 => LD8( B, A),
            0x4F => LD8( C, A),
            0x57 => LD8( D, A),
            0x5F => LD8( E, A),
            0x67 => LD8( H, A),
            0x6F => LD8( L, A),
            0x02 => LD8( Memory( Box::new(B), 0 ), A),
            0x12 => LD8( Memory( Box::new(DE), 0 ), A),
            0x77 => LD8( Memory( Box::new(HL), 0 ), A),
            0xEA => LD8(Memory( Box::new(DWord), 0 ), A),

            // LD A,(C)
            0xF2 => LD8( A,Memory( Box::new(C), 0xFF00)),
            // LD (C),A
            0xE2 => LD8( Memory(Box::new(C), 0xFF00),A),

            // LDD A,(HL)
            0x3A => LDD(A, Memory( Box::new(HL), 0 )),
            // LDD (HL),A
            0x32 => LDD(Memory( Box::new(HL), 0 ), A),

            // LDI A,(HL)
            0x2A => LDI(A, Memory( Box::new(HL), 0 )),
            // LDI (HL),A
            0x22 => LDI(Memory( Box::new(HL), 0 ), A),

            // LDH (n),A
            0xE0 => LDH(Memory(Box::new(Word), 0xFF00 ), A),
            // LDH A,(n)
            0xF0 => LDH(A, Memory(Box::new(Word), 0xFF00 )),

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
            0x08 => LD16(Memory(Box::new(DWord), 0), SP),

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
            0x80 => ADD8(A, B),
            0x81 => ADD8(A, C),
            0x82 => ADD8(A, D),
            0x83 => ADD8(A, E),
            0x84 => ADD8(A, H),
            0x85 => ADD8(A, L),
            0x86 => ADD8(A, Memory(Box::new(HL), 0)),
            0x87 => ADD8(A, Word),
            0x87 => ADD8(A, A),

            // ADC A,n
            0x8F => ADC(A,A),
            0x88 => ADC(A,B),
            0x89 => ADC(A,C),
            0x8A => ADC(A,D),
            0x8B => ADC(A,E),
            0x8C => ADC(A,H),
            0x8D => ADC(A,L),
            0x8E => ADC(A,Memory(Box::new(HL), 0)),
            0xCE => ADC(A, Word),

            // SUB n
            0x97 => SUB(A),
            0x90 => SUB(B),
            0x91 => SUB(C),
            0x92 => SUB(D),
            0x93 => SUB(E),
            0x94 => SUB(H),
            0x95 => SUB(L),
            0x96 => SUB(Memory(Box::new(HL), 0)),
            0xD6 => SUB(Word),

            // SBC A,n
            0x9F => SBC(A,A),
            0x98 => SBC(A,B),
            0x99 => SBC(A,C),
            0x9A => SBC(A,D),
            0x9B => SBC(A,E),
            0x9C => SBC(A,H),
            0x9D => SBC(A,L),
            0x9E => SBC(A,Memory(Box::new(HL), 0)),
            //0x?? => SBC(A, Word),

            // AND n
            0xA7 => AND(A),
            0xA0 => AND(B),
            0xA1 => AND(C),
            0xA2 => AND(D),
            0xA3 => AND(E),
            0xA4 => AND(H),
            0xA5 => AND(L),
            0xA6 => AND(Memory(Box::new(HL), 0)),
            0xE6 => AND(Word),

            // OR n
            0xB7 => OR(A),
            0xB0 => OR(B),
            0xB1 => OR(C),
            0xB2 => OR(D),
            0xB3 => OR(E),
            0xB4 => OR(H),
            0xB5 => OR(L),
            0xB6 => OR(Memory(Box::new(HL), 0)),
            0xF6 => OR(Word),

            // XOR n
            0xAF => XOR(A),
            0xA8 => XOR(B),
            0xA9 => XOR(C),
            0xAA => XOR(D),
            0xAB => XOR(E),
            0xAC => XOR(H),
            0xAD => XOR(L),
            0xAE => XOR(Memory(Box::new(HL), 0)),
            0xEE => XOR(Word),

            // CP n
            0xBF => CP(A),
            0xB8 => CP(B),
            0xB9 => CP(C),
            0xBA => CP(D),
            0xBB => CP(E),
            0xBC => CP(H),
            0xBD => CP(L),
            0xBE => CP(Memory(Box::new(HL), 0)),
            0xFE => CP(Word),

            // INC n
            0x3C => INC8(A),
            0x04 => INC8(B),
            0x0C => INC8(C),
            0x14 => INC8(D),
            0x1C => INC8(E),
            0x24 => INC8(H),
            0x2C => INC8(L),
            0x34 => INC8(Memory(Box::new(HL), 0)),

            // DEC n
            0x3D => DEC8(A),
            0x05 => DEC8(B),
            0x0D => DEC8(C),
            0x15 => DEC8(D),
            0x1D => DEC8(E),
            0x25 => DEC8(H),
            0x2D => DEC8(L),
            0x35 => DEC8(Memory(Box::new(HL), 0)),

            // --------------- 16-Bit Arithmetic ---------------

            // ADD HL,n
            0x09 => ADD16(HL,BC),
            0x19 => ADD16(HL,DE),
            0x29 => ADD16(HL,HL),
            0x39 => ADD16(HL,SP),

            // ADD SP,n
            0xE8 => ADD16(SP,Word),

            // INC nn
            0x03 => INC16(BC),
            0x13 => INC16(DE),
            0x23 => INC16(HL),
            0x33 => INC16(SP),

            // DEC nn
            0x0B => DEC16(BC),
            0x1B => DEC16(DE),
            0x2B => DEC16(HL),
            0x3B => DEC16(SP),

            // --------------- Miscellaneous ---------------
            0x27 => DAA,
            0x2F => CPL,
            0x3F => CCF,
            0x37 => SCF,
            0x00 => NOP,
            0x76 => HALT,
            0xF3 => DI,
            0xFB => EI,
            // 0x1000 => STOP,

            // --------------- Rotates & Shifts ---------------

            0x07 => RLCA,
            0x17 => RLA,
            0x0F => RRCA,
            0x1F => RRA,

            // --------------- Jumps ---------------

            // JP nn
            0xC3 => JP1(DWord),

            // JP cc, nn
            0xC2 => JP(NoZero, DWord),
            0xCA => JP(Zero, DWord),
            0xD2 => JP(NoCarry, DWord),
            0xDA => JP(Carry, DWord),

            // JP (HL)
            0xE9 => JP1(Memory(Box::new(HL), 0)),
            // JR n
            0x18 => JR1(Word),

            // JR cc,n
            0x20 => JR(NoZero, DWord),
            0x28 => JR(Zero, DWord),
            0x30 => JR(NoCarry, DWord),
            0x38 => JR(Carry, DWord),

            // --------------- Calls ---------------

            0xCD => CALL1(DWord),

            // CALL cc,n
            0xC4 => CALL(NoZero, DWord),
            0xCC => CALL(Zero, DWord),
            0xD4 => CALL(NoCarry, DWord),
            0xDC => CALL(Carry, DWord),

            // --------------- Restarts ---------------

            // TODO!
            0xC7 => RST(Memory(Box::new(Word), 0)),
            0xCF => RST(Memory(Box::new(Word), 0)),
            0xD7 => RST(Memory(Box::new(Word), 0)),
            0xDF => RST(Memory(Box::new(Word), 0)),
            0xE7 => RST(Memory(Box::new(Word), 0)),
            0xEF => RST(Memory(Box::new(Word), 0)),
            0xF7 => RST(Memory(Box::new(Word), 0)),
            0xFF => RST(Memory(Box::new(Word), 0)),

            // --------------- Returns ---------------

            0xC9 => RET_,

            // RET cc
            0xC0 => RET(NoZero),
            0xC8 => RET(Zero),
            0xD0 => RET(NoCarry),
            0xD8 => RET(Carry),

            0xD9 => RETI,
            
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
            0xCB36 => SWAP(Memory(Box::new(HL), 0)),

            // RLC n
            0xCB07 => RLC(A),
            0xCB00 => RLC(B),
            0xCB01 => RLC(C),
            0xCB02 => RLC(D),
            0xCB03 => RLC(E),
            0xCB04 => RLC(H),
            0xCB05 => RLC(L),
            0xCB06 => RLC(Memory(Box::new(HL), 0)),

            // RL n
            0xCB17 => RL(A),
            0xCB10 => RL(B),
            0xCB11 => RL(C),
            0xCB12 => RL(D),
            0xCB13 => RL(E),
            0xCB14 => RL(H),
            0xCB15 => RL(L),
            0xCB16 => RL(Memory(Box::new(HL), 0)),

            // RRC n
            0xCB0F => RRC(A),
            0xCB08 => RRC(B),
            0xCB09 => RRC(C),
            0xCB0A => RRC(D),
            0xCB0B => RRC(E),
            0xCB0C => RRC(H),
            0xCB0D => RRC(L),
            0xCB0E => RRC(Memory(Box::new(HL), 0)),

            // RR n
            0xCB1F => RR(A),
            0xCB18 => RR(B),
            0xCB19 => RR(C),
            0xCB1A => RR(D),
            0xCB1B => RR(E),
            0xCB1C => RR(H),
            0xCB1D => RR(L),
            0xCB1E => RR(Memory(Box::new(HL), 0)),

            // SLA n
            0xCB27 => SLA(A),
            0xCB20 => SLA(B),
            0xCB21 => SLA(C),
            0xCB22 => SLA(D),
            0xCB23 => SLA(E),
            0xCB24 => SLA(H),
            0xCB25 => SLA(L),
            0xCB26 => SLA(Memory(Box::new(HL), 0)),

            // SRA n
            0xCB2F => SRA(A),
            0xCB28 => SRA(B),
            0xCB29 => SRA(C),
            0xCB2A => SRA(D),
            0xCB2B => SRA(E),
            0xCB2C => SRA(H),
            0xCB2D => SRA(L),
            0xCB2E => SRA(Memory(Box::new(HL), 0)),

            // SRL n
            0xCB3F => SRL(A),
            0xCB38 => SRL(B),
            0xCB39 => SRL(C),
            0xCB3A => SRL(D),
            0xCB3B => SRL(E),
            0xCB3C => SRL(H),
            0xCB3D => SRL(L),
            0xCB3E => SRL(Memory(Box::new(HL), 0)),

            // Bit Opcodes

            // BIT b,r
            0xCB47 => BIT(Bit, A),
            0xCB40 => BIT(Bit, B),
            0xCB41 => BIT(Bit, C),
            0xCB42 => BIT(Bit, D),
            0xCB43 => BIT(Bit, E),
            0xCB44 => BIT(Bit, H),
            0xCB45 => BIT(Bit, L),
            0xCB46 => BIT(Bit, Memory(Box::new(HL), 0)),

            // SET b,r
            0xCBC7 => SET(Bit, A),
            0xCBC0 => SET(Bit, B),
            0xCBC1 => SET(Bit, C),
            0xCBC2 => SET(Bit, D),
            0xCBC3 => SET(Bit, E),
            0xCBC4 => SET(Bit, H),
            0xCBC5 => SET(Bit, L),
            0xCBC6 => SET(Bit, Memory(Box::new(HL), 0)),

            // RES b,r
            0xCB87 => RES(Bit, A),
            0xCB80 => RES(Bit, B),
            0xCB81 => RES(Bit, C),
            0xCB82 => RES(Bit, D),
            0xCB83 => RES(Bit, E),
            0xCB84 => RES(Bit, H),
            0xCB85 => RES(Bit, L),
            0xCB86 => RES(Bit, Memory(Box::new(HL), 0)),

            _ => NOP
        }
    }
}
