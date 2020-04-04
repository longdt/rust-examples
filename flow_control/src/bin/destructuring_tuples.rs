fn main() {
    let pair = (0, -2);
    //TODO ^ try different values for `pair`

    println!("Tell me about {:?}", pair);
    //match can be used to destructure a tuple
    match pair {
        //destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
        //`_` mean dont' bind the value to a variable
    }
}
