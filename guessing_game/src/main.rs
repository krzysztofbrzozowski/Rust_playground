use std::io;
use rand::prelude::*;
use std::cmp;

fn main() {
    loop {
        println!("Put in the number");

        let mut input = String::new();
        let random_num = rand::thread_rng().gen_range(1..=100);
        // println!("Random num is {random_num}");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
                println!("Input has to be some num base"); 
                continue;
            },
        };
        println!("Input string is {input}");

        match input.cmp(&random_num) {
            cmp::Ordering::Less => println!("Value too small"),
            cmp::Ordering::Greater => println!("Value too big"),
            cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
