use std::env;
use std::process;
use colored::Colorize;
use minigrep::Config;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!( + " {}", String::from(err).red());
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
