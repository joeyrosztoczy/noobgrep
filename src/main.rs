use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is always the program binary, we want two additional arguments
    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        match args.len() == 3 {
            true => {
                let query = args[1].clone();
                let filename = args[2].clone();
                println!("Query Target: {:?}, Search For: {:?}", query, filename);
                return Config { query, filename };
            }
            _ => panic!("Please provide exactly one file to query and one term to search for"),
        };
    }
}
