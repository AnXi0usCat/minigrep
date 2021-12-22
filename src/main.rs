use std::env;
use std::fs;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let query = &args[0];
    let filename = &args[1];

    println!("Searching for: {}", query);
    println!("In the file: {}", filename);

    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", text)
}
