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

    results
        .iter()
        .for_each(|line| println!("Found line: {}", line));

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        match args.len() == 3 {
            true => {
                args.next(); // Skip filename arg
                let query = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a query string!"),
                };

                let filename = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a filename!"),
                };

                let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
                println!("Query Target: {:?}, Search For: {:?}", query, filename);
                return Ok(Config {
                    query,
                    filename,
                    case_sensitive,
                });
            }
            _ => return Err("Please provide exactly one file to query and one term to search for"),
        };
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            return lower.contains(&lower_query);
        })
        .map(|line| line.trim())
        .collect()
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
