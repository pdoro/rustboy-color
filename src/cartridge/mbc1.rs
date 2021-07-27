use std::ops;
use std::ops::{Index, Range};
use crate::cartridge::cartridge::Cartridge;

enum BankMode {
    ROM, RAM
}

pub struct Mbc1Cartridge {
    data: Vec<u8>,
    current_rom_bank: u8,
    current_ram_bank: u8,
    ram_enabled: bool,
    bank_mode: BankMode
}

impl Mbc1Cartridge {
    pub fn new(blob: Vec<u8>) -> Mbc1Cartridge {
        Mbc1Cartridge {
            data : blob,
            current_rom_bank: 0,
            current_ram_bank: 0,
            ram_enabled: false,
            bank_mode: BankMode::ROM
        }
    }
}

impl Cartridge for Mbc1Cartridge {}

impl ops::Index<u16> for Mbc1Cartridge {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl ops::Index<Range<u16>> for Mbc1Cartridge {
    type Output = [u8];

    fn index(&self, index: Range<u16>) -> &Self::Output {
        let idx = index.start as usize .. index.end as usize;
        &self.data[idx]
    }
}

impl ops::IndexMut<u16> for Mbc1Cartridge {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}