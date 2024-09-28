fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("hello");
    let len = calculate_length(&s1); // borrowing / pass by reference
    println!("The length of 's1': {} is: {}", s1, len);
    
    change(&mut s1);
    println!("Changed is s1: {}", s1);
    
    let mut t1 = String::from("Hallo");
    let t2 = &mut t1;
    //let t3 = &mut t1; // this will not work!/ only one mutable reference is allowed
    // println!("t1:{}", t1); this will not work! t2 has already mutabality borrowed
    println!("t2:{}", t2);


    // -------- mutable reference vs immutable reference 
    let a = String::from("immutable reference example");
    let _b = & a; // a immutable reference allowed here
    let _c = & a; // a immutable reference allowed here as well
    let _d = & a; // a immutable reference allowed here as well

    let mut x = String::from("mutable reference example");
    let y = &mut x; // a mutable reference allowed here
    // let z = &mut x; // will not work
    // println!("{}",z); 
    println!("{}", y);
    
}

fn calculate_length(s :&String) -> usize {
    // s.push_str("!"); // this is still a reference, cannot modify the data as it does not own it
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("!!");
}