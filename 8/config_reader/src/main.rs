#![warn(clippy::all, clippy::pedantic)]
mod config;
mod custom_error;
mod reader;

use crate::{config::Config, reader::ConfigReader};

fn main() {
    let config = Config::new(".\\src\\config.txt").expect("Could not open the config file");
    let reader = ConfigReader::new(Some(&config)).expect("Could not initialize config reader");
    reader.real_config_values();
}
