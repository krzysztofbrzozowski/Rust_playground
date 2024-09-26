use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Getting the value
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score is {score:#?}");

    // Iterating through it
    for (k, v) in &scores {
        println!("Key: {k}, Value: {v}");
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Unable to use filed name due to moved ownership, for i32 it will not be moved
    // println!("{field_name}");


    // Overwriting the value via insert
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Adding the value if kv is empty, otherwise leave old value
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    updating_value_based_on_old_one();
    
}

fn updating_value_based_on_old_one() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    // The split_whitespace method returns an iterator over subslices, separated by whitespace, of the value in text.
    // The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
    // Here, we store that mutable reference in the count variable, so in order to assign to that value, 
    // we must first dereference count using the asterisk (*).
    // The mutable reference goes out of scope at the end of the for loop, 
    // so all of these changes are safe and allowed by the borrowing rules.
}

