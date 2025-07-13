#[allow(unused)]
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    InvalidFormat(String),
    InvalidValue(String, String),
    MissingSetting(String),
}
