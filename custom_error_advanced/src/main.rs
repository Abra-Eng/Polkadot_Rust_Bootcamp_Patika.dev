use std::fs;
use std::io;
use std:: num;

fn main() {
    match read_and_parse("test.txt") {
        Ok(num) => println!("Nothing Wrong"),
        Err(err) => println!("Error {}", err),
    }
}

#[derive(Debug)]
enum MyCustomError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Other(String),
}

use std::fmt;

// implementing Display trait for MyCustomError, this allows us to print out our error messages
// in a human-readable format
impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f," I/O Error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse Error: {}", err),
            MyCustomError::Other(message) => write!(f, "Other Error: {}", message),
        }
    }
}
// implementing Error trait for out Custom error
// This makes our custom error type an "error" in the eyes of Rust 
impl std::error::Error for MyCustomError {}

fn read_and_parse(filename: &str) -> Result<i32, MyCustomError> {
    let content = fs::read_to_string(filename).map_err(MyCustomError::Io)?;
    let num: i32 = content.trim().parse().map_err(MyCustomError::Parse)?;
    Ok(num)
}
