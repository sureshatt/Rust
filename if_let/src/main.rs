fn main() {
    let temp = Some(50);

    if let Some(val) = temp {
        println!("Value is: {}", val);
    } else {
        println!("Value not defined");
    }
}
