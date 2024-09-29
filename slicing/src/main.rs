fn main() {
    let mut s = String::from("Hello world");
    
    // providing a immutable reference to s which goes out of scope after returning from the function
    // however, the fuction returns a &str which is another immutable referece to the 's'
    let first_word: &str = find_first_word(&s); 

    //s.clear(); // this is an error because you cannot borrow as an mutable while the immutable reference (first_word) still exists
  
    println!("first word: {}", first_word);

    s.clear(); // this mutation is possible because the immutable reference has ended at println

    // ------ understanding slices further 
    let s1: String = String::from("Hello world"); // a String
    let s2: &str = &s1[0..4]; // a slice of s1 which is a reference

    // same function, but works for &String and &str
    let _first_word_str1 = find_first_word_str(&s1); // providing the &String type reference
    let _first_word_str2 = find_first_word_str(s2); // providing the &str reference
}

fn find_first_word(s: &String) -> &str {
    let byte_array = s.as_bytes();

    for (i, &byte_item) in byte_array.iter().enumerate() {
        if byte_item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

// instead of getting &String, it is better use &str because a &str can refer both &str and &String 
fn find_first_word_str(s: &str) -> &str {
    let byte_array = s.as_bytes();

    for (i, &byte_item) in byte_array.iter().enumerate() {
        if byte_item == b' ' {
            return &s[0..i]; 
        }
    }
    return &s;
}