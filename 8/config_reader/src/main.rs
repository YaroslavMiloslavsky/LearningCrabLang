#![warn(clippy::all, clippy::pedantic)]
mod config;
mod custom_error;
mod reader;

use crate::{config::Config, reader::ConfigReader};

fn main() {
    let config = Config::new(".\\src\\config.txt").expect("Could not open the config file");
    let mut reader = ConfigReader::new(Some(&config)).expect("Could not initialize config reader");

    match reader.load_config() {
        Ok(val) => println!("{val:#?}"),
        Err(e) => panic!("Encountered error while loading config: {e:#?}"),
    }

    let some_key = String::from("too_big");
    let value = reader.get_setting_as_u32(&some_key).unwrap();

    println!("{some_key} : {value}");
}

/*
I have tested all of the things you've mentioned in main loop while writing

I slightly altered you request and played around with some ideas, I hope you will accept this fact
Also, I tried to reduce smell in this code, but lifetimes are needed.

    Conceptual Questions
    panic va Result -> panic aborts the program (unless configured otherwise) while Result gives us the opportunity explicitly handle the error, we still can call panic
    I provided lots of examples of when to use Result, panic and unwrap (still the same panic)

    The ? Operator
    This operator simplifies the error propagation, even though sometimes we would nee to either map the error or implement the From trait
    The question regarding the methods, without ? the methods would still work, but the would be less readable.

    Custom Error Types
    Custom error type would help us to handle app specific errors, which is better then throwing just default errors
*/