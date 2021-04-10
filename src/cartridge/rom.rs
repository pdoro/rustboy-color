use crate::cartridge::cartridge::Cartridge;
use core::ops;
use std::ops::{Index, Range};

pub struct RomOnly {
    data: Vec<u8>,
}

impl RomOnly {
    pub fn new(blob: Vec<u8>) -> RomOnly {
        RomOnly {
            data : blob
        }
    }
}

impl Cartridge for RomOnly {
    fn read(&self, address: u16) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, address: u16, data: u8) {
        unimplemented!()
    }
}

impl ops::Index<u16> for RomOnly {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl ops::Index<Range<u16>> for RomOnly {
    type Output = [u8];

    fn index(&self, index: Range<u16>) -> &Self::Output {
        let idx = index.start as usize .. index.end as usize;
        &self.data[idx]
    }
}

impl ops::IndexMut<u16> for RomOnly {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

// impl ops::IndexMut<Range<u16>> for RomOnly {
//
//     fn index_mut(&mut self, index: Range<u16>) -> &mut Self::Output {
//         unimplemented!()
//     }
// }