#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetime to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
