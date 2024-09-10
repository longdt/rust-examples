#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new() -> Self {
        Person {
            name: "".to_string(),
            age: 0,
        }
    }

    fn with_name<T: ToString>(self, name: T) -> Self {
        Self {
            name: name.to_string(),
            ..self
        }
    }

    fn with_age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }
}

fn main() {
    let person = Person::new();
    println!("{:?}", person);
    let person = person.with_name("hello");
    println!("{:?}", person);
    let p = person.with_age(10);
    println!("{:?}", p);
}