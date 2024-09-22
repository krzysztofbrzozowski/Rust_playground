fn main() {
    let mut sentence = String::from("Hello world!");
    let first_word_idx = first_word_idx(&sentence);
    
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally in
    // Instead of working with idx, better return slice type -> see the function first word
    // let hello = &sentence[0..first_word_idx];
    // (uncomment this to see error) -> // println!("First word (with idx) {hello}");

    // We now have a straightforward API that’s much harder to mess up because the compiler will ensure
    // the references into the String remain valid. 
    // Remember the bug in the program in first_word_idx, when we got the index to the end of the first 
    // word but then cleared the string so our index was invalid? 
    // That code was logically incorrect but didn’t show any immediate errors. 
    // The problems would show up later if we kept trying to use the first word index with an emptied string. 
    // Slices make this bug impossible and let us know we have a problem with our code much sooner. 
    // Using the slice version of first_word will throw a compile-time error

    //     error[E0502]: cannot borrow `sentence` as mutable because it is also borrowed as immutable
    //     --> src/main.rs:14:5
    //      |
    //   11 |     let hello = first_word(&sentence);
    //      |                            --------- immutable borrow occurs here
    //   ...
    //   14 |     sentence.clear();
    //      |     ^^^^^^^^^^^^^^^^ mutable borrow occurs here
    //   15 |
    //   16 |     println!("First word {hello}");
    //      |                          ------- immutable borrow later used here

    let hello = first_word(&sentence);

    // this empties the String, making it equal to ""
    sentence.clear();

    println!("First word {hello}");
}

// Will work but better return slice type - fn first_word
fn first_word_idx(s: &String) -> usize{
    let s_bytes = s.as_bytes();

    for (idx, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return idx;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let s_bytes = s.as_bytes();

    for (idx, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            // Return first word if found ' '
            return &s[0..idx];
        }
    }

    // Else return whole sentence
    &s[..]
}
