use std::num::ParseIntError;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}

fn main() {
    let error = DoubleError::EmptyVec;
    let other_error = error;
    // println!("original error: {:?}", error);
    match other_error {
        DoubleError::EmptyVec => println!("EmptyVec"),
        DoubleError::Parse(_) => println!("Parse")
    }
    println!("other error: {:?}", other_error);
    let new_other_error = other_error;
    println!("new other error: {:?}", new_other_error);
}
