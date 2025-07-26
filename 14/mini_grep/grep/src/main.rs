use search::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Couldn't construct config: {err}");
        process::exit(1);
    });
    config.parse_config().unwrap_or_else(|err| {
        dbg!(config);
        eprintln!("Could not parse config: {err}");
    });
}
