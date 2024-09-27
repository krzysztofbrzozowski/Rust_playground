use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector and return 
    // the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    // 1. Median
    let mut list = vec![1, 5, 12, 8, 5, 9, 14, 88];
    list.sort();

    let idx = list.len() / 2;

    println!("Sorted list is {list:#?}");

    match list.len() % 2 {
        // If devide is even number
        0 => println!("median is {}", 
                &list[idx - 1] + &list[idx]),

        1 => println!("median is {}", 
                &list[idx]),

        _ => {},

    }

    // 2. Mode
    let mut map = HashMap::new();
    for item in list {
        let num = map.entry(item).or_insert(0);
        *num += 1;
    }
    println!("Map is {map:#?}");
    // TODO - do not understand, copied that from stack
    let max_val = map.iter().max_by_key(|entry | entry.1).unwrap();
    println!("Max occurance has {} -> {}", max_val.0, max_val.1);
}
