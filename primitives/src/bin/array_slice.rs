use std::fmt::Display;
use std::mem;

fn main() {
    //fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    //all element can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    //indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // array.len() to return size of array (array length)
    println!("array size: {}", xs.len());

    //arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    //arrays can be automatically borrowed as slices
    println!("borrow the whole array as slice");
    analyze_slice(&xs);

    //slices can point to a section of an array
    //they are of form [start_index..end_index]
    //start_index: first position in slice
    //end_index: is one more than the last position in the slice (excluded)
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    //out of bound indexing cause compile error
    // println!("{}", xs[5]);
}

fn analyze_slice(slice: &[impl Display]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
