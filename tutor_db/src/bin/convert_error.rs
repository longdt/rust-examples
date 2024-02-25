use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;

fn main() {
    println!("{:?}", square("2"));
    println!("{:?}", square("INVALID"));

    println!("{:?}", square_question_mark("2"));
    println!("{:?}", square_question_mark("INVALID"));
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
    let mut f = File::open("fictional_file.txt")?;
    let file_content = format!("square of {} is {}", num, num.pow(2));
    f.write_all(file_content.as_bytes())?;
    Ok(num.pow(2))
}
