use std::env;

fn main() {
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();
    let query = &args[0];
    let filename = &args[1];

    println!("Searching for: {}", query);
    println!("In the file: {}", filename);

    
}
