use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct ValidatedDate<'a, T>
where
    T: Debug,
{
    value: &'a T,
}
