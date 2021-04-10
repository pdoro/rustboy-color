
// Crate modules
mod soc;
mod memory;
mod tests;
mod utils;
mod cartridge;
mod configuration;

use chrono;
use soc::cpu::CPU;
use log::{debug, error, info};
use memory::MemorySpace;
use cartridge::cartridge::Cartridge;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Output;
use std::{fs::File, io::{Read, BufReader}, str::FromStr};
use color_eyre::eyre::Result;
use clap::Clap;
use configuration::Config;

fn main() -> Result<()> {
    color_eyre::install()?;

    // let opts: Opts = Opts::parse();
    let log_level = "INFO";
    let cartridge = "Tetris.gb";

    setup_logger(log_level);

    info!("Starting rustboy emulator");

    let file = File::open(cartridge).expect("Cartridge not found");
    let mut reader = BufReader::new(file);
    let mut blob = Vec::new();

    reader.read_to_end(&mut blob);

    let mut cartridge: Box<dyn Cartridge> = cartridge::cartridge::decode_cartridge(blob);
    cartridge.report();

    let mut memory = MemorySpace::new(cartridge);
    let mut cpu = CPU::new(memory);
    info!("CPU execution started");

    cpu.run();
    info!("Execution finished");

    Ok(())
}

fn setup_logger(level: &str) {
    let level = log::LevelFilter::from_str(level).expect("Invalid logging level");

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::White)
        .trace(Color::Blue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                date = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.3f]"),
                target = record.target(),
                level = colors.color(record.level()),
                message = message,
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
        .expect("Error configuring logger");
}
