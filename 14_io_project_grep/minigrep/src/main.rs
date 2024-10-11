use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query:      String,
    file_path:  String,
}

impl Config {
    // &'static str -> will live lifetime of program, in this case point to Error
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    println!("File content is {content}");
    Ok(())
}
