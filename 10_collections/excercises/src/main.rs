mod median_mode;
mod pig_latin;
mod hashmap_interface;

fn main() {
    // Given a list of integers, use a vector and return 
    // the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    
    // 1. Median
    let mut list = vec![1, 5, 12, 8, 5, 9, 14, 88];
    list.sort();

    let median = median_mode::median(&list);
    println!("median is {median:#?}");

    // 2. Mode
    let mode = median_mode::mode(&list);
    println!("Max occurance has {mode:#?}");

    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
    // so first becomes irst-fay. 
    // Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
    // Keep in mind the details about UTF-8 encoding!

    // 3. Pig latin
    // Word 'bas' will be -> as-bay 
    // Word 'elo' will be -> lo-hay
    let word = String::from("blo");
    let pig_latin_word = pig_latin::pig_latin(&word);
    println!("Pig latin for word '{word}' is '{pig_latin_word}'");

    // 4. Hashmap interface
    // Using a hash map and vectors, create a text interface to allow a user 
    // to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
    // Then let the user retrieve a list of all people in a department 
    // or all people in the company by department, sorted alphabetically.
    let command = hashmap_interface::read_user_input();
}



