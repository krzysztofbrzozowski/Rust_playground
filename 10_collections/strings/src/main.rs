fn main() {
    // Some interesting feature 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Above is because 
    // fn add(self, s: &str) -> String {

    // Sample usage of adding
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // or more clear
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Some characters can use 2 bytes
    let hello = "ŁÓDŹ";
    let s = &hello[0..4];
    println!("{s}");
    // ŁÓ
}
