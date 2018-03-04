use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is always the program binary, we want two additional arguments
    let (query, filename) = parse_config(&args);

    let mut f = File::open(query).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error reading the file!");
    println!("File contents: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    match args.len() == 3 {
        true => {
            let query = &args[1];
            let search_term = &args[2];
            println!("Query Target: {:?}, Search For: {:?}", query, search_term);
        }
        _ => panic!("Please provide exactly one file to query and one term to search for"),
    };
}
