#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y
}

impl <X,Y> Point<X,Y> {

    // the method signature itself also has a type <X2, Y2> defined to indicate it accept a different Point type as 'other'
    fn mix<X2, Y2> (self, other: Point<X2,Y2>) -> Point<X,Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let p1 = Point {x:1, y:2};
    println!("p1 is: {:#?}", p1);
    
    let p2 = Point {x:'a', y:'b'};
    println!("p2 is: {:#?}", p2);
    
    let p3 = p1.mix(p2);
    println!("p3 is: {:#?}", p3);
}
