// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}
fn main() {
    // _Stack_ allocated integer
    let x = 5u32;
    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x at {:p} is {}, and y at {:p} is {}", &x, x, &y, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);
    println!("a at {:p} contains: {}", a, a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    println!("b at {:p} contains: {}", b, b);

    // Error! `a` can no longer access the data, because it no longer owns the heap memory
    // println!("a at {:p} contains: {}", a, a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would result in dereferencing
    // freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b at {:p} contains: {}", b, b);
    // TODO ^ Try uncommenting this line
}
