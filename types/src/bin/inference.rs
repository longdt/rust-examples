fn main() {
    //because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    //create an empty vector (a growable array)
    let mut vec = Vec::new();
    //at this point the compiler doesn't know the exact type of `vec`, it just know that it's a vector
    //of something (`Vec<_>`).

    //insert `elem` in the vector.
    vec.push(elem);
    //aha! now compiler know that `vec` is a vector of `u8`s (`Vec<u8>`)
    //TODO ^ try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
