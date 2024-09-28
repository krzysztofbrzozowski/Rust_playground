use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector and return 
    // the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    
    // 1. Median
    let mut list = vec![1, 5, 12, 8, 5, 9, 14, 88];
    list.sort();

    let median = median(&list);
    println!("median is {median:#?}");

    // 2. Mode
    let mode = mode(&list);
    println!("Max occurance has {mode:#?}");

    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
    // so first becomes irst-fay. 
    // Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
    // Keep in mind the details about UTF-8 encoding!

    // 3. Pig latin
    // Word 'bas' will be -> as-bay 
    // Word 'elo' will be -> lo-hay
    let word = String::from("blo");
    let pig_latin_word = pig_latin(&word);
    println!("Pig latin for word '{word}' is '{pig_latin_word}'");


}

fn median(list: &Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        // let median: Option<i32> = None;
        return None;
    }

    let idx = list.len() / 2;
    println!("Sorted list is {list:#?}");

    match list.len() % 2 {
        // If devide is even number
        0 => {
            let median = &list[idx - 1] + &list[idx];
            return Some(median);
        },

        1 => {
            let median = &list[idx];
            return Some(*median);
        },

        _ => None,
    }
}

fn mode(list: &Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }

    let mut map = HashMap::new();
    for item in list {
        let num = map.entry(item).or_insert(0);
        *num += 1;
    }
    println!("Map is {map:#?}");
    // TODO - do not understand, copied that from stack
    let max_val = map.iter().max_by_key(|entry | entry.1).unwrap();

    Some(**max_val.0)
}

fn pig_latin(some_word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut postfix = String::from("ay");
    let mut return_word = String::from("");

    let letters: Vec<char> = some_word.chars().collect();

    for (idx, c) in letters.iter().enumerate() {
        if idx == 0 && vowels.iter().any(|&v| *c == v) {
            postfix = format!("-h{}", postfix);
            continue;
        }
        else if idx == 0 {
            postfix = format!("-{}{}", *c, postfix);
            continue;
        }
        return_word.push(*c);
    }
    return_word.push_str(&postfix);
    return_word
}

// Smarter solution for pig latin
fn pig_latin_2(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut char_iter = word.chars();
    let first_letter = char_iter.next().unwrap();

    if vowels.contains(&first_letter) {
        format!("{}-hay", &word)
    } else {
        let remaining: String = char_iter.take(word.len() - 1).collect();
        format!("{}-{}ay", &remaining, first_letter)
    }
}

fn pig_latin_3(word: &str) -> String {
    let mut letters = word.chars();
    let first_letter = letters.next().unwrap();

    let lowercase_first_letter = first_letter.to_lowercase().next().unwrap();

    match lowercase_first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", letters.as_str(), lowercase_first_letter),
    }

}