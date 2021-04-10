use crate::cartridge::cartridge::Cartridge;
use core::ops;
use std::ops::Range;

pub struct Mbc5Cartridge {
    data: Vec<u8>,
}

impl Mbc5Cartridge {
    pub fn new(blob: Vec<u8>) -> Mbc5Cartridge {
        Mbc5Cartridge {
            data : blob
        }
    }
}

impl Cartridge for Mbc5Cartridge {
    fn read(&self, address: u8) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, address: u8, data: u8) {
        unimplemented!()
    }
}

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