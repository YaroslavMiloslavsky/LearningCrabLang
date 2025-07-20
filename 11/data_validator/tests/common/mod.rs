use data_validator::DataProcessor;
use data_validator::EmailFormatValidator;
use data_validator::MinLengthValidator;

use data_validator::ToLowerCaseSanitizer;
use data_validator::TrimWhitespaceSanitizer;

pub fn create_processor_email_max_len_20() -> DataProcessor<String>{
    let email_val = Box::new(EmailFormatValidator::new());
    let min_val = Box::new(MinLengthValidator::new(20));

    let trim_san = Box::new(TrimWhitespaceSanitizer::new());
    let low_san = Box::new(ToLowerCaseSanitizer::new());

   DataProcessor::new(vec![email_val, min_val], vec![trim_san, low_san])
}
