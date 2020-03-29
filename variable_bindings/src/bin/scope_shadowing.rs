fn main() {
    //this binding lives in the main function
    let long_lived_binding = 1;
    //this is a block, and has a smaller scope than the main function
    {
        //this binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        //this binding *shadows* the outer one
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    //end of the block

    //error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    //FIXME ^ comment out this line

    println!("outer long: {}", long_lived_binding);

    //this binding also *shadows* the previous binding
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}
