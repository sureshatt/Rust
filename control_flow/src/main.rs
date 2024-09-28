fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 { 
        println!("condition was true");
    } else if number != 0 {
        println!("not zero");
    } else {
        println!("condition was false");
    }

    // if block is an expression
    let y = if number > 2 {"big"} else {"small"};
    println!("y: {}", y);
}

    
