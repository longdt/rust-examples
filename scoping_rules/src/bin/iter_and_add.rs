fn main() {
    let mut v = vec![1, 2, 3];
    let mut iter = v.iter();
    assert_eq!(iter.next(), Some(&1));
    // Error! cannot borrow `v` as mutable because it is also borrowed as immutable at `iter`
    // v.push(4);
    // Try to uncomment ^ this line
    assert_eq!(iter.next(), Some(&2));

    let mut v = vec![1, 2, 3];
    let mut iter = v.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    // Error! cannot borrow `v` as mutable more than once at a time
    // v.push(4);
    // Try to uncomment ^ this line
    assert_eq!(iter.next(), Some(&mut 2));
}
