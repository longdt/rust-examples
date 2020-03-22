use std::mem;

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    //init a `Point`
    let point = Point { x: 10.3, y: 0.4 };

    //access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    //make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point };

    //`bottom_right.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    //destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    println!("rectangle has area: {}", rectangle.area());
    println!("rectangle has area: {}", rect_area(&rectangle));

    let nil1 = Nil;
    let nil2 = Nil;
    println!(
        "1 {0:?} has address {0:p} and size {1} while Nil has size: {2}",
        &nil1,
        mem::size_of_val(&nil1),
        mem::size_of::<Nil>()
    );
    println!(
        "2 {0:?} has address {0:p} and size {1} while Nil has size: {2}",
        &nil2,
        mem::size_of_val(&nil2),
        mem::size_of::<Nil>()
    );

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let bottom_left = Point { x: 2.5, y: 6.3 };
    println!("square: {:?}", square(bottom_left, 30f32));
}

#[derive(Debug)]
struct Person<'a> {
    //the 'a defines a lifetime
    name: &'a str,
    age: u8,
}

//A unit struct
#[derive(Debug)]
struct Nil;

//A tuple struct
struct Pair(i32, f32);

//A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

//structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        ((self.top_left.x - self.bottom_right.x) * (self.top_left.y - self.bottom_right.y)).abs()
    }
}

fn rect_area(
    Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right:
            Point {
                x: bottom_right_x,
                y: bottom_right_y,
            },
    }: &Rectangle,
) -> f32 {
    ((top_left_x - bottom_right_x) * (top_left_y - bottom_right_y)).abs()
}

fn square(bottom_left: Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            y: bottom_left.y + size,
            ..bottom_left
        },
        bottom_right: Point {
            x: bottom_left.x + size,
            ..bottom_left
        },
    }
}
