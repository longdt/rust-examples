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

    //right-align text with specified width. Output: "     1". 5 white spaces and "1"
    println!("{number:>width$}", number = 1, width = 6);

    //right-align text with pad. Output: "000001"
    println!("{number:>06}", number = 1);

    //Rust check correct number of argument are used.
    // println!("My name is {0}, {1} {0}", "Bond");
    //FIXME ^ Add missing argument

    #[allow(dead_code)]
    struct AStructure;

    //Custom type need implement Display trait
    // println!("This struct `{}` won't print...", AStructure);
    //FIXME ^ impl Display for AStructure

    //println pi in format 3.14
    #[allow(clippy::approx_constant)]
    let pi = 3.141_592;
    println!("PI = `{:10.2}`", pi);
    println!("Hello `{:10}`", "x");

    //can't use $ in first part of {part1:part2}
    // println!("{number_idx$:width$}", number_idx=0, width=10);
}
