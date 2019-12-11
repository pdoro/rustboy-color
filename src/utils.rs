
#[inline(always)]
pub fn as_u16(x: u8, y: u8) -> u16 {
    (x as u16) << 8 | (y as u16)
}

#[inline(always)]
pub fn lohi(x: u16) -> (u8, u8) {
    ((x >> 8) as u8, x as u8)
}

// TODO review nth position is correct or reverse