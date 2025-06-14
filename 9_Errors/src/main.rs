use std::error::Error;

fn main()  -> Result<(), Box<dyn Error>> {
    let _res = read_username_from_file;
    Ok(())
}

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

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

#[allow(unused)]
fn read_username_from_file_short() -> Result<String, io::Error> {
    // Use ? to propagate errors
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    // OR:
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}