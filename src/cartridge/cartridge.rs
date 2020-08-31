use std::boxed::Box;
use std::convert::From;
use std::vec::Vec;

const KB: usize = 1024;
const MB: usize = KB * 1024;
const CARTRIDGE_TYPE_LOCATION: usize = 0x147;

pub trait Cartridge : ops::Index<Address> + ops::IndexMut<Address> {

    fn report(&self) {
        let metadata = self.header_metadata();

        info!("[---------- Cartridge Header Metadata ----------]");
        info!("Title...........................{}", metadata.title);
        info!("Manufacturer code...............{}", metadata.manufacturer_code);
        info!("CGB flag........................{}", metadata.cgb_flag);
        info!("New license code................{}", metadata.new_license_code);
        info!("SGB flag........................{}", metadata.sgb_flag);
        info!("Cartridge type..................{}", metadata.cartridge_type);
        info!("ROM banks.......................{}", metadata.rom_banks);
        info!("RAM size........................{}", metadata.ram_size);
        info!("Destination code................{}", metadata.destination_code);
        info!("Old license code................{}", metadata.old_license_code);
        info!("Version number..................{}", metadata.version_number);
        info!("Checksum........................{}", metadata.checksum);
        info!("Global checksum.................{}", metadata.global_checksum);
    }
}

impl From<Vec<u8>> for Cartridge {

    fn from(cartridge_blob: Vec<u8>) -> Box<Cartridge> {

        let cartridge_type = cartridge_blob
            .get(CARTRIDGE_TYPE_LOCATION as usize)
            .expect("Unknown cartridge type");

        let cartridge_type = match cartridge_type {
            0 => RomOnly,
            1 | 2 | 3 => MBC1 {
                mode: BankMode::Mode_4_32,
            },
            5 | 6 => MBC2,
            12 | 13 => MBC3,
            _ => panic!("Unknown cartridge type: {}", cartridge_type),
        };

        Box::new()
    }
}