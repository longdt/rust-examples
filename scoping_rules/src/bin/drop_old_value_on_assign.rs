struct Person {
    name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Drop person: {}", self.name);
    }
}


fn main() {
    let mut p = Person {
        name: String::from("ALong")
    };
    println!("Before assign");
    // old value of `p` will be drop before assign new value
    p = Person {
        name: String::from("New Person")
    };
    println!("After assign");
}
