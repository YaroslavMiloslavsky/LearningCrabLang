use crate::rules::sanitization_rules::SanitizerRule;

pub struct TrimWhitespaceSanitizer;

#[allow(unused)]
impl TrimWhitespaceSanitizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl SanitizerRule<String> for TrimWhitespaceSanitizer {
    fn sanitize(&self, data: &mut String) -> String {
        data.trim().trim_end().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_with_spaces() {
        let mut s = " Some String ".to_string();
        let san = TrimWhitespaceSanitizer::new();
        let result = san.sanitize(&mut s);
        assert_eq!(result, "Some String");
    }

    #[test]
    fn test_string_without_spaces() {
        let mut s = "Some String".to_string();
        let san = TrimWhitespaceSanitizer::new();
        let result = san.sanitize(&mut s);
        assert_eq!(result, "Some String");
    }

    #[test]
    fn test_empty_string() {
        let mut s = "".to_string();
        let san = TrimWhitespaceSanitizer::new();
        let result = san.sanitize(&mut s);
        assert_eq!(result, "");
    }
}
