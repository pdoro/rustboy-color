
enum BankMode {
    Mode_16_8,
    Mode_4_32,
}

struct Mbc1Cartridge {
    data: Vec<u8>
}

impl Cartridge for Mbc1Cartridge {

}