#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

fn main() {
    let react_obj = Rectangle {
        // We can put dbg! around the expression 20 * 2 and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg!
        width:  dbg!(20 * 2),
        height: 30,
    };

    //     error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    //   --> src/main.rs:12:33
    //    |
    // 12 |     println!("react_obj is {}", react_obj);
    //    |                                 ^^^^^^^^^ `Rectangle` cannot be formatted with the default formatter
    //    |
    //    = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    //    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // (uncomment this to see error) -> // println!("react_obj is {}", react_obj);

    // Adding some debug -> Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. 
    // The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code
    // (uncomment this to see print output) -> // println!("react_obj is {react_obj:#?}");
    dbg!(&react_obj);


    println!(
        "Rectangle area = {}", 
        area(&react_obj)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}