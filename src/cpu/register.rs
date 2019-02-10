
use crate::utils::as_u16;

#[derive(Debug)]
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
    fn AF(&self) -> u16 {
        as_u16(self.A, self.F)
    }

    fn HL(&self) -> u16 {
        as_u16(self.H, self.L)
    }

    fn BC(&self) -> u16 {
        as_u16(self.B, self.C)
    }

    fn DE(&self) -> u16 {
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