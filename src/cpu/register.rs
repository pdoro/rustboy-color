
use crate::utils::as_u16;
use std::fmt;

pub struct Register {
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

impl Register {
    // Special registers
    pub fn AF(&self) -> u16 {
        as_u16(self.A, self.F)
    }

    pub fn HL(&self) -> u16 {
        as_u16(self.H, self.L)
    }

    pub fn BC(&self) -> u16 {
        as_u16(self.B, self.C)
    }

    pub fn DE(&self) -> u16 {
        as_u16(self.D, self.E)
    }

    pub fn new() -> Register {
        Register {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            H: 0,
            L: 0,

            SP: 0,
            PC: 0xFFFE,
        }
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Register(A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, F: {:#X}, H: {:#X}, L: {:#X}, SP: {:#X}, PC: {:#X})",
               self.A, self.B, self.C, self.D, self.E, self.F, self.H, self.L, self.SP, self.PC)
    }
}