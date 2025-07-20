use crate::errors;
use crate::rules::validation_rules::ValidatorRule;
use regex::Regex;

pub struct EmailFormatValidator;

#[allow(unused)]
impl EmailFormatValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValidatorRule<String> for EmailFormatValidator {
    fn validate(&self, data: &String) -> Result<(), errors::ValidatorError> {
        let email_rgx = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")?;
        match email_rgx.captures(data) {
            Some(_) => Ok(()),
            None => Err(errors::ValidatorError::ValidationError(
                "{data} is not a valid email".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_email() {
        let valid_email = "example@some.com".to_string();
        let validator = EmailFormatValidator::new();
        let result = validator.validate(&valid_email);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_email_not_fully_classified() {
        let invalid_email = "user@".to_string();
        let validator = EmailFormatValidator::new();
        let result = validator.validate(&invalid_email);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_illegal_format() {
        let invalid_email = "user@exampple@ex.com".to_string();
        let validator = EmailFormatValidator::new();
        let result = validator.validate(&invalid_email);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_empty_email() {
        let invalid_email = "".to_string();
        let validator = EmailFormatValidator::new();
        let result = validator.validate(&invalid_email);
        assert!(result.is_err());
    }
}
