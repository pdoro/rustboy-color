use crate::memory::MemorySpace;
use crate::soc::cpu::{ReadWrite, CPU};
use crate::soc::instruction::Operand;
use crate::soc::register::*;

macro_rules! test_instruction {
    ( $cpu:ident, $reg:expr, $expected_val:expr, $expected_cycle:expr ) => {
        $cpu.exec_single_instruction();
        assert_eq!($reg, $expected_val);
        assert_eq!($cpu.cycle, $expected_cycle);
    };
}

pub fn print_bit(x: u8) {
    println!("{:#010b}", x);
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
    //assert_eq!(soc.cycle, 8); TODO review
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

#[test]
fn should_swap() {
    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x37]));
    cpu.register.A = 0xAB;
    test_instruction!(cpu, cpu.register.A, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x30]));
    cpu.register.B = 0xAB;
    test_instruction!(cpu, cpu.register.B, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x31]));
    cpu.register.C = 0xAB;
    test_instruction!(cpu, cpu.register.C, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x32]));
    cpu.register.D = 0xAB;
    test_instruction!(cpu, cpu.register.D, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x33]));
    cpu.register.E = 0xAB;
    test_instruction!(cpu, cpu.register.E, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x34]));
    cpu.register.H = 0xAB;
    test_instruction!(cpu, cpu.register.H, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x35]));
    cpu.register.L = 0xAB;
    test_instruction!(cpu, cpu.register.L, 0xBA, 8);

    let mut cpu = CPU::new(MemorySpace::new(&[0xCB, 0x36, 0xAB]));
    cpu.register.write_HL(2);
    cpu.exec_single_instruction();
    assert_eq!(cpu.memory[2], 0xBA);
    assert_eq!(cpu.cycle, 16);
}

#[test]
fn should_complement_register() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x2F]));
    cpu.register.A = 0x00;
    test_instruction!(cpu, cpu.register.A, 0xFF, 4);
}

#[test]
fn should_complement_carry_flag() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x3F]));
    cpu.register.F = 0b01100000;
    test_instruction!(cpu, cpu.register.F, 0b00010000, 4);

    let mut cpu = CPU::new(MemorySpace::new(&[0x3F]));
    cpu.register.F = 0b11110000;
    test_instruction!(cpu, cpu.register.F, 0b10000000, 4);
}

#[test]
fn should_set_carry_flag() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x37]));
    cpu.register.F = 0b01100000;
    test_instruction!(cpu, cpu.register.F, 0b00010000, 4);
}

#[test]
fn should_nope() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x00]));
    test_instruction!(cpu, cpu.register.A, 0x0, 4);
}

///////////////////// Rotates and Shift /////////////////////

#[test]
fn should_rotate_left() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x07]));
    cpu.register.A = 0b10010101;
    test_instruction!(cpu, cpu.register.A, 0b00101010, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), true);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x07]));
    cpu.register.A = 0b01010101;
    test_instruction!(cpu, cpu.register.A, 0b10101010, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), false);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x07]));
    cpu.register.A = 0x00;
    test_instruction!(cpu, cpu.register.A, 0x00, 4);
    assert_eq!(cpu.register.read_flag(Flags::Zero), true);
}

#[test]
fn should_rotate_left_through_carry() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x17]));
    cpu.register.A = 0b10010101;
    test_instruction!(cpu, cpu.register.A, 0b00101010, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), true);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x17]));
    cpu.register.F = 0b00010000; // carry on
    cpu.register.A = 0b01010101;
    test_instruction!(cpu, cpu.register.A, 0b10101011, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), false);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x17]));
    cpu.register.F = 0b00010000; // carry on
    cpu.register.A = 0b11010101;
    test_instruction!(cpu, cpu.register.A, 0b10101011, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), true);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);
}

#[test]
fn should_rotate_right() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x0F]));
    cpu.register.A = 0b10100101;
    test_instruction!(cpu, cpu.register.A, 0b01010010, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), true);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x0F]));
    cpu.register.A = 0b10101010;
    test_instruction!(cpu, cpu.register.A, 0b01010101, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), false);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x0F]));
    cpu.register.A = 0x00;
    test_instruction!(cpu, cpu.register.A, 0x00, 4);
    assert_eq!(cpu.register.read_flag(Flags::Zero), true);
}

