use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // Funny issue if we not "use std::process" ->
        // mismatched types
        // expected `Config`, found `()`rustcClick for full compiler diagnostic
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // If let because don't need to unwrap stuff returning
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
