use std::fs::{File, read_to_string};
use std::io;
use std::io::BufRead;
use std::path::Path;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn read_lines_v1(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()   // panic on possible file-reading errors
        .lines()             // split the string into an iterator of string lines
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

// The output is wrapped in a Result to allow matching on errors. Returns an Iterator to the Reader
// of the lines of the file.
fn read_lines_v2<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines_v2("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}
