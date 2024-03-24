use std::fmt::Debug;
use rand::Fill;

// 'static as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}

struct Ref<'a>(&'a i32);

// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static` lifetime is coerced to that of input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.try_fill(&mut rng).unwrap();
    Box::leak(boxed)
}

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    // A reference with 'static lifetime:
    let _s: &'static str = "hello world";
    let x = 10;
    let _tuple = &x;
    let _ref = Ref(&x);
    // Error! argument requires that `x` is borrowed for `'static`
    // generic(_tuple);
    // generic(_ref);

    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference can no longer be used, but data remains
        // in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);

    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second);
    println!("first: {:?}", first);

    // i is owned and contains no reference, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by scope of main(), so it's not 'static:
    // print_it(&i);
    // TODO ^ Try uncommenting this line
}
