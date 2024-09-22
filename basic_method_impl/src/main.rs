#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect0 = Rectangle {
        width: 25,
        height: 25,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };

    println!("Can rect0 hold rect1? -> {}", rect0.can_hold(&rect1));
    println!("Can rect0 hold rect2? -> {}", rect0.can_hold(&rect2));

    let rect3 = Rectangle::square(100);
    dbg!("Rect3 is {}", &rect3);
    // or
    println!("Rect3 is {rect3:#?}");



}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}