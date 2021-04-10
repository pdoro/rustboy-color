
use log::{debug, info, trace};
use std::collections::HashMap;
use crate::cartridge::cartridge::Cartridge;
use core::ops;
use phf::phf_map;

pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    MBC5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    MBC6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
}

// TODO check someday if I can extract the metadata methods to its own
// trait and give a default impl for dyn Cartridge or even ops::Index
// pub trait Metadata {
//     fn cgb_flag(&self) -> bool;
// }
//
// impl Metadata for dyn Cartridge {
//
//     fn cgb_flag(&self) -> bool {
//         match self[0x0143] {
//             0x80 => false,
//             0xC0 => true,
//             _ => panic!()
//         }
//     }
// }

// impl dyn Cartridge<Output = u8> {
//
//     fn title(&self) -> &str {
//
//     }
//
//     fn manufacturer_code(&self) -> &str {
//
//     }
//
//     fn cgb_flag(&self) -> bool {
//         match self[0x0143] {
//             0x80 => false,
//             0xC0 => true,
//             _ => panic!()
//         }
//     }
//
//     fn new_license_code(&self) -> &str {
// 0x00 => "none",
// 0x01 => "Nintendo R&D1",
// 0x08 => "Capcom",
// 0x13 => "Electronic Arts",
// 0x18 => "Hudson Soft",
// 0x19 => "b-ai",
// 0x20 => "kss",
// 0x22 => "pow",
// 0x24 => "PCM Complete",
// 0x25 => "san-x",
// 0x28 => "Kemco Japan",
// 0x29 => "seta",
// 0x30 => "Viacom",
// 0x31 => "Nintendo",
// 0x32 => "Bandai",
// 0x33 => "Ocean/Acclaim",
// 0x34 => "Konami",
// 0x35 => "Hector",
// 0x37 => "Taito",
// 0x38 => "Hudson",
// 0x39 => "Banpresto",
// 0x41 => "UbiSoft",
// 0x42 => "Atlus",
// 0x44 => "Malibu",
// 0x46 => "angel",
// 0x47 => "Bullet-Proof",
// 0x49 => "irem",
// 0x50 => "Absolute",
// 0x51 => "Acclaim",
// 0x52 => "Activision",
// 0x53 => "American sammy",
// 0x54 => "Konami",
// 0x55 => "Hi tech entertainment",
// 0x56 => "LJN",
// 0x57 => "Matchbox",
// 0x58 => "Mattel",
// 0x59 => "Milton Bradley",
// 0x60 => "Titus",
// 0x61 => "Virgin",
// 0x64 => "LucasArts",
// 0x67 => "Ocean",
// 0x69 => "Electronic Arts",
// 0x70 => "Infogrames",
// 0x71 => "Interplay",
// 0x72 => "Broderbund",
// 0x73 => "sculptured",
// 0x75 => "sci",
// 0x78 => "THQ",
// 0x79 => "Accolade",
// 0x80 => "misawa",
// 0x83 => "lozc",
// 0x86 => "tokuma shoten i*",
// 0x87 => "tsukuda ori*",
// 0x91 => "Chunsoft",
// 0x92 => "Video system",
// 0x93 => "Ocean/Acclaim",
// 0x95 => "Varie",
// 0x96 => "Yonezawa/s'pal",
// 0x97 => "Kaneko",
// 0x99 => "Pack in soft",
// 0xA4, "Konami (Yu-Gi-Oh!)",
//
//     }
//
//     fn sgb_flag(&self) -> bool {
//         match self[0x0146] {
//             0x00 => false,
//             0x03 => true,
//             _ => panic!()
//         }
//     }
//
//     fn cartridge_type(&self) -> CartridgeType {
//
//     }
//
//     fn rom_banks(&self) -> u8 {
//         match self[0x0148] {
//             0x00 => 0,
//             0x01 => 4,
//             0x02 => 8,
//             0x03 => 16,
//             0x04 => 32,
//             0x05 => 64,
//             0x06 => 128,
//             0x07 => 256,
//             0x08 => 512,
//             0x52 => 72,
//             0x53 => 80,
//             0x54 => 96,
//             _ => panic!()
//         }
//     }
//
//     fn ram_size(&self) -> u16 {
//         match self[0x0149] {
//             0x00 => 0,
//             0x01 => 2048,
//             0x02 => 8192,
//             0x03 => 32_768,
//             0x04 => 131_072,
//             0x05 => 65_536,
//             _ => panic!()
//         }
//     }
//
//     fn destination_code(&self) -> &str {
//         match self[0x014A] {
//             0x00 => "Japanese",
//             0x01 => "Non-Japanese",
//             _ => {}
//         }
//     }
//
//     fn old_license_code(&self) -> &str {
//
//     }
//
//     fn version_number(&self) -> &str {
//
//     }
//
//     fn checksum(&self) -> u8 {
//         match self[0x014D] {
//
//         }
//     }
//
//     fn global_checksum(&self) -> u16 {
//
//     }
// }