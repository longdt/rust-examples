use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)]    // Allow equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>
}

// error[E0392]: parameter `T` is never used
// = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
// = help: if you intended `T` to be a const parameter, use `const T: usize` instead
// struct RedundancyStruct<T> {
//     value: i32,
// }

// Note: Storage is allowed to generic type `A`, but not for `B`.
// Therefore, `B` cannot be used in computations.

fn main() {
    // Here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple type specified as `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    // Type specified as `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Compile-time Error! mismatched types
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    // Compile-time Error! mismatched types
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