#[test]
fn should_rotate_right_through_carry() {
    let mut cpu = CPU::new(MemorySpace::new(&[0x1F]));
    cpu.register.A = 0b10100101;
    test_instruction!(cpu, cpu.register.A, 0b01010010, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), true);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x1F]));
    cpu.register.F = 0b00010000; // carry on
    cpu.register.A = 0b10101010;
    test_instruction!(cpu, cpu.register.A, 0b11010101, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), false);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);

    let mut cpu = CPU::new(MemorySpace::new(&[0x1F]));
    cpu.register.F = 0b00010000; // carry on
    cpu.register.A = 0b10101011;
    test_instruction!(cpu, cpu.register.A, 0b11010101, 4);
    assert_eq!(cpu.register.read_flag(Flags::Carry), true);
    assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
    assert_eq!(cpu.register.read_flag(Flags::HalfCarry), false);
}

#[test]
fn should_test_bits() {
    let args = [
        //  register, test base-op
        (Operand::A, 0x47),
        (Operand::B, 0x40),
        (Operand::C, 0x41),
        (Operand::D, 0x42),
        (Operand::E, 0x43),
        (Operand::H, 0x44),
        (Operand::L, 0x45),
    ];

    //test_bits!(Memory(Box::new(HL), 0), 0x46);

    for iter in 0..7_u8 {
        for (operand, test_base_opcode) in &args {
            // TEST NEGATIVE BIT
            let opcode = test_base_opcode + iter * 8;

            let mut cpu = CPU::new(MemorySpace::new(&[0xCB, opcode]));
            cpu.write(operand.clone(), 0x00 as u8);
            cpu.exec_single_instruction();
            assert_eq!(cpu.cycle, 8);
            assert_eq!(cpu.register.read_flag(Flags::Zero), true);
            assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
            assert_eq!(cpu.register.read_flag(Flags::HalfCarry), true);
            // TEST POSITIVE BIT
            let mut cpu = CPU::new(MemorySpace::new(&[0xCB, opcode]));
            cpu.write(operand.clone(), (1 << iter) as u8);
            cpu.exec_single_instruction();
            assert_eq!(cpu.cycle, 8);
            assert_eq!(cpu.register.read_flag(Flags::Zero), false);
            assert_eq!(cpu.register.read_flag(Flags::Subtract), false);
            assert_eq!(cpu.register.read_flag(Flags::HalfCarry), true);
        }
    }
}

#[test]
fn should_set_bits() {
    let args = [
        //  register, set base op
        (Operand::A, 0xC7),
        (Operand::B, 0xC0),
        (Operand::C, 0xC1),
        (Operand::D, 0xC2),
        (Operand::E, 0xC3),
        (Operand::H, 0xC4),
        (Operand::L, 0xC5),
    ];

    for iter in 0..7_u8 {
        for (operand, set_base_opcode) in &args {
            // SET NEGATIVE BIT
            let opcode = set_base_opcode + iter * 8;
            let mut cpu = CPU::new(MemorySpace::new(&[0xCB, opcode]));
            cpu.exec_single_instruction();
            assert_eq!(cpu.cycle, 8);
            let value: u8 = cpu.read(operand.clone());
            assert_ne!(value & (1 << iter), 0x00);
        }
    }
}

#[test]
fn should_reset_bits() {
    let args = [
        //  register, reset base op
        (Operand::A, 0x87),
        (Operand::B, 0x80),
        (Operand::C, 0x81),
        (Operand::D, 0x82),
        (Operand::E, 0x83),
        (Operand::H, 0x84),
        (Operand::L, 0x85),
    ];

    for iter in 0..7_u8 {
        for (operand, reset_base_opcode) in &args {
            // SET NEGATIVE BIT
            let opcode = reset_base_opcode + iter * 8;
            let mut cpu = CPU::new(MemorySpace::new(&[0xCB, opcode]));

            // Ensure bits are set
            cpu.register.write_AF(0xFF);
            cpu.register.write_HL(0xFF);
            cpu.register.write_DE(0xFF);
            cpu.register.write_BC(0xFF);

            cpu.exec_single_instruction();
            assert_eq!(cpu.cycle, 8);
            let value: u8 = cpu.read(operand.clone());
            assert_eq!(value & (1 << iter), 0x00);
        }
    }
}
