use std::{
    fs::File,
    io::{ErrorKind, Read},
};
fn main() {
    // panic!("Crash & burn"); // this is an explicit panic

    let v = vec![1, 2, 3];
    println!("value at 1 is: {}", v[1]);
    // println!("value at 10 is: {}", v[10]);    // a forced panic by the program

    let greeting_file_result = File::open("hello.txt");
    let mut buf = String::new();

    // hard to read code. nested match and also match does not provide much control. See the Result<T,E> approach instead
    let mut geering_file = match greeting_file_result {
        Ok(file) => {
            println!("file found. reading..");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("file not found. creating..");
                match File::create("hello.txt") {
                    Ok(ok) => {
                        println!("file not found. creating..");
                        ok
                    }
                    Err(error2) => panic!("error: {:?}, {:?}", error, error2),
                }
            }
            _ => {
                panic!("error: {:?}, ", error)
            }
        },
    };

    // easy to read and also provide control over the code flow
    if let Ok(data) = geering_file.read_to_string(&mut buf) {
        println!("Data is: {}", data);
    } else {
        println!("could not read data");
    }
}
