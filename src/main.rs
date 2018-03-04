use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is always the program binary, we want two additional arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {}", err);
        process::exit(1);
    });

    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error reading the file!");
    println!("File contents: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() == 3 {
            true => {
                let query = args[1].clone();
                let filename = args[2].clone();
                println!("Query Target: {:?}, Search For: {:?}", query, filename);
                return Ok(Config { query, filename });
            }
            _ => return Err("Please provide exactly one file to query and one term to search for"),
        };
    }
}
