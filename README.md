# Playground for RUST language
A lot of exapmples are inpired by Rust book -> https://doc.rust-lang.org/book

## hello dir
hello world for manually created

Build && run
```
rustc hello.rs
./hello
```

## hello_cargo dir
cargo created package


Created using
```
cargo new hello_cargo
```

Build && run
```
cargo build
./target/debug/hello_cargo
```
or to compile the code and then run the resultant executable all in one command
```
cargo run
```

## guessing game
simple game described https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## ownership_references_borrowing
Interesting vanilla Rust point of existence in few exaples -> ownership in a scope, borrowing (basically referencing)

## slice type
Continue of ownership_references_borrowing but in real life example, how borrowing concept is raising an error during compilation

```
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.
```
