
use log;
use clap::ArgMatches;
use crate::configuration::SwitchStatus::ON;


pub struct Config {
    log_level: log::LevelFilter,
    gui: SwitchStatus
}

pub enum SwitchStatus { ON, OFF }

impl From<ArgMatches> for Config {
    fn from(matches: ArgMatches) -> Self {
        Config {
            log_level: log::LevelFilter::from_str(matches.value_of("verbose").unwrap()).unwrap(),
            gui : ON
        }
    }
}