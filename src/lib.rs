use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("cannot parse search value")
        };
        let path: String = match args.next() {
            Some(arg) => arg,
            None => return  Err("cannot parse file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query,path,ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    let results = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents)
    };
    for line in results {
        print!("{line}\n");
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query,contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}