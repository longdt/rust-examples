fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess"),
        //could `match` 1..=12 directly but then what age would the child be? Instead, bind to `n`
        //for the sequence of 1..=12. Not the age can be reported
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        //nothing bound. Return the result.
        n => println!("I'm an old persion of age {:?}", n),
    }

    match some_number() {
        //got `Some` variant, match if its value, bound to `n`, is equal to 42
        Some(n @ 42) => println!("The answer: {}!", n),
        //match any other number.
        Some(n) => println!("Not interesting... {}", n),
        //match anything else (`None` variant).
        _ => (),
    }
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}
