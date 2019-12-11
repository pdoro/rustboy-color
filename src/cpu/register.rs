
use crate::utils::{as_u16};
use crate::cpu::instruction::Operand;
use std::fmt;
use std::ops;

#[allow(non_snake_case)]
pub struct Registers {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub F: u8,
    pub H: u8,
    pub L: u8,

    pub SP: u16,
    pub PC: u16,
}

pub enum Flags {
   Zero = 0,
   Subtract = 1,
   HalfCarry = 2,
   Carry = 3,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            H: 0,
            L: 0,

            SP: 0,
            PC: 0,
        }
    }
}

#[allow(non_snake_case)]
impl Registers {

    // Special registers
    pub fn read_AF(&self) -> u16 {
        as_u16(self.A, self.F)
    }
    pub fn read_HL(&self) -> u16 {
        as_u16(self.H, self.L)
    }
    pub fn read_BC(&self) -> u16 {
        as_u16(self.B, self.C)
    }
    pub fn read_DE(&self) -> u16 {
        as_u16(self.D, self.E)
    }

    // Write operations
    pub fn write_AF(&mut self, data: u16) {
        self.A = (data >> 8) as u8;
        self.F = data as u8;
    }

    pub fn write_HL(&mut self, data: u16) {
        self.H = (data >> 8) as u8;
        self.L = data as u8;
    }

    pub fn write_BC(&mut self, data: u16) {
        self.B = (data >> 8) as u8;
        self.C = data as u8;
    }

    pub fn write_DE(&mut self, data: u16) {
        self.D = (data >> 8) as u8;
        self.E = data as u8;
    }

    // FLAG OPERATIONS

    pub fn read_flag(&self, flag: Flags) -> bool {
        self.F & (1 << flag as u8) != 0
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.F |= (1 << flag as u8);
    }

    pub fn reset_flag(&mut self, flag: Flags) {
        self.F &= !(1 << flag as u8);
    }
}

impl ops::IndexMut<Operand> for Registers {

    fn index_mut(&mut self, register: Operand) -> &mut u8 {
        match register {
            Operand::A => &mut self.A,
            Operand::B => &mut self.B,
            Operand::C => &mut self.C,
            Operand::D => &mut self.D,
            Operand::E => &mut self.E,
            Operand::F => &mut self.F,
            Operand::H => &mut self.H,
            Operand::L => &mut self.L,
            _ => panic!("Invalid register {:?}", register),
        }
    }
}

impl ops::Index<Operand> for Registers {
    type Output = u8;

    fn index(& self, register: Operand) -> & Self::Output {
        match register {
            Operand::A => & self.A,
            Operand::B => & self.B,
            Operand::C => & self.C,
            Operand::D => & self.D,
            Operand::E => & self.E,
            Operand::F => & self.F,
            Operand::H => & self.H,
            Operand::L => & self.L,
            _ => panic!("Invalid register {:?}", register),
        }
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Register(A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, F: {:#X}, H: {:#X}, L: {:#X}, SP: {:#X}, PC: {:#X})",
               self.A, self.B, self.C, self.D, self.E, self.F, self.H, self.L, self.SP, self.PC)
    }
}

#[cfg(test)]
mod register_tests {
    use super::*;

    #[test]
    fn should_set_flags() {
        let mut register = Registers::default();

        register.set_flag(Flags::Zero);
        assert_eq!(register.F, 0b00000001);
        register.set_flag(Flags::Subtract);
        assert_eq!(register.F, 0b00000011);
        register.set_flag(Flags::HalfCarry);
        assert_eq!(register.F, 0b00000111);
        register.set_flag(Flags::Carry);
        assert_eq!(register.F, 0b00001111);

        register.set_flag(Flags::Carry);
        assert_eq!(register.F, 0b00001111);
    }

    #[test]
    fn should_reset_flags() {
        let mut register = Registers::default();
        register.F = 0b00001111;

        register.reset_flag(Flags::Zero);
        assert_eq!(register.F, 0b00001110);
        register.reset_flag(Flags::Subtract);
        assert_eq!(register.F, 0b00001100);
        register.reset_flag(Flags::HalfCarry);
        assert_eq!(register.F, 0b00001000);
        register.reset_flag(Flags::Carry);
        assert_eq!(register.F, 0b00000000);

        register.reset_flag(Flags::Carry);
        assert_eq!(register.F, 0b00000000);
    }

    #[test]
    fn should_read_flags() {
        let mut register = Registers::default();

        register.F = 0b00001111;
        assert_eq!(true, register.read_flag(Flags::Zero));
        assert_eq!  (true, register.read_flag(Flags::Subtract));
        assert_eq!(true, register.read_flag(Flags::HalfCarry));
        assert_eq!(true, register.read_flag(Flags::Carry));

        register.F = 0b00000000;
        assert_eq!(false, register.read_flag(Flags::Zero));
        assert_eq!(false, register.read_flag(Flags::Subtract));
        assert_eq!(false, register.read_flag(Flags::HalfCarry));
        assert_eq!(false, register.read_flag(Flags::Carry));
    }

    #[test]
    fn should_read_write_double_registers() {
        let mut register = Registers::default();

        register.write_HL(0b1010101011010011);
        assert_eq!(register.H, 0b10101010);
        assert_eq!(register.L, 0b11010011);
        assert_eq!(register.read_HL(), 0b1010101011010011);

        register.write_AF(0b1010101011010011);
        assert_eq!(register.A, 0b10101010);
        assert_eq!(register.F, 0b11010011);
        assert_eq!(register.read_AF(), 0b1010101011010011);

        register.write_BC(0b1010101011010011);
        assert_eq!(register.B, 0b10101010);
        assert_eq!(register.C, 0b11010011);
        assert_eq!(register.read_BC(), 0b1010101011010011);

        register.write_DE(0b1010101011010011);
        assert_eq!(register.D, 0b10101010);
        assert_eq!(register.E, 0b11010011);
        assert_eq!(register.read_DE(), 0b1010101011010011);
    }
}