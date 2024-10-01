use std::fmt::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::io::{self, Read};

// fn main() {
//     let file_result = File::open("some_file.txt");

//     let file = match file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {

//             ErrorKind::NotFound => match File::create("some_file.txt") {
//                 Ok(file_created) => file_created,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),   
//             }

//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         }
//     };
// }

// Implementation using closures
fn main() {
    let file = File::open("some_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("some_file").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

// Using expect which contains panic with custom error 
fn main2() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

// Propagating error to another match
fn main3 () -> Result<String, io::Error> {
    let file_open_result = File::open("some_file.txt");

    let mut file = match file_open_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut some_value_from_text = String::new();

    match file.read_to_string(&mut some_value_from_text) {
        Ok(_) => some_value_from_text,
        Err(e) => Err(e),
    }
}

