fn main() {
    let greeting = "hello";
    //a non-copy type.
    //`to_owned` creates owned data from borrowed one
    let mut farewell = "goodbyte".to_owned();

    //capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        //`greeting` is by reference: requires `Fn`.
        println!("I said {}", greeting);

        //mutation forces `farewell` to be captured by mutable reference. Now requires `FnMut`
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzzz");

        //manually calling drop forces `farewell` to be captured by value. Now requires `FnOnce`
        drop(farewell);
    };

    //call the function which applies the closure.
    apply(diary);

    //`double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

//a function which takes a closure as an argument and call it.
//<F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    //the closure thakes no input and returns nothing.
    F: FnOnce(),
{
    //TODO: ^ try changing this to `Fn` or `FnMut`
    f();
}

//a function which takes a closure and returns an `i32`.
fn apply_to_3(f: impl Fn(i32) -> i32) -> i32 {
    f(3)
}
