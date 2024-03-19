struct Container(i32, i32);
// A trait which checks if 2 items are stored inside of container. Also retrieves first or last value
// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for aliases).
trait Contains {
    // Define generic types here with methods will be able to utilize.
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type is `Container(i32, i32)`, the `output`
    // types are determined as `i32` and `i32`
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// Without using associated types
// fn difference<A, B, C>(container: &C) -> i32 where
//     C: Contains<A, B> {
// }

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}
fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
