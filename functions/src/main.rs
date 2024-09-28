fn main() {
    println!("Hello, world!");

    print_labeled_measurement(2, 'h');

    // scope block is an expression
    let y = {
        let x = 3; // statment. However, 3 itself is an expression
        x + 1 // no semicolon to make this line an expression
    };

    println!("value of scope block y: {:#?}", y); // statment

    let x = plus_one(5);
    println!("value of x is {}", x);

}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon to make this line an expression
}
