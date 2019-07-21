
#[inline(always)]
pub fn as_u16(x: u8, y: u8) -> u16 {
    (x as u16) << 8 | (y as u16)
}

#[inline(always)]
pub fn read_bit(x: u8, nth: u8) -> bool {
    x & (1 << nth) != 0
}

// TODO review nth position is correct or reverse

#[inline(always)]
pub fn set_bit(mut x: u8, nth: u8, val: bool) -> u8 {
    if val {
        x |= (1 << nth);
    } else {
        x &= (1 << nth); // TODO review if correct
    }
    x
}