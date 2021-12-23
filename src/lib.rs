use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }
        let query = args[0].clone();
        let filename = args[1].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &text) {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: & str, text: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in text.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}