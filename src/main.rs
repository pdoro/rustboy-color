
// Crate modules
mod soc;
mod memory;
mod tests;
mod utils;
mod cartrigbe;

use chrono;
use soc::cpu::CPU;
use log::{debug, error, info};
use memory::MemorySpace;
use fern::colors::{Color, ColoredLevelConfig};
//use clap::App;
use std::str::FromStr;

fn main() {
    //let yaml = load_yaml!("../cli.yaml");
    //let cfg = App::from_yaml(yaml).get_matches();

    let log_level = "trace"; //cfg.value_of("verbose").unwrap();

    setup_logger(log_level);

    info!("Starting rustboy emulator");
    let mut memory = MemorySpace::default();
    let mut cpu = CPU::new(memory);
    info!("CPU execution started");

    cpu.run();
    info!("Execution finished")
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
