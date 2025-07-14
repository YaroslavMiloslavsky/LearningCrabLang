#[allow(unused)]
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    InvalidFormat(String),
    InvalidValue(String, String),
    MissingSetting(String),
}

// From the brief example in chapter 9
impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value)
    }
}
