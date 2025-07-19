use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct ValidatedData<'a, T>
where
    T: Debug,
{
    value: &'a T,
}
