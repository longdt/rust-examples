#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;
}
