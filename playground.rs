fn main() {
    let mut x = 5;


    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    x = x + x + x;
    println!("The value of x is: {x}");
}