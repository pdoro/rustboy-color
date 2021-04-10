
use log;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(name = "basic")]
pub struct Config {

    #[clap(short, long, parse(from_os_str))]
    cartridge: PathBuf,

    #[clap(short, long, default_value = "INFO")]
    log_level: String,

    #[clap(short, long)]
    gui: bool
}

// impl From<ArgMatches> for Config {
//     fn from(matches: ArgMatches) -> Self {
//         Config {
//             log_level: log::LevelFilter::from_str(matches.value_of("verbose").unwrap()).unwrap(),
//             gui : ON
//         }
//     }
// }