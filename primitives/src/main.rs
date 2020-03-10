fn main() {
    // type annotated
    let _logical: bool = true;

    let _a_float: f64 = 1.0;      // regular annotation
    let _an_integer = 5i32;  // suffix annotation

    // default type
    let _default_float = 3.0; // `f64`
    let _default_integer = 7; // `i32`

    //A type can also be inferred from context
    let mut _inferred_type = 12; // Type i64 is inferred from below line
    _inferred_type = 123123345345345i64;

    // A mutable variable's value can be changed.
    let mut _mutable = 12;
    _mutable = 21;

    // error! type of variable can't be changed.
    // mutable = true;

    // variables can be shadow.
    let _mutable = true;
}
