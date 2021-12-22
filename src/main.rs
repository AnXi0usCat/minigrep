use std::env;
use std::fs;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let config =  Config::new(&args);

    println!("Searching for: {}", config.query);
    println!("In the file: {}", config.filename);

    let text = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", text)
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[0].clone();
        let filename = args[1].clone();

        Config { query, filename }
    }
}
