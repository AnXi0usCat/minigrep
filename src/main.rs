use std::env;
use std::fs;
use std::process;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let config =  Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In the file: {}", config.filename);

    run(config);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }
        let query = args[0].clone();
        let filename = args[1].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) {
    let text = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", text)
}
