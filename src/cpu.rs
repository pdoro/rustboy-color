
use std::ops::Index;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;

type OpCode = u32;
type Address = u32;
type byte = u8;

const BOOT_ROM: [OpCode; 10] = [
    0xF5, 0x00, 0x48, 0x1A, 0x3B,
    0x02, 0x3C, 0x8A, 0xFF, 0x48
];

struct Register {
    A: byte,
    B: byte,
    C: byte,
    D: byte,
    E: byte,
    F: byte,
    H: byte,
    L: byte,

    SP: u16,
    PC: u16
}

struct CPU {
    register: Register,
    cycle: u16
}

enum IORegister {
    SB,
    SC,
    DIV,
    TIMA,
    TMA,
    TAC,
    IF,

    NR_10,
    NR_11,
    NR_12,
    NR_13,
    NR_14,

    NR_21,
    NR_22,
    NR_23,
    NR_24,

    NR_31,
    NR_32,
    NR_33,
    NR_34,

    NR_41,
    NR_42,
    NR_43,
    NR_44,

    NR_50,
    NR_51,
    NR_52,

    WAVE_PATTERN,
    LCDC,
    STAT,
    SCY,
    SCX,
    LY,
    LYC,
    DMA,
    BGP,

    OBP0,
    OBP1,
    WY,
    WX,
    IE
}

struct InternalRAM {
    memory: [byte; 8192]
}

const RAM_START: Address = 0xC000;
const RAM_END:   Address = 0xE000;


impl Index<Address> for InternalRAM {
    type Output = byte;

    fn index(&self, address: Address) -> &<Self as Index<Address>>::Output {
        if address < RAM_START || address > RAM_END {
            panic!("Invalid unsafe memory access to {:#X}", address);
        } else {
            self.memory[ address - RAM_START ];
        }
    }
}

struct VideoRam {
    memory: [byte; 8192] // 8Kb memory
}

