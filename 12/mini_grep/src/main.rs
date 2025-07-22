use mini_grep::MIN_ARGS;
use mini_grep::config::Config;
use std::env::args;
use std::process;

fn main() {
    let args: Vec<String> = args().collect();
    let args_len = args.len();
    if args_len < MIN_ARGS {
        eprintln!(
            "Did not provide enough arguments, please provide at least {} more",
            (3 - args_len)
        );
    } else {
        let config = match Config::build(&args) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Couldn't construct config: {e}");
                process::exit(1);
            }
        };
        if let Err(e) = config.parse_config() {
            dbg!(config);
            eprintln!("Could not parse config: {e}");
        }
    }
}
