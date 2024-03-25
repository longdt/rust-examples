struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// trait objects must include the `dyn` keyword
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn random_ref_animal(random_number: f64) -> &'static dyn Animal {
    if random_number > 0.5 {
        Box::leak(Box::new(Sheep {}))
    } else {
        Box::leak(Box::new(Cow {}))
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
    let ref_animal = random_ref_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", ref_animal.noise());
}
