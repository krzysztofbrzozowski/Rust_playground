use std::env;
use std::fs;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let file_content = fs::read_to_string(file_path).expect("Uppps");

    println!("With text:\n{file_content}");
}