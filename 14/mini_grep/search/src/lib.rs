//! This module is mainly for testing the different doc features used by cargo
//!
//!

#![warn(clippy::all, clippy::pedantic)]

pub use config::Config;

pub mod config {
    //! The config data itself
    use std::{
        env,
        error::Error,
        fs::File,
        io::{self, BufRead},
        thread,
    };

    /// Config struct that holds the query to search and the file path
    /// also checks for global env to ignore case
    #[derive(Debug)]
    pub struct Config {
        query: String,
        file_path: String,
        ignore_case: bool,
    }

    impl Config {
        /// Creates a new Config instance from provided iterator
        /// # Examples
        /// ```
        /// let args = vec!["ignore".to_string(),"rust".to_string(),"tests/some_text".to_string()];
        /// let config = search::Config::build(args.into_iter());
        /// assert!(config.is_ok());
        /// ```
        /// # Errors:
        /// This function will return error if:
        /// - Less then 3 arguments were provided
        /// - The query argument was not provided
        /// - The file_path argument was not provided
        #[must_use]
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
            args.next();
            let Some(query) = args.next() else {
                return Err("Please provide query string!");
            };

            let Some(file_path) = args.next() else {
                return Err("Please provide file path!");
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Self {
                query,
                file_path,
                ignore_case,
            })
        }

        /// Does something with a file
        /// 
        /// # Examples
        /// ```
        /// let args = vec!["ignore".to_string(),"rust".to_string(),"tests/some_text".to_string()];
        /// let config = search::Config::build(args.into_iter()).unwrap();
        /// assert!(config.parse_config().is_ok());
        /// ```
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

            let ignore_case = self.ignore_case;

            let jobs: Vec<_> = buf_reader
                .lines()
                .filter_map(std::result::Result::ok)
                .map(|data| {
                    let query = self.query.to_string();
                    thread::spawn(move || Config::search(query, data, ignore_case))
                })
                .collect();

            for job in jobs {
                job.join().unwrap();
            }

            Ok(())
        }

        fn search(query: String, data: String, ignore_case: bool) {
            if ignore_case {
                if data.to_lowercase().contains(query.to_lowercase().as_str()) {
                    println!("{data}");
                }
            } else {
                if data.contains(&query) {
                    println!("{data}");
                }
            }
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
            let config = Config::build(args.into_iter())?;
            config.parse_config()
        }

        #[test]
        fn parse_config_no_match_pass() -> Result<(), Box<dyn Error>> {
            let args = vec![
                "ignore".to_string(),
                "glib".to_string(),
                "tests/some_text".to_string(),
            ];
            let config = Config::build(args.into_iter())?;
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
            let config = Config::build(args.into_iter())?;
            config.parse_config()
        }

        #[test]
        fn parse_config_not_enough_params_fail() {
            let args = vec!["ignore".to_string(), "rust".to_string()];
            if let Ok(config) = Config::build(args.into_iter()) {
                let result = config.parse_config();
                assert!(result.is_err());
            }
        }

        #[test]
        fn parse_config_no_params_fail() {
            let args = Vec::new();
            if let Ok(config) = Config::build(args.into_iter()) {
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
            if let Ok(config) = Config::build(args.into_iter()) {
                let result = config.parse_config();
                assert!(result.is_err());
            }
        }
    }
}
