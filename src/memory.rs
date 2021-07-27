use log::{debug, info, trace};
use std::ops::{Range, RangeInclusive};
use std::{fmt, ops};
use crate::cartridge::cartridge::Cartridge;

pub(crate) type Address = u16;
type Byte = u8;
type MemoryArea = RangeInclusive<Address>;

pub struct MemorySpace {
    // 8KB Working RAM
    // 8KB Video RAM
    work_ram: [u8; 8192],
    graphic_ram: [u8; 8192],

    // create VideoController struct with oam ram in CPU
    object_attribute_memory: [u8; 159],

    cartridge: Box<dyn Cartridge>,
}

impl MemorySpace {
    pub fn new(cartridge: Box<dyn Cartridge>) -> MemorySpace {
        MemorySpace {
            work_ram: [0; 8192],
            graphic_ram: [0; 8192],
            object_attribute_memory: [0; 159],
            cartridge
        }
    }

    pub fn cartridge_is_mapped(&self) -> bool {
        self[0xFF50] == 1
    }
}

const MEMORY_START: Address = 0x0000;
const MEMORY_END: Address = 0xFFFF;

// http://gameboy.mongenel.com/dmg/asmmemmap.html
// TODO try to match on this const when rust allows
// address if InterruptEnabledFlag_RANGE.contains(&address) =>
const InterruptEnabledFlag_RANGE: MemoryArea  = 0xFFFF..=0xFFFF;
const HighRam_RANGE: MemoryArea               = 0xFF80..=0xFFFE;
const IORegisters_RANGE: MemoryArea           = 0xFF00..=0xFF7F;
const Unmapped_RANGE: MemoryArea              = 0xFEA0..=0xFEFF;
const ObjectAttributeMemory_RANGE: MemoryArea = 0xFE00..=0xFE9F;
const EchoRam_RANGE: MemoryArea               = 0xE000..=0xFDFF;
const WorkingRam_RANGE: MemoryArea            = 0xC000..=0xDFFF;
const CartridgeRam_RANGE: MemoryArea          = 0xA000..=0xBFFF;
const BackgroundMap_RANGE: MemoryArea         = 0x9800..=0x9FFF;
const TileRam_RANGE: MemoryArea               = 0x8000..=0x97FF;
const CartridgeRom_RANGE: MemoryArea          = 0x0100..=0x07FF;
const InterruptVector_RANGE: MemoryArea       = 0x0000..=0x00FF;

impl ops::Index<Address> for MemorySpace {
    type Output = Byte;

    fn index(&self, address: Address) -> &Self::Output {
        if address < MEMORY_START || address > MEMORY_END {
            panic!("Invalid unsafe memory access to {:#X}", address);
        } else {
            trace!("Reading memory address {:#X}", address);

            let data = match address {
                // Interrupt Register
                0xFFFF..=0xFFFF => {
                    panic!("This address belongs to Interrupt Register. It should be dispatched by the CPU!")
                },
                // High Ram
                0xFF80..=0xFFFE => {
                    panic!("This address belongs to HighRam space. It should be dispatched by the CPU!")
                },
                // IO Ports
                0xFF00..=0xFF7F => {
                    unimplemented!()
                }
                // Unmapped memory
                0xFEA0..=0xFEFF => {
                    panic!("Invalid access to unmapped memory")
                },
                // OAM memory
                0xFE00..=0xFE9F => {
                    &self.object_attribute_memory[(address - 0xFE00) as usize]
                },
                // Echo RAM
                0xE000..=0xFDFF => {
                    // 0xE000 == 0xC000
                    &self.work_ram[(address - 0xE000) as usize]
                },
                // Work Ram
                0xC000..=0xDFFF => {
                    &self.work_ram[(address - 0xC000) as usize]
                },
                // External RAM (Cartridge)
                0xA000..=0xBFFF => {
                    // TODO check if cartridge has external RAM available?
                    &self.cartridge[address]
                },
                // Graphics RAM
                0x8000..=0x9FFF => {
                    // Remember, space is only 16KB although the whole memory map is 64KB
                    &self.graphic_ram[(address - 0x8000) as usize]
                },
                // Cartridge
                0x0000..=0x7FFF => {
                    if self.cartridge_is_mapped() {
                        &self.cartridge[address]
                    } else {
                        &BOOT_ROM[address as usize]
                    }
                },
                _ => panic!("Address {:#X} does not belong to memory space, cannot map to area", address),
            };

            data
        }
    }
}

impl ops::IndexMut<Address> for MemorySpace {
    fn index_mut(&mut self, address: Address) -> &mut Self::Output {
        if address < MEMORY_START || address > MEMORY_END {
            panic!("Invalid unsafe memory write to {:#X}", address);
        } else {
            trace!("Writing memory address {:#X}", address);
            &mut self[address]
        }
    }
}

impl fmt::Debug for MemorySpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory({:?} bytes)", self.work_ram.len() + self.graphic_ram.len())
    }
}

// https://realboyemulator.wordpress.com/2013/01/03/a-look-at-the-game-boy-bootstrap-let-the-fun-begin/
const BOOT_ROM: [Byte; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];
