use crate::cartridge::cartridge::Cartridge;
use core::ops;
use std::ops::{Index, Range, RangeInclusive};
use crate::memory::Address;

const MEMORY_RANGE: RangeInclusive<Address> = 0x0000..=0x7FFF;

pub struct RomOnly {
    data: Vec<u8>,
}

impl RomOnly {
    pub fn new(data: Vec<u8>) -> RomOnly {
        RomOnly { data }
    }
}

impl Cartridge for RomOnly {}

impl ops::Index<Address> for RomOnly {
    type Output = u8;

    fn index(&self, index: Address) -> &Self::Output {
        if MEMORY_RANGE.contains(&index) {
            &self.data
                .get(index as usize)
                .expect("Error fetching data")
        } else {
            panic!("Illegal ROM read access at address {:#X}", index)
        }
    }
}

impl ops::Index<Range<Address>> for RomOnly {
    type Output = [u8];

    fn index(&self, index: Range<Address>) -> &Self::Output {
        if MEMORY_RANGE.contains(&index.start) && MEMORY_RANGE.contains(&index.end) {
            let range = index.start as usize .. index.end as usize;
            &self.data
                .get(range)
                .expect("Error fetching data")
        } else {
            panic!("Illegal ROM read access at address {:#?}", index)
        }
    }
}

impl ops::IndexMut<Address> for RomOnly {

    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        panic!("Illegal ROM write access at address {:#X}", index)
    }
}

impl ops::IndexMut<Range<u16>> for RomOnly {

    fn index_mut(&mut self, index: Range<u16>) -> &mut Self::Output {
        panic!("Illegal ROM write access at address {:#?}", index)
    }
}