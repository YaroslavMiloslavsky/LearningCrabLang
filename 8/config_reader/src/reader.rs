use crate::{config::Config, custom_error::ConfigError};

use std::{fs, io::ErrorKind};

#[allow(unused)]
pub struct ConfigReader {
    config: Config,
}

impl ConfigReader {
    pub fn real_config_values(&self) {
        let buffer = fs::read_to_string(self.config.get_config_path())
            .unwrap_or_else(|e| panic!("Could not read values from config: {e}"));
        println!("{buffer}");
    }
}

impl ConfigReader {
    pub fn new(config: Option<&Config>) -> Result<Self, ConfigError> {
        let config = if let Some(val) = config {
            val.clone()
        } else {
            return Err(ConfigError::Io(std::io::Error::new(
                ErrorKind::NotFound,
                "Could not find a valid configuration, please pass a valid configuration",
            )));
        };

        Ok(Self { config })
    }
}
