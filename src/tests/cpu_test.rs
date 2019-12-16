pub use crate::cpu::cpu::CPU;
use crate::cpu::cpu::ReadWrite;
pub use crate::memory::MemorySpace;
pub use crate::cpu::instruction::Operand;

macro_rules! test_instruction {
    ( $cpu:ident, $reg:expr, $expected_val:expr, $expected_cycle:expr ) => {
        $cpu.exec_single_instruction();
        assert_eq!($reg, $expected_val);
        assert_eq!($cpu.cycle, $expected_cycle);
    };
}

#[test]
fn should_load_immediate_value() {

    // LD A, Word
    let mut cpu = CPU::new(MemorySpace::new((&[0x06, 0x11])));
    test_instruction!(cpu, cpu.register.B, 0x11, 8);

    let mut cpu = CPU::new(MemorySpace::new((&[0x0E, 0x11])));
    test_instruction!(cpu, cpu.register.C, 0x11, 8);

    let mut cpu = CPU::new(MemorySpace::new((&[0x16, 0x11])));
    test_instruction!(cpu, cpu.register.D, 0x11, 8);

    let mut cpu = CPU::new(MemorySpace::new((&[0x1E, 0x11])));
    test_instruction!(cpu, cpu.register.E, 0x11, 8);

    let mut cpu = CPU::new(MemorySpace::new((&[0x26, 0x11])));
    test_instruction!(cpu, cpu.register.H, 0x11, 8);

    let mut cpu = CPU::new(MemorySpace::new((&[0x2E, 0x11])));
    test_instruction!(cpu, cpu.register.L, 0x11, 8);
}

#[test]
fn should_load_between_registers() {

    // ------------
    // LD A, r2
    // ------------
    let mut cpu = CPU::new(MemorySpace::new((&[0x7F])));
    cpu.register.A = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x78])));
    cpu.register.B = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x79])));
    cpu.register.C = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7A])));
    cpu.register.D = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7B])));
    cpu.register.E = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7C])));
    cpu.register.H = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7D])));
    cpu.register.L = 0xFF;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7E, 0xFF])));
    cpu.register.write_HL(0x01);
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.A, 0xFF);
    assert_eq!(cpu.cycle, 8);
}

#[test]
fn should_load_immediate_double_value() {

    // LD A, Word
    let mut cpu = CPU::new(MemorySpace::new((&[0x01, 0x11, 0x22])));
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.B, 0x11);
    assert_eq!(cpu.register.C, 0x22);
    assert_eq!(cpu.cycle, 12);

    let mut cpu = CPU::new(MemorySpace::new((&[0x11, 0x11, 0x22])));
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.D, 0x11);
    assert_eq!(cpu.register.E, 0x22);
    assert_eq!(cpu.cycle, 12);

    let mut cpu = CPU::new(MemorySpace::new((&[0x21, 0x11, 0x22])));
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.H, 0x11);
    assert_eq!(cpu.register.L, 0x22);
    assert_eq!(cpu.cycle, 12);

    let mut cpu = CPU::new(MemorySpace::new((&[0x31, 0x11, 0x22])));
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, 0x1122);
    assert_eq!(cpu.cycle, 12);

    let mut cpu = CPU::new(MemorySpace::new((&[0xF9])));
    cpu.register.write_HL(0x1234);
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, 0x1234);
    //assert_eq!(cpu.cycle, 8); TODO review
}

#[test]
fn should_push() {

    let mut cpu = CPU::new(MemorySpace::new(&[0xF5, 0x00, 0x00]));
    cpu.register.write_AF(0x1234);
    cpu.register.SP = 0x02;
    let prev_sp = cpu.register.SP;

    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, prev_sp - 2);
    assert_eq!(cpu.memory[1], 0x12);
    assert_eq!(cpu.memory[2], 0x34);
    assert_eq!(cpu.cycle, 12);

    //////////////////////////////////////////////////

    let mut cpu = CPU::new(MemorySpace::new(&[0xC5, 0x00, 0x00]));
    cpu.register.write_BC(0x1234);
    cpu.register.SP = 0x02;
    let prev_sp = cpu.register.SP;

    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, prev_sp - 2);
    assert_eq!(cpu.memory[1], 0x12);
    assert_eq!(cpu.memory[2], 0x34);
    assert_eq!(cpu.cycle, 12);

    //////////////////////////////////////////////////

    let mut cpu = CPU::new(MemorySpace::new(&[0xD5, 0x00, 0x00]));
    cpu.register.write_DE(0x1234);
    cpu.register.SP = 0x02;
    let prev_sp = cpu.register.SP;

    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, prev_sp - 2);
    assert_eq!(cpu.memory[1], 0x12);
    assert_eq!(cpu.memory[2], 0x34);
    assert_eq!(cpu.cycle, 12);

    //////////////////////////////////////////////////

    let mut cpu = CPU::new(MemorySpace::new(&[0xE5, 0x00, 0x00]));
    cpu.register.write_HL(0x1234);
    cpu.register.SP = 0x02;
    let prev_sp = cpu.register.SP;

    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, prev_sp - 2);
    assert_eq!(cpu.memory[1], 0x12);
    assert_eq!(cpu.memory[2], 0x34);
    assert_eq!(cpu.cycle, 12);
}

