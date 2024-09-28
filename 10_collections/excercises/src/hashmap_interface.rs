use std::io;


pub fn read_user_input() -> String {
    let mut input_command = String::new();

    io::stdin()
        .read_line(&mut input_command)
        .expect("Failed to read input");

    input_command
}

fn parse_command(command: &String) {
    for word in command.split_whitespace() {
        
        match word.to_lowercase().as_ref() {
            "add" => todo!(),
            _ => todo!(),
        }
    }
}

