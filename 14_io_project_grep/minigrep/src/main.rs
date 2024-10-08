use std::env;
use std::fs;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let file_content = fs::read_to_string(config.file_path).expect("Uppps");

    println!("With text:\n{file_content}");
}

struct Config {
    query:      String,
    file_path:  String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}