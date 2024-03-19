use std::{env, thread};
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{:?}: out dir = {:?}", thread::current().id(), out_dir);
}
