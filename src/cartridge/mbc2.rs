use core::ops;
use crate::cartridge::cartridge::Cartridge;
use std::ops::Range;
use crate::memory::Address;

pub struct Mbc2Cartridge {
    data: Vec<u8>,
}

impl Mbc2Cartridge {
    pub fn new(data: Vec<u8>) -> Mbc2Cartridge {
        Mbc2Cartridge { data }
    }
}

impl Cartridge for Mbc2Cartridge {}

impl ops::Index<u16> for Mbc2Cartridge {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl ops::Index<Range<u16>> for Mbc2Cartridge {
    type Output = [u8];

    fn index(&self, index: Range<u16>) -> &Self::Output {
        let idx = index.start as usize .. index.end as usize;
        &self.data[idx]
    }
}

impl ops::IndexMut<u16> for Mbc2Cartridge {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}