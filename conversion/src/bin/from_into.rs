fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    //we got free implementation `impl Into<Number> for i32` due blanket implementation
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}
