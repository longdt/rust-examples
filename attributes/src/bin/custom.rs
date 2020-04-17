//try to compile with `rustc --cfg some_condition custom.rs`
fn main() {
    conditional_function();
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

#[cfg(not(some_condition))]
fn conditional_function() {
    println!("condition not met!");
}
