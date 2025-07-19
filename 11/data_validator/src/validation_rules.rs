use crate::errors;

pub trait ValidatorRule<T: ?Sized> {
    fn validate(&self, data: &T) -> Result<(), errors::ValidatorError>;
}
