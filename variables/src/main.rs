fn main() {
    println!("Variables");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // will never be mutable
    
    let mut x = 5; // making x mutable
    println!("The value of x: {}", x);
    
    {
        // shadowing
        let x = x + 1;
        println!("The value of x: {}", x);
        
    }
    
    x = 7; // this is mutable
    
    println!("The value of x: {}", x);
    println!("The value of const: {}", THREE_HOURS_IN_SECONDS);
    
    let _guess: u32 = "42".parse().expect("Not a number"); // type needs to be know before calling parse()
    
    println!("The Max value: {} and Min value: {}", u8::MAX, u8::MIN); 
    println!("The Max value: {} and Min value: {}", isize::MAX, isize::MIN);
    
    // different ways to write an integer
    println!("Byte value: {}, Binary value: {}, Hex value: {}, Octal value: {}", b'A', 0b11110000, 0xff, 0o77);
    
    let a:i8 = 10;
    let b:i8 = 3;
    let c:i8 = a / b;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // accessing tuple data
    let tup = (500, 1.1, 'b');
    println!("tuple 1th: {}", tup.0);

    // arrays
    let aar:[i32;3] = [2,3,4]; // initializing 
    let bar = [6,7,8,9];

    println!("a1:{} & b2:{}", aar[0], bar[0]);
}
