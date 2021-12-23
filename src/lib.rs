use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }
        let query = args[0].clone();
        let filename = args[1].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &text)
    } else {
        search_case_insensitive(&config.query, &text)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

fn search<'a>(query: & str, text: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in text.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

fn search_case_insensitive<'a>(query: & str, text: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in text.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}