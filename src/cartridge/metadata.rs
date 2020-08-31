
use log::{debug, info, trace};

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

const LICENSE_CODES: HashMap<u8, &str> = vec![
    (00, "none"),
    (01, "Nintendo R&D1"),
    (08, "Capcom"),
    (13, "Electronic Arts"),
    (18, "Hudson Soft"),
    (19, "b-ai"),
    (20, "kss"),
    (22, "pow"),
    (24, "PCM Complete"),
    (25, "san-x"),
    (28, "Kemco Japan"),
    (29, "seta"),
    (30, "Viacom"),
    (31, "Nintendo"),
    (32, "Bandai"),
    (33, "Ocean/Acclaim"),
    (34, "Konami"),
    (35, "Hector"),
    (37, "Taito"),
    (38, "Hudson"),
    (39, "Banpresto"),
    (41, "UbiSoft"),
    (42, "Atlus"),
    (44, "Malibu"),
    (46, "angel"),
    (47, "Bullet-Proof"),
    (49, "irem"),
    (50, "Absolute"),
    (51, "Acclaim"),
    (52, "Activision"),
    (53, "American sammy"),
    (54, "Konami"),
    (55, "Hi tech entertainment"),
    (56, "LJN"),
    (57, "Matchbox"),
    (58, "Mattel"),
    (59, "Milton Bradley"),
    (60, "Titus"),
    (61, "Virgin"),
    (64, "LucasArts"),
    (67, "Ocean"),
    (69, "Electronic Arts"),
    (70, "Infogrames"),
    (71, "Interplay"),
    (72, "Broderbund"),
    (73, "sculptured"),
    (75, "sci"),
    (78, "THQ"),
    (79, "Accolade"),
    (80, "misawa"),
    (83, "lozc"),
    (86, "tokuma shoten i*"),
    (87, "tsukuda ori*"),
    (91, "Chunsoft"),
    (92, "Video system"),
    (93, "Ocean/Acclaim"),
    (95, "Varie"),
    (96, "Yonezawa/s'pal"),
    (97, "Kaneko"),
    (99, "Pack in soft")
    //(A4, "Konami (Yu-Gi-Oh!)"),
].into_iter().collect();

pub struct HeaderMetadata {
    pub title: str,
    pub manufacturer_code: str,
    pub cgb_flag: bool,
    pub new_license_code: str,
    pub sgb_flag: bool,
    pub cartridge_type: CartridgeType,
    pub rom_banks: u8,
    pub ram_size: u16,
    pub destination_code: str,
    pub old_license_code: str,
    pub version_number: str,
    pub checksum: u8,
    pub global_checksum: u16
}

impl HeaderMetadata {

    pub fn new(header: &[u8]) -> HeaderMetadata {
        HeaderMetadata {
            title: title(header[0x0134..0x0143]),
            manufacturer_code: manufacturer_code(header[0x013F..0x0142]),
            cgb_flag: cgb_flag(header[0x0143]),
            new_license_code: new_license_code(header[0x0144..0x0145]),
            sgb_flag: sgb_flag(header[0x0146]),
            cartridge_type: cartridge_type(header[0x0147]),
            rom_banks: rom_banks(header[0x0148]),
            ram_size: ram_size(header[0x0149]),
            destination_code: destination_code(header[0x014A]),
            old_license_code: old_license_code(header[0x14B]),
            version_number: version_number(header[0x014C]),
            checksum: header[0x014D],
            global_checksum: header[0x014E..0x014F]
        }
    }
}

fn title(slice: &[u8]) -> &str {
    unimplemented!()
}

fn sgb_flag(flag: u8) -> bool {
    match flag {
        0x00 => false,
        0x03 => true,
        _ => {}
    }
}

fn cgb_flag(flag: u8) -> bool {
    match flag {
        0x80 => false, // TODO
        0xC0 => true,
        _ => {}
    }
}

fn rom_banks(data: u8) -> u8 {
    match data{
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
        _ => {}
    }
}

fn ram_size(data: u8) -> u16 {
    match data {
        0x00 => 0,
        0x01 => 2048,
        0x02 => 8192,
        0x03 => 32_768,
        0x04 => 131_072,
        0x05 => 65_536,
        _ => {}
    }
}

fn manufacturer_code(data: u8) -> &str {
    unimplemented!()
}

fn destination_code(data: u8) -> &str {
    match data {
        0x00 => "Japanese",
        0x01 => "Non-Japanese",
        _ => {}
    }
}

fn old_license_code(data: u8) -> &str {
    unimplemented!()
}

fn version_number(data: u8) -> &str {
    unimplemented!()
}