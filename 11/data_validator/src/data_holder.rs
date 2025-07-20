use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct ValidatedData<T> // Here I decided to not use lifetimes, as ValidatedData will always be returned from process, it is better to own the data and be complete
where
    T: Debug,
{
    value: T,
}

impl<T> ValidatedData<T>
where
    T: Debug,
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}
