use crate::{config::Config, custom_error::ConfigError};

use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, ErrorKind},
};

#[allow(unused)]
pub struct ConfigReader {
    config: Config,
    values: HashMap<String, String>,
}

impl ConfigReader {
    // pub fn get_setting_as_u32(&self, key: &str) -> Result<u32, ConfigError> {

    // }

    // Altered the function a bit
    pub fn load_config(&mut self) -> Result<&HashMap<String, String>, ConfigError> {
        let file = fs::File::open(self.config.get_config_path())?;

        let buffer_reader = BufReader::new(file);

        for line_result in buffer_reader.lines() {
            match line_result {
                Ok(val) => {
                    let values = Self::get_values(&val)?;
                    let (key, value) = values;
                    self.values.insert(key.to_string(), value.to_string());
                }
                Err(e) => {
                    return Err(ConfigError::Io(std::io::Error::new(
                        ErrorKind::InvalidData,
                        e,
                    )));
                }
            }
        }
        Ok(&self.values)
    }
}

impl ConfigReader {
    const MAX_VALUES: usize = 256;

    pub fn new(config: Option<&Config>) -> Result<Self, ConfigError> {
        let config = if let Some(val) = config {
            val.clone()
        } else {
            return Err(ConfigError::Io(std::io::Error::new(
                ErrorKind::NotFound,
                "Could not find a valid configuration, please pass a valid configuration",
            )));
        };

        Ok(Self {
            config,
            values: HashMap::with_capacity(Self::MAX_VALUES),
        })
    }

    // contains exactly 1 =
    fn get_values(val: &str) -> Result<(&str, &str), ConfigError> {
        let sanitized_val = val.trim();
        let mut values: Vec<&str> = sanitized_val.split('=').collect();

        match values.len() {
            2 => {
                let Some(value) = values.pop() else {
                    return Err(ConfigError::InvalidValue(
                        val.to_string(),
                        "Not available".to_string(),
                    ));
                };
                let Some(key) = values.pop() else {
                    return Err(ConfigError::InvalidValue(
                        val.to_string(),
                        "Not available".to_string(),
                    ));
                };
                Ok((key, value))
            }
            _ => Err(ConfigError::InvalidFormat(val.to_string())),
        }
    }
}
