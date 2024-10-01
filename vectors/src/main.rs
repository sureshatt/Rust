fn main() {
    let mut v = vec![1,2,3];

    for (i, ele) in v.iter().enumerate() {
        println!("{} element is: {}", i, ele);
    }

    v.push(5);
    v.push(6);
    v.push(4);

    for (i, ele) in v.iter().enumerate() {
        println!("{} element is: {}", i, ele);
    }
    
    println!("value at 2nd: {}", &v[1]);
    
    for (i, ele) in v.iter().enumerate() {
        println!("{} element is: {}", i, ele);
    }



    match v.get(1) {
        Some(val) => println!("Value at 1 is: {}", val),
        None => println!("No value found"),
    }

    for (i, ele) in v.iter().enumerate() {
        println!("{} element is: {}", i, ele);
    }

    let third = &v[2];
    // v.push(5); // Not allowed, cannot have a mutation with a immutable borrow 
    println!("third: {}", third);


    //another way to iterate
    for item in &mut v {
        *item += 1; // dereferencing 
    }

}
