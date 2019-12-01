
use crate::utils::{as_u16, set_bit};
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

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry
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
            PC: 0x64,
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

    pub fn read_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Zero => self.F & (1 << 1) != 0,
            Flag::Subtract => self.F & (1 << 2) != 0,
            Flag::HalfCarry => self.F & (1 << 3) != 0,
            Flag::Carry => self.F & (1 << 4) != 0,
        }
    }

    pub fn write_flag(&mut self, flag: Flag, status: bool) {
        match flag {
            Flag::Zero => self.F = set_bit(self.F, 1, status),
            Flag::Subtract => self.F = set_bit(self.F, 2, status),
            Flag::HalfCarry => self.F = set_bit(self.F, 3, status),
            Flag::Carry => self.F = set_bit(self.F, 4, status),
        }
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