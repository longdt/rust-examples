//an attribute to hide warnings for unused code.
#![allow(dead_code, clippy::unreadable_literal)]
fn main() {
    //`enums` can be cast as integers.
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("{:?} are #{:06x}", Color::Red, Color::Red as i32);
    println!("{:?} are #{:06x}", Color::Blue, Color::Blue as i32);
}

//enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

//enum with explicit discriminator
#[derive(Debug)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
