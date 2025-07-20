#![warn(clippy::all, clippy::pedantic)]
#![allow(unused)]

mod data_holder;
mod errors;
mod processor;
mod rules;
mod sanitizer;
mod validator;

pub use processor::DataProcessor;
pub use validator::{email_validator::EmailFormatValidator, min_validator::MinLengthValidator, positive_number_validator::PositiveNumberValidator};
pub use sanitizer::{to_lower_case_sanitizer::ToLowerCaseSanitizer, white_space_sanitizer::TrimWhitespaceSanitizer};