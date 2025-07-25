use mini_grep::config::Config;
use std::env::args;
use std::process;

fn main() {
    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Couldn't construct config: {err}");
        process::exit(1);
    });
    config.parse_config().unwrap_or_else(|err| {
        dbg!(config);
        eprintln!("Could not parse config: {err}");
    });
}
