
use crate::utils::as_u16;
use std::{fmt, u16, u8};

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

    // Stack Pointer
    pub SP: u16,
    // Program Counter
    pub PC: u16,

    // Interrupt Register
    pub IR: u8,
}

pub enum Flags {
    Zero = 7,
    Subtract = 6,
    HalfCarry = 5,
    Carry = 4,
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

            IR: 0,
        }
    }
}

#[allow(non_snake_case)]
impl Registers {
    // Special registers
    pub fn read_AF(&self) -> u16 {
        (self.A, self.F).into()
        //as_u16(self.A, self.F)
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

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Register(A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, F: {:#X}, H: {:#X}, L: {:#X}, SP: {:#X}, PC: {:#X})",
               self.A, self.B, self.C, self.D, self.E, self.F, self.H, self.L, self.SP, self.PC)
    }
}

pub trait MathOps<T> {
    fn carrying_add(&mut self, x: T, y: T) -> T;
    fn borrowing_sub(&mut self, x: T, y: T) -> T;
}

impl MathOps<u8> for Registers {
    // https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
    fn carrying_add(&mut self, x: u8, y: u8) -> u8 {
        let (sum, carry) = x.overflowing_add(y);
        let half_carry = (((x & 0xF) + (y & 0xF)) & 0x10) == 0x10;

        if carry {
            self.set_flag(Flags::Carry);
        }
        if half_carry {
            self.set_flag(Flags::HalfCarry);
        }
        if sum == 0 {
            self.set_flag(Flags::Zero)
        }
        self.reset_flag(Flags::Subtract);

        sum
    }

    fn borrowing_sub(&mut self, x: u8, y: u8) -> u8 {
        let (sub, borrow) = x.overflowing_sub(y);
        let half_borrow = (x & 0xF) < (y & 0xF); // https://www.reddit.com/r/EmuDev/comments/4clh23/trouble_with_halfcarrycarry_flag

        if !borrow {
            self.set_flag(Flags::Carry)
        }
        if !half_borrow {
            self.set_flag(Flags::HalfCarry)
        }
        if sub == 0 {
            self.set_flag(Flags::Zero)
        }
        self.set_flag(Flags::Subtract);

        sub
    }
}

impl MathOps<u16> for Registers {
    // https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
    fn carrying_add(&mut self, x: u16, y: u16) -> u16 {
        let (sum, carry) = x.overflowing_add(y);
        let half_carry = (((x & 0xFFF) + (y & 0xFFF)) & 0x1000) == 0x1000;

        if carry {
            self.set_flag(Flags::Carry);
        }
        if half_carry {
            self.set_flag(Flags::HalfCarry);
        }
        if sum == 0 {
            self.set_flag(Flags::Zero)
        }
        self.reset_flag(Flags::Subtract);

        sum
    }

    fn borrowing_sub(&mut self, x: u16, y: u16) -> u16 {
        let (sub, borrow) = x.overflowing_sub(y);
        let half_borrow = (x & 0xFFF) < (y & 0xFFF); // https://www.reddit.com/r/EmuDev/comments/4clh23/trouble_with_halfcarrycarry_flag/

        if !borrow {
            self.set_flag(Flags::Carry)
        }
        if !half_borrow {
            self.set_flag(Flags::HalfCarry)
        }
        if sub == 0 {
            self.set_flag(Flags::Zero)
        }
        self.set_flag(Flags::Subtract);

        sub
    }
}

#[cfg(test)]
mod register_tests {
    use super::*;

    #[test]
    fn should_set_flags() {
        let mut register = Registers::default();

        register.set_flag(Flags::Zero);
        assert_eq!(register.F, 0b10000000);
        register.set_flag(Flags::Subtract);
        assert_eq!(register.F, 0b11000000);
        register.set_flag(Flags::HalfCarry);
        assert_eq!(register.F, 0b11100000);
        register.set_flag(Flags::Carry);
        assert_eq!(register.F, 0b11110000);

        register.set_flag(Flags::Carry);
        assert_eq!(register.F, 0b11110000);
    }

    #[test]
    fn should_reset_flags() {
        let mut register = Registers::default();
        register.F = 0b11110000;

        register.reset_flag(Flags::Zero);
        assert_eq!(register.F, 0b01110000);
        register.reset_flag(Flags::Subtract);
        assert_eq!(register.F, 0b00110000);
        register.reset_flag(Flags::HalfCarry);
        assert_eq!(register.F, 0b00010000);
        register.reset_flag(Flags::Carry);
        assert_eq!(register.F, 0b00000000);

        register.reset_flag(Flags::Carry);
        assert_eq!(register.F, 0b00000000);
    }

    #[test]
    fn should_read_flags() {
        let mut register = Registers::default();

        register.F = 0b11110000;
        assert_eq!(true, register.read_flag(Flags::Zero));
        assert_eq!(true, register.read_flag(Flags::Subtract));
        assert_eq!(true, register.read_flag(Flags::HalfCarry));
        assert_eq!(true, register.read_flag(Flags::Carry));

        register.F = 0b00000000;
        assert_eq!(false, register.read_flag(Flags::Zero));
        assert_eq!(false, register.read_flag(Flags::Subtract));
        assert_eq!(false, register.read_flag(Flags::HalfCarry));
        assert_eq!(false, register.read_flag(Flags::Carry));
    }

