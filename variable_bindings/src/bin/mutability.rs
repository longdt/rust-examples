fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    //ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //error!
    // _immutable_binding += 1;
    //FIXME ^ comment out this line
}
