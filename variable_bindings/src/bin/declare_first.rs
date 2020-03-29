fn main() {
    //declare a variable binding
    let a_binding;
    {
        let x = 2;

        //initialize the binding
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;

    //error! use of uninitialized binding
    // println!("another binding: {}", another_binding);
    //FIXME ^ comment out this line

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
