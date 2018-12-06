
enum Flags {
    Z,
    N,
    H,
    C
}

enum Instruction {
    LD,
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

struct Command {
    inst: Instruction,
    op1:  byte,
    op2:  byte,
}

fn process(op: OpCode) -> Command {
    match op {
        0x06 => Command { inst: Instruction::LD, op1: Registers::A, op2: Registers::B },
        0x0E => Command { inst: LD, op1: 2, op2: 3 },
        _ => panic!()
    }
}