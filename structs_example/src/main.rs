#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Square {
    length: u32,
    color: String,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    
    dbg!(&rect1);
    println!("Rectangle: {:#?}", rect1);
    println!("Rectangle area: {}", area(&rect1));
    
    let square = Square {
        length: dbg!(30 * 2), // debug the expression which results in 60 = 30 * 2 logged
        color: String::from("red"),
    };

    dbg!(&square);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
