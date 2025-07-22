#![warn(clippy::all, clippy::pedantic)]

pub static MIN_ARGS: usize = 3;

pub mod config {
    use std::{
        env,
        error::Error,
        fs::File,
        io::{self, BufRead},
    };

    #[derive(Debug)]
    pub struct Config<'a> {
        query: &'a str,
        file_path: &'a str,
        ignore_case: bool,
    }

    impl<'a> Config<'a> {
        #[must_use]
        pub fn build(args: &'a [String]) -> Result<Self, String> {
            if args.len() < 3 {
                Err("Not enough args".to_string())
            } else {
                let query = &args[1];
                let file_path = &args[2];
                let ignore_case = env::var("IGNORE_CASE").is_ok();
                Ok(Self {
                    query,
                    file_path,
                    ignore_case,
                })
            }
        }

        /// Does something with a file
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// - The file cannot be opened
        /// - The file cannot be read
        /// - The file contains invalid data
        pub fn parse_config(&self) -> Result<(), Box<dyn Error>> {
            let file_path = format!("./{}.txt", self.file_path);
            let file = File::open(file_path)?;
            let buf_reader = io::BufReader::new(file);

            for line in buf_reader.lines() {
                let content = line?;
                match self.ignore_case {
                    true => {
                        if content.to_lowercase().contains(self.query.to_lowercase().as_str()) {
                            println!("{content}");
                        }
                    }
                    false => {
                        if content.contains(self.query) {
                            println!("{content}");
                        }
                    }
                }
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn parse_config_pass() -> Result<(), Box<dyn Error>> {
            let args = vec![
                "ignore".to_string(),
                "rust".to_string(),
                "tests/some_text".to_string(),
            ];
            let config = Config::build(&args)?;
            config.parse_config()
        }

        #[test]
        fn parse_config_no_match_pass() -> Result<(), Box<dyn Error>> {
            let args = vec![
                "ignore".to_string(),
                "glib".to_string(),
                "tests/some_text".to_string(),
            ];
            let config = Config::build(&args)?;
            config.parse_config()
        }

        #[test]
        fn parse_config_too_many_params_pass() -> Result<(), Box<dyn Error>> {
            let args = vec![
                "ignore".to_string(),
                "rust".to_string(),
                "tests/some_text".to_string(),
                "some_more".to_string(),
            ];
            let config = Config::build(&args)?;
            config.parse_config()
        }

        #[test]
        fn parse_config_not_enough_params_fail() {
            let args = vec!["ignore".to_string(), "rust".to_string()];
            if let Ok(config) = Config::build(&args) {
                let result = config.parse_config();
                assert!(result.is_err());
            }
        }

        #[test]
        fn parse_config_no_params_fail() {
            let args = Vec::new();
            if let Ok(config) = Config::build(&args) {
                let result = config.parse_config();
                assert!(result.is_err());
            }
        }

        #[test]
        fn parse_config_no_file_fail() {
            let args = vec![
                "ignore".to_string(),
                "rust".to_string(),
                "tests/oops".to_string(),
            ];
            if let Ok(config) = Config::build(&args) {
                let result = config.parse_config();
                assert!(result.is_err());
            }
        }
    }
}
