#![crate_name = "documentation"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Creates a person with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will event test it for you!
    /// use documentation::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Hello, {}", self.name);
    }
}

pub use bar::Bar;

#[doc(inline)]
pub mod bar {
    /// The docs for Bar
    pub struct Bar;
}

// Example from libcore/prelude
#[doc(no_inline)]
pub use std::mem::drop;

// Example from the futures-rs library
#[doc(hidden)]
pub use futures::*;

fn main() {
    let john = Person::new("John");

    john.hello();
}
