use std::boxed::Box;
use std::convert::From;
use std::vec::Vec;

const KB: usize = 1024;
const MB: usize = KB * 1024;
const CARTRIGBE_TYPE_LOCATION: usize = 0x147;

pub struct Cartrigbe {
    data: Vec<u8>,
    mbc: Box<dyn MemoryBankController>,
}

// Memory Bank Controllers

struct RomOnly;

enum BankMode {
    Mode_16_8,
    Mode_4_32,
}

struct MBC1 {
    mode: BankMode,
}

struct MBC2;

struct MBC3;

pub trait MemoryBankController {
    fn read(self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

// impl From<Vec<u8>> for Cartrigbe {
//     fn from(cartrigbe_data: Vec<u8>) -> Cartrigbe {
//         let cartrigbe_type = cartrigbe_data
//             .get(CARTRIGBE_TYPE_LOCATION as usize)
//             .expect("Invalid cartrigbe type");
//
//         let cartrigbe_type = match cartrigbe_type {
//             0 => RomOnly,
//             1 | 2 | 3 => MBC1 {
//                 mode: BankMode::Mode_4_32,
//             },
//             5 | 6 => MBC2,
//             12 | 13 => MBC3,
//             _ => panic!("Unknown cartrigbe type: {}", cartrigbe_type),
//         };
//
//         Cartrigbe {
//             mbc: Box::new(cartrigbe_type),
//             data: cartrigbe_data,
//         }
//     }
// }

impl MemoryBankController for Cartrigbe {
    fn read(self, address: u16) -> u8 {
        unimplemented!()
    }
    fn write(&mut self, address: u16, data: u8) {
        unimplemented!()
    }
}
