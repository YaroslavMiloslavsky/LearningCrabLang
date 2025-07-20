use crate::errors;
use crate::rules::validation_rules::ValidatorRule;

pub struct PositiveNumberValidator;

#[allow(unused)]
impl PositiveNumberValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValidatorRule<i32> for PositiveNumberValidator {
    fn validate(&self, data: &i32) -> Result<(), errors::ValidatorError> {
        if *data > 0 {
            Ok(())
        } else {
            Err(errors::ValidatorError::ValidationError(
                "{data} is negative".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_positive_number() {
        let validator = PositiveNumberValidator::new();
        let result = validator.validate(&Box::new(1));
        assert!(result.is_ok());
    }

    #[test]
    fn test_negative_number() {
        let validator = PositiveNumberValidator::new();
        let result = validator.validate(&Box::new(-11));
        assert!(result.is_err());
    }

    #[test]
    fn test_zero() {
        let validator = PositiveNumberValidator::new();
        let result = validator.validate(&Box::new(0));
        assert!(result.is_err());
    }
}
