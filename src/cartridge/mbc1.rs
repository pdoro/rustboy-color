use std::ops;
use std::ops::{Index, Range};
use crate::cartridge::cartridge::Cartridge;

enum BankingMode {
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
            bank_mode: BankingMode::ROM
        }
    }
}

impl Cartridge for Mbc1Cartridge {
    fn read(&self, address: u16) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, address: u16, data: u8) {

        match address {
            // RAM Enable (Write Only)
            0x0000..0x1FFF => {
                match data {
                    0x0A => self.ram_enabled = true,
                    _ => self.ram_enabled = false
                }
            }
            // ROM Bank Number (Write Only)
            0x2000..0x3FFF => {
                match data {
                    0x00 | 0x20 | 0x40 | 0x60 => self.current_rom_bank = data + 1,
                    // 5 lower bits select rom bank
                    _ => self.current_rom_bank = 0b00011111 & data
                }
            }
            // RAM Bank Number or Upper Bits of ROM Bank Number (Write Only)
            0x4000..0x5FFF => {
                let selector = data & 0b11000000;
                match self.bank_mode {
                    ROM => {},
                    RAM => {},
                }
            }
            // ROM/RAM Mode Select (Write Only)
            0x6000..0x7FFF => {
                match data {
                    0x00 => self.bank_mode = BankingMode::ROM,
                    0x01 => self.bank_mode = BankingMode::RAM,
                    _ => panic!("Invalid banking mode")
                }
            }
            _ => panic!("TODO")
        }

    }
}

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