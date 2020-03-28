fn main() {
    let n = 16;
    //access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    let num = Number(0);
    println!("is zero: {}", num.is_zero());

    //error! can't modify a `const`.
    // THRESHOLD = 5;
}

//Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    //access constant in some function
    n > THRESHOLD
}

struct Number(i32);

impl Number {
    const ZERO: i32 = 0;

    fn is_zero(&self) -> bool {
        self.0 == Number::ZERO
    }
}
