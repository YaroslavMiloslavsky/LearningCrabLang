use crate::rules::sanitization_rules::SanitizerRule;

pub struct ToLowerCaseSanitizer;

#[allow(unused)]
impl ToLowerCaseSanitizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl SanitizerRule<String> for ToLowerCaseSanitizer {
    fn sanitize(&self, data: &mut String) -> String {
        data.to_lowercase()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_lower_case() {
        let san = ToLowerCaseSanitizer::new();
        let mut s = "HELLO!".to_string();
        let result = san.sanitize(&mut s);
        assert_eq!(result, s.to_lowercase());
    }

    #[test]
    fn test_to_lower_case_empty_string() {
        let san = ToLowerCaseSanitizer::new();
        let mut s = "".to_string();
        let result = san.sanitize(&mut s);
        assert_eq!(result, s.to_lowercase());
    }
}
