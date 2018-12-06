

pub enum Flags {
    Z,
    N,
    H,
    C
}

pub enum Instruction {
    LD(Box<u16>, u16, u8),
    LDD,
    LDI,
    LDH,
    LDHL,
    PUSH,
    POP,
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    OR,
    XOR,
    CP,
    INC,
    DEC,
    NOP
}