fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// Everything in lifetime 'a so ok
// fn main() {
    // let long = String::from("xyz");
    // let result;    
    // let longer = String::from("abcd");
    
    // result = longest(&longer.as_str(), &long.as_str());
    // println!("{result}");
// }


// Still valid since everuthing is included in 'a lifetime
// fn main() {
//     let long = String::from("xyz"); 
//     let result;
//     {
//         let longer = String::from("abcd");
//         result = longest(&longer.as_str(), &long.as_str());
//         println!("{result}");
//     }
// }


// NOK, 'longer' lifetime ends before result is
fn main() {
    let long = String::from("xyz");
    let result;

    {
        let longer = String::from("abcd");
        result = longest(&longer.as_str(), &long.as_str());
    }
    println!("{result}");
}
