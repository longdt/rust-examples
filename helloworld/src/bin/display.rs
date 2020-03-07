use std::fmt::{Display, Formatter, Debug};
use std::fmt::Result;

fn main() {
    let iw = IntWrapper(10);
    println!("{}", iw);

    let point = Point2D {
        x: 3.3,
        y: 7.2
    };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let list = List(vec![3, 4, 10, 100]);
    println!("{}", list);
}

struct IntWrapper(i32);

impl Display for IntWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

struct Point2D {
    x: f64,
    y: f64
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

impl Debug for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("[")?;
        let mut iter = self.0.iter().enumerate();
        if let Some((_, v)) = iter.next() {
            write!(f, "0: {}", v)?;
            for (idx, v) in iter {
                write!(f, ", {}: {}", idx, v)?;
            }
        }
        f.write_str("]")
    }
}