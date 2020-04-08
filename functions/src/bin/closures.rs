fn main() {
    //increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }
    //closures are anonymous, here we are binding them to references
    //annotation is identical to function annotation but is optional as are the `{}` wrapping
    //the body. These nameless functions are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    //call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    //a closure taking no arguments which return an `i32`. The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}
