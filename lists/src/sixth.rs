pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    front: Link<T>,
    back: Link<T>,
}

