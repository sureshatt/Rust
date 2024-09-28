fn main() {
    println!("Hello, world!");

    // ------------- copy behaviour 
    let x = 1;
    let y = x;
    println!("x: {}, y: {}",x,y);

    // ------------- move / drop behaviour
    let s = String::from("hello");
    // let t = s; this will make the s to be 'drop'ed/'move'ed
    let t = s.clone(); 
    println!("s: {}, t:{}", s, t);

    // compond variables
    let mut array1 = [1,2,3,4];
    let array2 = array1; // this has the 'copy' triat

    for item in array1 {
        println!("item: {}", item);
    }

    for item in array2 {
        println!("item: {}", item);
    }

    array1[0]=9; // mutating array1 does not affect array2

    for item in array1 {
        println!("item: {}", item);
    }

    for item in array2 {
        println!("item: {}", item);
    }
}
