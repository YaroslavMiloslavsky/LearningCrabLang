use crate::custom_error::ConfigError;

use std::{fs, io::ErrorKind, path::Path};

#[derive(Clone)]
pub struct Config {
    config_path: String,
}

impl Config {
    pub fn get_config_path(&self) -> &str {
        &self.config_path
    }
}

impl Config {
    // I looked that up, I have no idea what it is used for and how yet
    const CONFIG_FILE_NAME: &'static str = "config.txt";

    // This is a function for testing std::fs and getting a feeling of using custom errors
    fn new_default() -> Result<Self, ConfigError> {
        let mut files_in_current_folder =
            fs::read_dir("./src").expect("Could not read from directory");

        if files_in_current_folder.any(|file| {
            file.as_ref().unwrap().file_name().to_str().unwrap() == Self::CONFIG_FILE_NAME
        }) {
            let config_path = format!("./src/{}", Self::CONFIG_FILE_NAME);
            Ok(Self { config_path })
        } else {
            Err(ConfigError::Io(std::io::Error::new(
                ErrorKind::NotFound,
                format!("No {} file was found", Self::CONFIG_FILE_NAME),
            )))
        }
    }

    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        if Path::new(config_path).exists() {
            println!("Found {config_path}");
            Ok(Self {
                config_path: config_path.to_string(),
            })
        } else {
            println!("Could not read {config_path}, trying to read default");
            Self::new_default()
        }
    }
}
