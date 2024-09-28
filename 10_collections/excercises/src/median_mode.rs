use std::collections::HashMap;

pub fn median(list: &Vec<i32>) -> Option<i32> {
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

pub fn mode(list: &Vec<i32>) -> Option<i32> {
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