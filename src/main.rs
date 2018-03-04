extern crate noobgrep;

use noobgrep::Config;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If we don't successfully create the Config struct, exit with helpful message
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = noobgrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
