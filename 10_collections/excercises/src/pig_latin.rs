pub fn pig_latin(some_word: &str) -> String {
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
pub fn pig_latin_2(word: &str) -> String {
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

pub fn pig_latin_3(word: &str) -> String {
    let mut letters = word.chars();
    let first_letter = letters.next().unwrap();

    let lowercase_first_letter = first_letter.to_lowercase().next().unwrap();

    match lowercase_first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", letters.as_str(), lowercase_first_letter),
    }

}