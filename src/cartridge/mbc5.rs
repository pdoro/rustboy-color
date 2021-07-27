use crate::cartridge::cartridge::Cartridge;
use core::ops;
use std::ops::Range;
use crate::memory::Address;

pub struct Mbc5Cartridge {
    data: Vec<u8>,
}

impl Mbc5Cartridge {
    pub fn new(data: Vec<u8>) -> Mbc5Cartridge {
        Mbc5Cartridge { data }
    }
}

impl Cartridge for Mbc5Cartridge {}

impl ops::Index<u16> for Mbc5Cartridge {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl ops::Index<Range<u16>> for Mbc5Cartridge {
    type Output = [u8];

    fn index(&self, index: Range<u16>) -> &Self::Output {
        let idx = index.start as usize .. index.end as usize;
        &self.data[idx]
    }
}

impl ops::IndexMut<u16> for Mbc5Cartridge {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}