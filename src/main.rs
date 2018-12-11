
mod memory;
mod cpu;

fn main() {
    let mut memory = memory::MemorySpace::new();
    let mut cpu = cpu::cpu::CPU::new(memory);
    cpu.run();
}
