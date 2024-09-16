// warning: unused variable: `s2`
//   --> ownership.rs:36:9
//    |
// 36 |     let s2 = s1;
//    |         ^^ help: if this is intentional, prefix it with an underscore: `_s2`
//    |
//    = note: `#[warn(unused_variables)]` on by default

// error[E0382]: borrow of moved value: `s1`
//   --> ownership.rs:38:15
//    |
// 35 |     let s1 = String::from("hello");
//    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 36 |     let s2 = s1;
//    |              -- value moved here
// 37 |
// 38 |     println!("{s1}, world!");
//    |               ^^^^ value borrowed here after move
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 36 |     let s2 = s1.clone();
//    |                ++++++++

// error: aborting due to 1 previous error; 1 warning emitted
// 
// ownership_example not working because
// Error is raised due to string literal is created on a heap due to use ::from
// Since String::from("hello") is assigned to s1 we the ownershop has been moved to s1
// Another try of use is causing an error
// 
// ownership_example_with_clone working because
// string literal is implicitly copied on heap 
// bcause of method 'clone'
// 
// Below you can find something interesting example with stack (i32) and heap (String) 
// fn main_run_second_exaple

fn main() {
    ownership_example();
    ownership_example_with_clone();
    main_run_second_exaple();
}

fn ownership_example () {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
}

fn ownership_example_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

// Here is some other interesting example with stack (i32) and heap (String)
fn main_run_second_exaple() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.