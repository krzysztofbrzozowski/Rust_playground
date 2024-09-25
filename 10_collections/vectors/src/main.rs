fn sample_collection() {
    // Defining a vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // or
    let mut some_vec = vec![1, 3, 6];
    some_vec.push(10);


    // Get element
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterate over vector
    for i in &v {
        println!("Item in vect {}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("Item in vect + 50 = {}", *i);
    }

    for i in v {
        println!("Item in vect + 50 = {}", i);
    }
}

fn sample_vect_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn main() {
    sample_collection();
    sample_vect_enum()
}
