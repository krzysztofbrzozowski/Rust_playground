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

## structs
Basic implementation and example how strcts implementation looks like.
Inside you can also see example of
```
#[derive(Debug)]
```

## basic_method_impl
Intro to OOP, basic implementation of methods in Rust

## enums_and_pattern_matching
- enum.rs => basic usage of enums in Rust
- enum_option.rs => Polular Rust pattern to cover Null vallue
- enum_match.rs => enum + matching (arm pattern)
- enum_match_pattern.rs => enum + matching + additional pattern find in another enum
- match_pattern_other_or_ => enum + matching all other values out of defined pattern
- if_let.rs => if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values

## 09_packages_crates_and_modules
### backyard
Example implementation of external modules within one crate, binary one, structure looks like

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
### restaurant
Basically demos for
- Modules in the same lib
- Public enum
- Super call
- Private field
```
crate
 └── front_of_house
 │   ├── hosting
 │   │   ├── add_to_waitlist
 │   │   └── seat_at_table
 │   └── serving
 │       ├── take_order
 │       ├── serve_order
 │       └── take_payment
 └── back_of_house
     ├── deliver_order
     └── back_of_house
         ├── fix_incorrect_order
         └── cook_order
    ...
    +
    Some other implementations

```
This tree shows how some of the modules nest inside other modules; for example, hosting nests inside front_of_house. The tree also shows that some modules are siblings, meaning they’re defined in the same module; hosting and serving are siblings defined within front_of_house. If module A is contained inside module B, we say that module A is the child of module B and that module B is the parent of module A. Notice that the entire module tree is rooted under the implicit module named crate.

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

Build with
```
cargo build
```




