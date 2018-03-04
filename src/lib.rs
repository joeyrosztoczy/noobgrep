use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("Found line: {}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() == 3 {
            true => {
                let query = args[1].clone();
                let filename = args[2].clone();

                let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
                println!("Query Target: {:?}, Search For: {:?}", query, filename);
                return Ok(Config { query, filename, case_sensitive });
            }
            _ => return Err("Please provide exactly one file to query and one term to search for"),
        };
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            let clean_line = line.trim();
            results.push(clean_line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // to_lowercase creates a lowercase copy String
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            let clean_line = line.trim();
            results.push(clean_line);
        }
    }
    results
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

        assert_eq!(vec!["safe, fast, productive,"], search(query, contents));
    }

    #[test]
    // Should not find "Duct" in last line
    fn case_sensitive() {
        let query = "duct";
        let contents = "\

        Rust:
        safe, fast, productive,
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive,"], search(query, contents));
    }
    #[test]
    // Should find "Duct" in last line
    fn case_insensitive() {
        let query = "duct";
        let contents = "\

        Rust:
        safe, fast, productive,
        Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive,", "Duct tape."],
            search_case_insensitive(query, contents)
        );
    }
}
