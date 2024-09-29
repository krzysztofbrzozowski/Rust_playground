use std::{io, process::exit, collections::HashMap};

pub struct HashmapInterface {
    input_command: String,
    company: HashMap<String, Vec<String>>
}


impl HashmapInterface {
    pub fn new() -> Self {
        let mut input_command = String::from("");
        let mut company = HashMap::new();

        HashmapInterface { input_command, company }
    }

    pub fn read_user_input(&mut self) {
        self.input_command.clear();

        println!("Type command -> <action> <person> <where>");
        println!("Example: Add Amir to Sales");
    
        io::stdin()
            .read_line(&mut self.input_command)
            .expect("Failed to read input");
    }
    
    // Needed to add &mut since add_item_to_company requires mutable var to add something into HashMap
    pub fn parse_command(&mut self) {
        let commands_vec: Vec<&str> = self.input_command.split_whitespace().collect();
    
        match commands_vec[0].to_lowercase().as_ref() {
            "add" => {
                let person = commands_vec[1].to_string();
                let department = commands_vec[3].to_string();

                self.add_item_to_company(department, person);
                println!("Here will add something")
            },
            _ => {
                println!("Unimplemented action, will exit");
                exit(0);
            }
        }
    }

    fn add_item_to_company(&mut self, department: String, person: String) {
        self.company
            .entry(department)
            .or_insert_with(Vec::new)
            .push(person);
    }

    pub fn print_company(&mut self) {
        for (department, persons) in &mut self.company {
            persons.sort();

            println!("{}: {:?}", department, persons);
        }
    }
}

