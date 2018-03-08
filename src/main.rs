extern crate search;

use search::Config;

use std::env;
use std::process;

fn main() {
    // If we don't successfully create the Config struct, exit with helpful message
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = search::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
