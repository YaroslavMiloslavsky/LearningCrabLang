use std::fmt::Debug;

use crate::data_holder::ValidatedData;
use crate::errors::ValidatorError;
use crate::rules::{sanitization_rules::SanitizerRule, validation_rules::ValidatorRule};

pub struct DataProcessor<T: ?Sized> {
    validators: Vec<Box<dyn ValidatorRule<T>>>,
    sanitizers: Vec<Box<dyn SanitizerRule<T>>>,
}

impl<T: Debug + Default> DataProcessor<T> {
    pub fn new(
        validators: Vec<Box<dyn ValidatorRule<T>>>,
        sanitizers: Vec<Box<dyn SanitizerRule<T>>>,
    ) -> Self {
        Self {
            validators,
            sanitizers,
        }
    }

    // 0 Cloning!! But of course we moved data twice ;) is that costly? Is that correct?
    pub fn process(&self, data: &mut T) -> Result<ValidatedData<T>, ValidatorError> {
        let mut sanitized_data = std::mem::take(data);
        for sanitizer in &self.sanitizers {
            sanitized_data = sanitizer.sanitize(&mut sanitized_data);
        }

        for validator in &self.validators {
            match validator.validate(&sanitized_data) {
                Ok(()) => continue,
                Err(e) => {
                    return Err(ValidatorError::ValidationError(format!(
                        "Could not validate data {sanitized_data:?}, cause: {e:?}"
                    )));
                }
            }
        }
        Ok(ValidatedData::new(sanitized_data))
    }
}
