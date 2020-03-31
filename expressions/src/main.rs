#[allow(
    path_statements,
    unused_must_use,
    clippy::no_effect,
    clippy::let_unit_value
)]
fn main() {
    //statement: variable binding
    let x = 5;

    //statement: expression;
    x;
    x + 1;
    15;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        //this expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        //the semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
