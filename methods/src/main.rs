#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    // associated functions --> similar to constructors to create objects
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// proving multiple impl possible
impl Rectangle {
    fn is_squre(&self) -> bool {
        self.width == self.height
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let square = Rectangle::square(10);

    println!("Area: {}", rect1.area());
    println!("Can hold?: {}", rect1.can_hold(&rect2));
    println!("Can hold?: {}", square.can_hold(&rect2));
    
    let rect3 = Rectangle {
        width:5,
        height: 5,
    };
    
    println!("is squre?: {}", rect3.is_squre());

}
