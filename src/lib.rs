use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("File contents: \n{}", contents);
    
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn search(query: &str,contents:  &str) -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\

        Rust:
        safe, fast, productive,
        Pick 3.";

        assert_eq!(vec!["safe", "fast", "productive"], search(query, contents));
    }
        
}
