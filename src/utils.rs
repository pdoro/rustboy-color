
#[inline(always)]
pub fn as_u16(x: u8, y: u8) -> u16 {
    (x as u16) << 8 | (y as u16)
}