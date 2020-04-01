#![allow(
    unreachable_code,
    unused_labels,
    clippy::never_loop,
    clippy::unused_label
)]
fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            //this would break only the inner loop
            // break;

            //this breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
