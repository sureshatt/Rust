fn main() {
    println!("Hello, world!");

    let s = String::from("Hello");
    take_ownership(s); // giving ownership to the function, so the variable 's' looses it
    // println!("String s:{}",s); // this results in an error because the variable S no longer has ownership (reference)
    
    let x = 5;
    make_copy(x); // passing the value to the function triggers a copy
    println!("x: {}", x);
    
    let s1 = give_ownership();
    let s2 = String::from("Hallo"); 
    let s3 = takes_and_gives_back(s2); // ownership for s2 is lost, hence droped

    println!("s1:{},s3:{}", s1, s3);

}

fn take_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string // with this return, the variable some_string looses the ownership
}

fn takes_and_gives_back(a_string: String) -> String { // get the ownership here
    a_string // looses the ownership here
}