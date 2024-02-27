use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;

use crate::MyError::{IOError, ParseError};

#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Parse Error"),
            MyError::IOError => write!(f, "IO Error"),
        }
    }
}

impl std::error::Error for MyError {}

fn main() {
    println!("{:?}", square("2"));
    println!("{:?}", square("INVALID"));

    println!("{:?}", square_question_mark("2"));
    println!("{:?}", square_question_mark("INVALID"));

    println!("{:?}", square_new("2"));
    println!("{:?}", square_new("INVALID"));
}

fn square(value: &str) -> Result<i32, ParseIntError> {
    match value.parse::<i32>() {
        Ok(num) => Ok(num.pow(2)),
        Err(e) => Err(e),
    }
}

fn square_question_mark(value: &str) -> Result<i32, ParseIntError> {
    let num: i32 = value.parse()?;
    Ok(num.pow(2))
}

fn square_to_file(value: &str) -> Result<i32, ParseIntError> {
    let num: i32 = value.parse()?;
    // error[E0277]: `?` couldn't convert the error to `ParseIntError`
    // the trait `From<std::io::Error>` is not implemented for `ParseIntError`
    // let mut f = File::create("fictional_file.txt")?;
    // let file_content = format!("square of {} is {}", num, num.pow(2));
    // f.write_all(file_content.as_bytes())?;
    Ok(num.pow(2))
}

fn square_new(value: &str) -> Result<i32, MyError> {
    let num: i32 = value.parse()
        .map_err(|_| ParseError)?;
    let mut f = File::create("fictional_file.txt")
        .map_err(|_| IOError)?;
    let file_content = format!("square of {} is {}", num, num.pow(2));
    f.write_all(file_content.as_bytes())
        .map_err(|_| IOError)?;
    Ok(num.pow(2))
}