#[test]
fn should_pop() {

    let mut cpu = CPU::new(MemorySpace::new(&[0xF1, 0x12, 0x34]));
    cpu.register.SP = 1;
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, 3);
    assert_eq!(cpu.register.A, 0x12);
    assert_eq!(cpu.register.F, 0x34);
    assert_eq!(cpu.cycle, 12);

    //////////////////////////////////////////////////

    let mut cpu = CPU::new(MemorySpace::new(&[0xC1, 0x12, 0x34]));
    cpu.register.SP = 1;
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, 3);
    assert_eq!(cpu.register.B, 0x12);
    assert_eq!(cpu.register.C, 0x34);
    assert_eq!(cpu.cycle, 12);

    //////////////////////////////////////////////////

    let mut cpu = CPU::new(MemorySpace::new(&[0xD1, 0x12, 0x34]));
    cpu.register.SP = 1;
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, 3);
    assert_eq!(cpu.register.D, 0x12);
    assert_eq!(cpu.register.E, 0x34);
    assert_eq!(cpu.cycle, 12);

    //////////////////////////////////////////////////

    let mut cpu = CPU::new(MemorySpace::new(&[0xE1, 0x12, 0x34]));
    cpu.register.SP = 1;
    cpu.exec_single_instruction();
    assert_eq!(cpu.register.SP, 3);
    assert_eq!(cpu.register.H, 0x12);
    assert_eq!(cpu.register.L, 0x34);
    assert_eq!(cpu.cycle, 12);
}

#[test]
fn should_add() {

    let mut cpu = CPU::new(MemorySpace::new(&[0x87]));
    cpu.register.A = 1;
    test_instruction!(cpu, cpu.register.A, 2, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x80]));
    cpu.register.B = 1;
    test_instruction!(cpu, cpu.register.A, 1, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x81]));
    cpu.register.C = 1;
    test_instruction!(cpu, cpu.register.A, 1, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x82]));
    cpu.register.D = 1;
    test_instruction!(cpu, cpu.register.A, 1, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x83]));
    cpu.register.E = 1;
    test_instruction!(cpu, cpu.register.A, 1, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x84]));
    cpu.register.H = 1;
    test_instruction!(cpu, cpu.register.A, 1, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x85]));
    cpu.register.L = 1;
    test_instruction!(cpu, cpu.register.A, 1, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x86, 0x12]));
    cpu.register.write_HL(1);
    test_instruction!(cpu, cpu.register.A, 0x12, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xC6, 0x34]));
    test_instruction!(cpu, cpu.register.A, 0x34, 8);
}

#[test]
fn should_sub() {

    let mut cpu = CPU::new(MemorySpace::new(&[0x97]));
    cpu.register.A = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x90]));
    cpu.register.A = 1;
    cpu.register.B = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x91]));
    cpu.register.A = 1;
    cpu.register.C = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x92]));
    cpu.register.A = 1;
    cpu.register.D = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x93]));
    cpu.register.A = 1;
    cpu.register.E = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x94]));
    cpu.register.A = 1;
    cpu.register.H = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x95]));
    cpu.register.A = 1;
    cpu.register.L = 1;
    test_instruction!(cpu, cpu.register.A, 0, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x96, 0x01]));
    cpu.register.A = 1;
    cpu.register.write_HL(1);
    test_instruction!(cpu, cpu.register.A, 0x00, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xD6, 0x01]));
    cpu.register.A = 1;
    test_instruction!(cpu, cpu.register.A, 0x00, 8);
}