    #[test]
    fn should_read_double_registers() {
        let mut register = Registers::default();

        register.H = 0b10101010;
        register.L = 0b11010011;
        assert_eq!(register.read_HL(), 0b1010101011010011);

        register.A = 0b10101010;
        register.F = 0b11010011;
        assert_eq!(register.read_AF(), 0b1010101011010011);

        register.B = 0b10101010;
        register.C = 0b11010011;
        assert_eq!(register.read_BC(), 0b1010101011010011);

        register.D = 0b10101010;
        register.E = 0b11010011;
        assert_eq!(register.read_DE(), 0b1010101011010011);
    }

    #[test]
    fn should_write_double_registers() {
        let mut register = Registers::default();

        register.write_HL(0b1010101011010011);
        assert_eq!(register.H, 0b10101010);
        assert_eq!(register.L, 0b11010011);

        register.write_AF(0b1010101011010011);
        assert_eq!(register.A, 0b10101010);
        assert_eq!(register.F, 0b11010011);

        register.write_BC(0b1010101011010011);
        assert_eq!(register.B, 0b10101010);
        assert_eq!(register.C, 0b11010011);

        register.write_DE(0b1010101011010011);
        assert_eq!(register.D, 0b10101010);
        assert_eq!(register.E, 0b11010011);
    }

    #[test]
    fn should_add_with_carry_and_halfcarry_for_u8() {
        let mut register = Registers::default();

        let x: u8 = 0b00001111;
        let y: u8 = 0b00000001;

        let sum = register.carrying_add(x, y);

        assert_eq!(sum, 0b00010000);
        assert_eq!(register.read_flag(Flags::Carry), false);
        assert_eq!(register.read_flag(Flags::HalfCarry), true);
        assert_eq!(register.read_flag(Flags::Zero), false);
        assert_eq!(register.read_flag(Flags::Subtract), false);

        let mut register = Registers::default();

        let x: u8 = 0b11111111;
        let y: u8 = 0b00000001;

        let sum = register.carrying_add(x, y);

        assert_eq!(sum, 0b00000000);
        assert_eq!(register.read_flag(Flags::Carry), true);
        assert_eq!(register.read_flag(Flags::HalfCarry), true);
        assert_eq!(register.read_flag(Flags::Zero), true);
        assert_eq!(register.read_flag(Flags::Subtract), false);
    }

    #[test]
    fn should_add_with_carry_and_halfcarry_for_u16() {
        let mut register = Registers::default();

        let x: u16 = 0b0000111111111111;
        let y: u16 = 0b0000000000000001;

        let sum = register.carrying_add(x, y);

        assert_eq!(sum, 0b0001000000000000);
        assert_eq!(register.read_flag(Flags::Carry), false);
        assert_eq!(register.read_flag(Flags::HalfCarry), true);
        assert_eq!(register.read_flag(Flags::Zero), false);
        assert_eq!(register.read_flag(Flags::Subtract), false);

        let mut register = Registers::default();

        let x: u16 = 0b1111111111111111;
        let y: u16 = 0b0000000000000001;

        let sum = register.carrying_add(x, y);

        assert_eq!(sum, 0b0000000000000000);
        assert_eq!(register.read_flag(Flags::Carry), true);
        assert_eq!(register.read_flag(Flags::HalfCarry), true);
        assert_eq!(register.read_flag(Flags::Zero), true);
        assert_eq!(register.read_flag(Flags::Subtract), false);
    }

    #[test]
    fn should_sub_with_carry_and_halfcarry_for_u8() {
        let mut register = Registers::default();

        let x: u8 = 0b00010000;
        let y: u8 = 0b00000001;

        let sub = register.borrowing_sub(x, y);

        assert_eq!(sub, 0b00001111);
        assert_eq!(register.read_flag(Flags::Carry), true);
        assert_eq!(register.read_flag(Flags::HalfCarry), false);
        assert_eq!(register.read_flag(Flags::Zero), false);
        assert_eq!(register.read_flag(Flags::Subtract), true);

        let mut register = Registers::default();

        let x: u8 = 0b00000000;
        let y: u8 = 0b00000001;

        let sub = register.borrowing_sub(x, y);

        assert_eq!(sub, 0b11111111);
        assert_eq!(register.read_flag(Flags::Carry), false);
        assert_eq!(register.read_flag(Flags::HalfCarry), false);
        assert_eq!(register.read_flag(Flags::Zero), false);
        assert_eq!(register.read_flag(Flags::Subtract), true);
    }

    #[test]
    fn should_sub_with_carry_and_halfcarry_for_u16() {
        let mut register = Registers::default();

        let x: u16 = 0b0001000000000000;
        let y: u16 = 0b0000000000000001;

        let sub = register.borrowing_sub(x, y);

        assert_eq!(sub, 0b0000111111111111);
        assert_eq!(register.read_flag(Flags::Carry), true);
        assert_eq!(register.read_flag(Flags::HalfCarry), false);
        assert_eq!(register.read_flag(Flags::Zero), false);
        assert_eq!(register.read_flag(Flags::Subtract), true);

        let mut register = Registers::default();

        let x: u16 = 0b0000000000000000;
        let y: u16 = 0b0000000000000001;

        let sub = register.borrowing_sub(x, y);

        assert_eq!(sub, 0b1111111111111111);
        assert_eq!(register.read_flag(Flags::Carry), false);
        assert_eq!(register.read_flag(Flags::HalfCarry), false);
        assert_eq!(register.read_flag(Flags::Zero), false);
        assert_eq!(register.read_flag(Flags::Subtract), true);
    }
}
