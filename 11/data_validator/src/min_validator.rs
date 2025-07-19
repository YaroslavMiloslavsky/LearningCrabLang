use crate::errors;
use crate::validation_rules::ValidatorRule;

pub struct MinLengthValidator {
    length: usize,
}

impl MinLengthValidator {
    pub fn new(length: usize) -> Self {
        Self { length }
    }
}

impl ValidatorRule<str> for MinLengthValidator {
    fn validate(&self, data: &str) -> Result<(), errors::ValidatorError> {
        if data.is_empty() {
            Err(errors::ValidatorError::ValidationError(
                "Data should be non empty".to_string(),
            ))
        } else if data.len() > self.length {
            Err(errors::ValidatorError::ValidationError(format!(
                "Data should not exceed {} but was {}",
                self.length,
                data.len()
            )))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_within_range() -> Result<(), errors::ValidatorError> {
        let min_validator = MinLengthValidator::new(10);
        let test_str = "test";
        min_validator.validate(test_str)
    }

    #[test]
    fn test_empty_string() {
        let min_validator = MinLengthValidator::new(10);
        let test_str = "";
        let result = min_validator.validate(test_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_big() {
        let min_validator = MinLengthValidator::new(10);
        let test_str = "a".repeat(11);
        let result = min_validator.validate(&test_str);
        assert!(result.is_err());
    }
}
