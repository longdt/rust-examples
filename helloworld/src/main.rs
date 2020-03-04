fn main() {
    //default 31 becomes an i32
    println!("{} days", 31);

    //position of arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //special formatting can be specified after a `:`.
    println!("{0} in {0:b} binary format", 2);
}
