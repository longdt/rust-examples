use std::cell::Cell;
use std::fmt::Debug;

fn take_two<T: Debug>(val1: T, val2: T) {
    println!("val1: {:?}, val2: {:?}", val1, val2);
}

fn take_refs<'big : 'small, 'small>(big: &'big u32, small: &'small u32) {
    take_two(big, small);
}

// Error! lifetime may not live long enough: argument requires that `'small` must outlive `'big`
// fn take_cell_refs<'big: 'small, 'small>(big: Cell<&'big u32>, small: Cell<&'small u32>) {
//     take_two(big, small);
// }

fn take_refs_cell<'big: 'small, 'small>(big: &'big Cell<u32>, small: &'small Cell<u32>) {
    take_two(big, small);
}

fn take_vec_refs<'big: 'small, 'small>(big: Vec<&'big u32>, small: Vec<&'small u32>) {
    take_two(big, small);
}

fn main() {
    let big = Cell::new(10);
    let big_ref = &big;
    {
        let small = Cell::new(1);
        let small_ref = &small;
        take_refs_cell(big_ref, small_ref);
    }
}