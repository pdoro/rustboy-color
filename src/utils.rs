
impl Into<u16> for (u8, u8) {
    #[inline(always)]
    fn into(self) -> u16 {
        (self.0 as u16) << 8 | (self.1 as u16)
    }
}

impl Into<(u8,u8)> for u16 {
    #[inline(always)]
    fn into(self) -> (u8, u8) {
        ((self >> 8) as u8, self as u8)
    }
}

// TODO review nth position is correct or reverse
