pub enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    // TODO - fix error 
    //     error[E0308]: mismatched types
    //    --> enum_option.rs:11:38
    //     |
    // 11  |     let absent_number: Option<i32> = None;
    //     |                        -----------   ^^^^ expected `Option<i32>`, found `Option<_>`
    //     |                        |
    //     |                        expected due to this
    //     |
    //     = note: `std::option::Option<_>` and `Option<i32>` have similar names, but are actually distinct types
    // note: `std::option::Option<_>` is defined in crate `core`
    //    --> /Users/krzysztofbrzozowski/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:574:1
    //     |
    // 574 | pub enum Option<T> {
    //     | ^^^^^^^^^^^^^^^^^^
    // note: `Option<i32>` is defined in the current crate
    //    --> enum_option.rs:1:1
    //     |
    // 1   | enum Option<T> {
    //     | ^^^^^^^^^^^^^^

    // error: aborting due to 1 previous error
    let absent_number: Option<i32> = None;
}