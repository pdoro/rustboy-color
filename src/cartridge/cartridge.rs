use std::{
    boxed::Box,
    convert::From,
    vec::Vec,
    ops
};
use log::{debug, error, info};
use super::{
    rom::RomOnly,
    mbc1::Mbc1Cartridge,
    mbc2::Mbc2Cartridge,
    mbc3::Mbc3Cartridge,
    mbc5::Mbc5Cartridge
};
use ops::Range;
use crate::utils::as_u16;
use crate::soc::instruction::Instruction;
use crate::memory::Address;

const KB: usize = 1024;
const MB: usize = KB * 1024;
const CARTRIDGE_TYPE_LOCATION: usize = 0x0147;

pub trait Cartridge :
    ops::Index<Address, Output = u8> +
    ops::Index<Range<Address>, Output = [u8]> +
    ops::IndexMut<Address, Output = u8>
{
    fn report(&self) {
        info!("[---------- Cartridge Metadata ----------]");
        info!("Title...........................{}", self.title());
        info!("Cartridge type..................{}", self.cartridge_type());
        info!("CGB flag........................{}", self.cgb_flag());
        info!("SGB flag........................{}", self.sgb_flag());
        info!("ROM banks.......................{}", self.rom_banks());
        info!("RAM size........................{}", self.ram_size());
        info!("Manufacturer code...............{}", self.manufacturer_code());
        info!("Destination code................{}", self.destination_code());
        info!("Old license code................{:#X?}", self.old_license_code());
        info!("New license code................{}", self.new_license_code());
        info!("Version number..................{}", self.version_number());
        info!("Checksum........................{}", self.checksum());
        info!("Global checksum.................{}", self.global_checksum());
    }

    // ---------------- Metadata ---------------- //

    fn title(&self) -> String {
        String::from_utf8_lossy(&self[0x0134..0x0143])
            .into_owned()
    }

    fn manufacturer_code(&self) -> String {
        String::from_utf8_lossy(&self[0x013F..0x0142])
            .into_owned()
    }

    fn cgb_flag(&self) -> bool {
        match self[0x0143] {
            0x80 => false,
            0xC0 => true,
            _ => false
        }
    }

    // fn new_license_code(&self) -> String {
    //     String::from_utf8_lossy(&self[0x0144..0x0145])
    //         .into_owned()
    // }

    fn sgb_flag(&self) -> bool {
        match self[0x0146] {
            0x00 => false,
            0x03 => true,
            _ => false
        }
    }

    fn cartridge_type(&self) -> &'static str {
        match self[0x0147] {
           0x00 => "ROM ONLY",
           0x19 => "MBC5",
           0x01 => "MBC1",
           0x1A => "MBC5+RAM",
           0x02 => "MBC1+RAM",
           0x1B => "MBC5+RAM+BATTERY",
           0x03 => "MBC1+RAM+BATTERY",
           0x1C => "MBC5+RUMBLE",
           0x05 => "MBC2",
           0x1D => "MBC5+RUMBLE+RAM",
           0x06 => "MBC2+BATTERY",
           0x1E => "MBC5+RUMBLE+RAM+BATTERY",
           0x08 => "ROM+RAM",
           0x20 => "MBC6",
           0x09 => "ROM+RAM+BATTERY",
           0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
           0x0B => "MMM01",
           0x0C => "MMM01+RAM",
           0x0D => "MMM01+RAM+BATTERY",
           0x0F => "MBC3+TIMER+BATTERY",
           0x10 => "MBC3+TIMER+RAM+BATTERY",
           0xFC => "POCKET CAMERA",
           0x11 => "MBC3",
           0xFD => "BANDAI TAMA5",
           0x12 => "MBC3+RAM",
           0xFE => "HuC3",
           0x13 => "MBC3+RAM+BATTERY",
           0xFF => "HuC1+RAM+BATTERY",
              _ => "Unknown"
        }
    }

    fn rom_banks(&self) -> u16 {
        match self[0x0148] {
            0x00 => 0,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => 0
        }
    }

    fn ram_size(&self) -> u32 {
        match self[0x0149] {
            0x00 => 0,
            0x01 => 2048,
            0x02 => 8192,
            0x03 => 32_768,
            0x04 => 131_072,
            0x05 => 65_536,
            _ => 0
        }
    }

    fn destination_code(&self) -> &'static str {
        match self[0x014A] {
            0x00 => "Japanese",
            0x01 => "Non-Japanese",
            _ => "Unknown"
        }
    }

    fn new_license_code(&self) -> &'static str {
        match (self[0x0144], self[0x0145]) {
            (0x0, 0x0) => "none",
            (0x0, 0x1) => "Nintendo R&D1",
            (0x0, 0x8) => "Capcom",
            (0x1, 0x3) => "Electronic Arts",
            (0x1, 0x8) => "Hudson Soft",
            (0x1, 0x9) => "b-ai",
            (0x2, 0x0) => "kss",
            (0x2, 0x2) => "pow",
            (0x2, 0x4) => "PCM Complete",
            (0x2, 0x5) => "san-x",
            (0x2, 0x8) => "Kemco Japan",
            (0x2, 0x9) => "seta",
            (0x3, 0x0) => "Viacom",
            (0x3, 0x1) => "Nintendo",
            (0x3, 0x2) => "Bandai",
            (0x3, 0x3) => "Ocean/Acclaim",
            (0x3, 0x4) => "Konami",
            (0x3, 0x5) => "Hector",
            (0x3, 0x7) => "Taito",
            (0x3, 0x8) => "Hudson",
            (0x3, 0x9) => "Banpresto",
            (0x4, 0x1) => "UbiSoft",
            (0x4, 0x2) => "Atlus",
            (0x4, 0x4) => "Malibu",
            (0x4, 0x6) => "angel",
            (0x4, 0x7) => "Bullet-Proof",
            (0x4, 0x9) => "irem",
            (0x5, 0x0) => "Absolute",
            (0x5, 0x1) => "Acclaim",
            (0x5, 0x2) => "Activision",
            (0x5, 0x3) => "American sammy",
            (0x5, 0x4) => "Konami",
            (0x5, 0x5) => "Hi tech entertainment",
            (0x5, 0x6) => "LJN",
            (0x5, 0x7) => "Matchbox",
            (0x5, 0x8) => "Mattel",
            (0x5, 0x9) => "Milton Bradley",
            (0x6, 0x0) => "Titus",
            (0x6, 0x1) => "Virgin",
            (0x6, 0x4) => "LucasArts",
            (0x6, 0x7) => "Ocean",
            (0x6, 0x9) => "Electronic Arts",
            (0x7, 0x0) => "Infogrames",
            (0x7, 0x1) => "Interplay",
            (0x7, 0x2) => "Broderbund",
            (0x7, 0x3) => "sculptured",
            (0x7, 0x5) => "sci",
            (0x7, 0x8) => "THQ",
            (0x7, 0x9) => "Accolade",
            (0x8, 0x0) => "misawa",
            (0x8, 0x3) => "lozc",
            (0x8, 0x6) => "tokuma shoten i*",
            (0x8, 0x7) => "tsukuda ori*",
            (0x9, 0x1) => "Chunsoft",
            (0x9, 0x2) => "Video system",
            (0x9, 0x3) => "Ocean/Acclaim",
            (0x9, 0x5) => "Varie",
            (0x9, 0x6) => "Yonezawa/s'pal",
            (0x9, 0x7) => "Kaneko",
            (0x9, 0x9) => "Pack in soft",
            (0xA, 0x4) => "Konami (Yu-Gi-Oh!)",
            _ => "Unknown"
        }
    }

    fn old_license_code(&self) -> u8 {
        self[0x014B]
    }

    fn version_number(&self) -> u8 {
        self[0x014C]
    }

    fn checksum(&self) -> u8 {
        self[0x014D]
    }

    fn global_checksum(&self) -> u16 {
        as_u16(self[0x014E], self[0x014F])
    }
}

pub fn decode_cartridge(blob: Vec<u8>) -> Box<dyn Cartridge> {

    info!("Decoding cartridge");

    let cartridge_type = blob
        .get(CARTRIDGE_TYPE_LOCATION as usize)
        .expect("Error accessing address to find cartridge type");

    match cartridge_type {
        0x00 | 0x08 | 0x09 => Box::new(RomOnly::new(blob)),
        0x01 | 0x02 | 0x03 => Box::new(Mbc1Cartridge::new(blob)),
        5 | 6 => Box::new(Mbc2Cartridge::new(blob)),
        0x0F..=0x13 => Box::new(Mbc3Cartridge::new(blob)),
        0x19..=0x1E => Box::new(Mbc5Cartridge::new(blob)),
        _ => panic!("Unsupported cartridge type: {}", cartridge_type),
    }
}