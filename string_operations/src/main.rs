fn main() {
   
   let mut s0 = String::new();

   println!("{s0}");
   
   let s1 = String::from("Hello world");
   let s2 = "Good morning!".to_string();
   
   s0.push_str(&s1);
   println!("{s0}");

   println!("s1: {}", s1);
   println!("s2: {}", s2);
   
   let s3 = s2 + &s1; // s2 is moved to s3. 
   println!("s3: {}", s3);
   
   let s4 = format!("{s0}-{s1}-{s3}");
   println!("s4: {}", s4);

    //Strings are stores as Vectors in memory where each UTF-8 char is encoded into two bytes. Therefore iterating bytes will not give the char
    let s5 = "abcß";

    for c in s5.chars() {
        println!("{c}");
    }

    for b in s5.bytes() {
        println!("{b}");
    }


}
