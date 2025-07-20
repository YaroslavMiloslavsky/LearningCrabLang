use crate::errors;
use crate::rules::validation_rules::ValidatorRule;

pub struct MinLengthValidator {
    length: usize,
}

#[allow(unused)]
impl MinLengthValidator {
    pub fn new(length: usize) -> Self {
        Self { length }
    }
}

// I changed it to String for better performance and less cloning
// Does this change hurt flexibility and performance?
impl ValidatorRule<String> for MinLengthValidator {
    fn validate(&self, data: &String) -> Result<(), errors::ValidatorError> {
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
        let test_str = "test".to_string();
        min_validator.validate(&test_str)
    }

    #[test]
    fn test_empty_string() {
        let min_validator = MinLengthValidator::new(10);
        let test_str = "".to_string();
        let result = min_validator.validate(&test_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_big() {
        let min_validator = MinLengthValidator::new(10);
        let test_str = "a".repeat(11).to_string();
        let result = min_validator.validate(&test_str);
        assert!(result.is_err());
    }
}
