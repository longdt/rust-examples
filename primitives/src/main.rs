#[allow(unused_variables, unused_assignments)]
fn main() {
    // type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // default type
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    //A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from below line
    inferred_type = 123_123_345_345_345_i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12;
    mutable = 21;

    // error! type of variable can't be changed.
    // mutable = true;

    // variables can be shadow.
    let mutable = true;
}
