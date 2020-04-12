fn main() {
    let color = String::from("green");

    //a closure to print `color` which immediately borrow (`&`) `color` and stores the borrow and
    //closure in the `print` variable. It will remain borrowed until `print` is used the last time.
    //
    //`println` only requires arguments by immutable reference so it doesn't impose anything more
    //restrictive.
    let print = || println!("`color`: {}", color);

    //call the closure using the borrow.
    print();

    //`color` can be borrowed immutably again, because the closure only holds an immutable reference
    //to `color`.
    let _reborrow = &color;
    print();

    //a move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    //a closure to increment `count` could take either `&mut count` or `count` but `&mut count` is
    //less restrictive so it takes that. Immediately borrows `count`.
    //
    //a `mut` is required on `inc` because a `&mut` is stored inside. Thus, calling the closure
    //mutates the closure which requires a `mut`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    //TODO ^ try removing mut which cause error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable
    inc();

    //the closure still mutably borrows `count` because it is called later. An attemp to reborrow
    //will lead to an error.
    // let _reborrow = &count;
    //TODO: ^ try uncommenting this line.
    inc();

    //the closure no longer need to borrow `&mut count`. Therefore, it is possible to reborrow
    //without an error
    let _count_reborrowed = &mut count;

    //a non-copy type.
    let movable = Box::new(3);

    //`mem::drop` requires `T` so this must take by value. A copy type would copy into the closure
    //leaving the original untouched. A non-copy must move and so `movable` immediately move into
    //the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        drop(movable);
    };

    //`consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    //TODO: ^ try uncommenting this line.

    //`Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    //`move` keyword move variables from the enclosing scope to closure's memory. In this case
    // `contains` still implements Fn trait
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    //TODO: ^ uncommenting above line will result in compile-time error because borrow checker
    //doesn't allow re-using variable after it has been moved.

    //removing `move` from closure's signature will cause closure to borrow _haystack_ immutably,
    //hence _haystack_ is still available and uncommenting above line will not cause an error.
}
