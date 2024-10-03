use std::{fmt::Error, fs::{self, File}, io::{self, Read}};
fn main() {

    // let greeting_file = File::open("hello.txt").unwrap(); // this will call panic! internally for us

    // bellow allows to define the error message
    //let greeting_file2 = File::open("hello.txt").expect("Error while opening the file");

    read_username_file();
}

fn read_username_file() -> Result<String, io::Error> {

    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// the '?' operator works only when Result<T,E> is returned from a function
fn read_username_file_2() -> Result<String, io::Error> {

    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// the '?' operator works only when Result<T,E> is returned from a function
fn read_username_file_3() -> Result<String, io::Error> {

    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// the '?' operator works only when Result<T,E> is returned from a function
fn read_username_file_4() -> Result<String, io::Error> {
        fs::read_to_string("username.txt")
}