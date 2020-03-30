use std::mem;

#[allow(clippy::many_single_char_names)]
fn main() {
    //suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    //unsuffixed literal, their type depend on how they are used
    let i = 1;
    let f = 1.0;

    //`size_of_val` return the size of a variable in bytes
    println!("size of `x` in bytes: {}", mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", mem::size_of_val(&f));
}
