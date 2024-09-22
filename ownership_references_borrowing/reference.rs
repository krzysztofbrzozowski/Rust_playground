// Reference and borowing in simple exmple
fn main() {
    let mut s = String::from("hello");

    // We call the action of creating a reference borrowing
    // As in real life, if a person owns something, you can borrow it from them
    // When you’re done, you have to give it back. You don’t own it
    change(&mut s);
    mutable_reference_error();
    mutable_reference_ok();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mutable_reference_error() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM (see the slice_type example to see that in function)
    // We can not have regular reference and mutable one to the same value
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!

    // println!("{}, {}, and {}", r1, r2, r3);
}

fn mutable_reference_ok() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn dangle() -> &String {
    // Here we have an error since reference will be passed out of the scope
    // Because s is created inside dangle, when the code of dangle is finished, s will be deallocated
    // But we tried to return a reference to it
    // That means this reference would be pointing to an invalid String
    // That’s no good! Rust won’t let us do this
    let s = String::from("hello");
    // error[E0515]: cannot return reference to local variable `s`
    //   --> reference.rs:49:5
    //    |
    // 49 |     &s
    //    |     ^^ returns a reference to data owned by the current function

    // error: aborting due to 2 previous errors; 3 warnings emitted
    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
