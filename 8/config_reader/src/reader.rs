use crate::{config::Config, custom_error::ConfigError};

use std::{collections::HashMap, fs, io::{BufRead, BufReader, ErrorKind}};

#[allow(unused)]
pub struct ConfigReader {
    config: Config,
}

impl ConfigReader {

    const MAX_VALUES: usize = 256;

    // Altered the function a bit
    pub fn load_config(&self) -> Result<HashMap<String, String>, ConfigError> {
        let file = match fs::File::open(self.config.get_config_path()) {
            Ok(val) => val,
            Err(e) => return Err(ConfigError::Io(std::io::Error::new(ErrorKind::NotFound, e))),
        };

        let buffer_reader = BufReader::new(file);
        let values_map: HashMap<String, String> = HashMap::with_capacity(Self::MAX_VALUES);

        for line_result in buffer_reader.lines() {
            match line_result {
                Ok(val) => {
                    let line_ref = & val;
                    if is_valid_value...
                },
                Err(e) => return Err(ConfigError::Io(std::io::Error::new(ErrorKind::InvalidData, e))),
            }
        }
    

        Ok(HashMap::new())
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
