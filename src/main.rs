extern crate chrono;

use log::{info, debug, error};
mod utils;
mod memory;
mod cpu;
use std::panic;

fn main() {
    setup_logger();

    info!("Starting rustboy emulator");
    let mut memory = memory::MemorySpace::new();
    let mut cpu = cpu::cpu::CPU::new(memory);
    info!("Execution started");

    /*
    panic:: catch_unwind(|| {
        error!("[=============== ERROR REPORT ===============]");
        error!("{:#?}", cpu);
    });
    */

    cpu.run();
    info!("Execution finished");
}

fn setup_logger() {
    use fern::colors::{Color, ColoredLevelConfig};
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
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .expect("Error configuring logger");
    debug!("Log setup correctly");
}
