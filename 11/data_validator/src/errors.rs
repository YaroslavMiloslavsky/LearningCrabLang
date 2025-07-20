use regex::Error;

#[allow(unused)]
#[derive(Debug)]
pub enum ValidatorError {
    ValidationError(String),
    RegexValidationError(Error),
}

impl From<Error> for ValidatorError {
    fn from(value: Error) -> Self {
        ValidatorError::RegexValidationError(value)
    }
}
