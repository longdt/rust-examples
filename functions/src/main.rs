//unlike C/C++, there's no restriction on the order of function definitions
fn main() {
    //we can use this function here, and define it somewhere later
    fizzbuzz_to(100);
}

//function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    //corner case, early return
    if rhs == 0 {
        return false;
    }

    //this is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

//functions that "don't" return a value, actually return the unit type `()`
#[allow(clippy::unused_unit)]
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz")
    } else if is_divisible_by(n, 5) {
        println!("buzz")
    } else {
        println!("{}", n);
    }
}

//when a function returns `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
