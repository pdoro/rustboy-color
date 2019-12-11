pub use crate::cpu::cpu::CPU;
pub use crate::memory::MemorySpace;

const HALT: u8 = 0x76;

#[test]
fn should_load_immediate_value() {

    // LD A, Word
    let mut cpu = CPU::new(MemorySpace::new((&[0x06, 0x11, HALT])));
    cpu.run();
    assert_eq!(cpu.register.B, 0x11);

    let mut cpu = CPU::new(MemorySpace::new((&[0x0E, 0x11, HALT])));
    cpu.run();
    assert_eq!(cpu.register.C, 0x11);

    let mut cpu = CPU::new(MemorySpace::new((&[0x16, 0x11, HALT])));
    cpu.run();
    assert_eq!(cpu.register.D, 0x11);

    let mut cpu = CPU::new(MemorySpace::new((&[0x1E, 0x11, HALT])));
    cpu.run();
    assert_eq!(cpu.register.E, 0x11);

    let mut cpu = CPU::new(MemorySpace::new((&[0x26, 0x11, HALT])));
    cpu.run();
    assert_eq!(cpu.register.H, 0x11);

    let mut cpu = CPU::new(MemorySpace::new((&[0x2E, 0x11, HALT])));
    cpu.run();
    assert_eq!(cpu.register.L, 0x11);
}

#[test]
fn should_load_between_registers() {

    // LD A, Word
    let mut cpu = CPU::new(MemorySpace::new((&[0x7F, HALT])));
    cpu.register.A = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x78, HALT])));
    cpu.register.B = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x79, HALT])));
    cpu.register.C = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7A, HALT])));
    cpu.register.D = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7B, HALT])));
    cpu.register.E = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7C, HALT])));
    cpu.register.H = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7D, HALT])));
    cpu.register.L = 0xFF;
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);

    let mut cpu = CPU::new(MemorySpace::new((&[0x7E, 0xFF, HALT])));
    cpu.register.write_HL(0x01);
    cpu.run();
    assert_eq!(cpu.register.A, 0xFF);
}
