use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_tail;
            }
        } else {
            self.head = new_tail;
        }
        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.head.is_null() {
            let head;
            unsafe {
                head = Box::from_raw(self.head);
            }
            self.head = head.next;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            Some(head.elem)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| {
                &node.elem
            })
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| {
                &mut node.elem
            })
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| unsafe {
            self.next = node.next.as_ref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| unsafe {
            self.next = node.next.as_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, UnsafeCell};

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6);
        list.push(7);

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        // error[E0502]: cannot borrow `list` as mutable because it is also borrowed as immutable
        // list.push_front(4);
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        list.push(5);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        // error[E0499]: cannot borrow `list` as mutable more than once at a time
        // list.push_front(4);
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
        list.push(5);
        let mut iter = list.iter_mut();
        iter.next().map(|v| *v = 10);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn reborrow() {
        let mut data = 10;
        let ref1 = &mut data;
        let ref2 = &mut *ref1;

        *ref2 += 2;
        *ref1 += 1;

        println!("{}", data)
    }

    #[test]
    fn raw_pointer_reborrow() {
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;

            // ORDER SWAPPED!
            *ptr2 += 2;
            *ref1 += 1;

            println!("{}", data);
        }
    }

    #[test]
    fn raw_pointer_reborrow1() {
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;

            // Access the first raw pointer first
            // *ptr2 += 2;

            // Then access things in "borrow stack" order
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;

            println!("{}", data);
        }
    }

    #[test]
    fn raw_pointer_reborrow_array() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr2_at_0 = ref1_at_0 as *mut i32;
            let ptr3_at_1 = ptr2_at_0.add(1); // This wasn't allowed because ptr2_at_0 only manage index 0

            *ptr3_at_1 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [3, 3, 0, ...]
            println!("{:?}", &data[..]);
        }
    }

    #[test]
    fn raw_pointer_reborrow_array1() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr2_at_0 = ref1_at_0 as *mut i32;
            let ptr3_at_0 = ptr2_at_0;

            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [6, 0, 0, ...]
            println!("{:?}", &data[..]);
        }
    }

    #[test]
    fn raw_pointer_reborrow_array2() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr2_at_0 = ref1_at_0 as *mut i32;
            let ptr3_at_0 = ptr2_at_0;
            let ptr4_at_0 = ptr2_at_0.add(0);
            let ptr5_at_0 = ptr3_at_0.add(1).sub(1);

            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ptr4_at_0 += 4;
            *ptr5_at_0 += 5;
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [20, 0, 0, ...]
            println!("{:?}", &data[..]);
        }
    }

    #[allow(unused)]
    #[test]
    fn raw_pointer_reborrow_array3() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            // Error! can't `data[_]` as mutable more than once at a time
            // let ref2_at_1 = &mut data[1];
            // let ptr3_at_0 = ref1_at_0 as *mut i32;
            // let ptr4_at_1 = ref2_at_1 as *mut i32;
        }
    }

    #[test]
    fn raw_pointer_reborrow_array4() {
        unsafe {
            let mut data = [0; 10];
            let slice1 = &mut data[..];
            let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);
            let ref4_at_0 = &mut slice2_at_0[0];
            let ref5_at_1 = &mut slice3_at_1[0];
            let ptr6_at_0 = ref4_at_0 as *mut i32;
            let ptr7_at_1 = ref5_at_1 as *mut i32;

            *ptr7_at_1 += 7;
            *ptr6_at_0 += 6;
            *ref5_at_1 += 5;
            *ref4_at_0 += 4;

            // Should be [10, 12, 0, ...]
            println!("{:?}", data);
        }
    }

    #[test]
    fn raw_pointer_reborrow_array5() {
        unsafe {
            let mut data = [0usize; 10];

            let slice1_all = &mut data[..];
            let ptr2_all = slice1_all.as_mut_ptr();

            let ptr3_at_0 = ptr2_all;
            let ptr4_at_1 = ptr2_all.add(1); // This was allowed because ptr2_all manage index 1
            let ref5_at_0 = &mut *ptr3_at_0;
            let ref6_at_1 = &mut *ptr4_at_1;

            *ref6_at_1 += 6;
            *ref5_at_0 += 5;
            *ptr4_at_1 += 4;
            *ptr3_at_0 += 3;

            // Just for fun, modify all the element in a loop
            // (Could use any of the raw pointers for this, they share a borrow!)
            for idx in 0..10 {
                *ptr2_all.add(idx) += idx;
            }

            // Safe version of this same code for fun
            for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
                *elem_ref += idx;
            }

            // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
            println!("{:?}", data);
        }
    }

    fn opaque_read(val: &i32) {
        println!("{}", val);
    }

    #[test]
    fn share_references() {
        let mut data = 10;
        let mref1 = &mut data;
        // Error! cannot borrow `data` as immutable because it is also borrowed as mutable
        // let sref2 = &data;
        let sref2 = &mref1;
        let sref3 = sref2;
        let sref4 = &*sref2;

        // Random hash of shared reference reads
        opaque_read(sref3);
        opaque_read(sref2);
        opaque_read(sref4);
        opaque_read(sref2);
        opaque_read(sref3);

        *mref1 += 1;

        opaque_read(&data);
    }

    #[test]
    fn share_references_with_raw_pointer() {
        unsafe {
            let mut data = 10;
            let mref1 = &mut data;
            let ptr2 = mref1 as *mut i32;
            let sref3 = &*mref1;
            let ptr4 = sref3 as *const i32 as *mut i32;

            // Error! this is not allowed to cast &T to &mut or *mut T
            // *ptr4 += 4;
            // TODO ^ Try uncommenting this line
            opaque_read(&*ptr4);
            opaque_read(sref3);
            *ptr2 += 2;
            *mref1 += 1;

            opaque_read(&data);
        }
    }

    #[test]
    fn share_references_with_raw_pointer1() {
        unsafe {
            let mut data = 10;
            let mref1 = &mut data;
            let ptr2 = mref1 as *mut i32;
            let sref3 = &*mref1;

            *ptr2 += 2;
            opaque_read(sref3); // Read in the wrong order cause error: Undefined Behavior: trying to retag from <251558> for SharedReadOnly permission at
            *mref1 += 1;

            opaque_read(&data);
        }
    }

    #[test]
    fn cell() {
        unsafe {
            let mut data = Cell::new(10);
            let mref1 = &mut data;
            let ptr2 = mref1 as *mut Cell<i32>;
            let sref3 = &*mref1;

            sref3.set(sref3.get() + 3);
            (*ptr2).set((*ptr2).get() + 2);
            mref1.set(mref1.get() + 1);

            println!("{}", data.get());
        }
    }

    #[test]
    fn unsafe_cell() {
        unsafe {
            let mut data = UnsafeCell::new(10);
            let mref1 = data.get_mut(); // get a mutable ref to the contents
            let ptr2 = mref1 as *mut i32;
            let sref3 = &*ptr2;

            *ptr2 += 2;
            // error: Undefined Behavior: trying to retag from <255176> for SharedReadOnly permission at alloc85980[0x0]
            opaque_read(sref3);
            *mref1 += 1;

            println!("{}", *data.get());
        }
    }

    #[test]
    fn unsafe_cell1() {
        unsafe {
            let mut data = UnsafeCell::new(10);
            let mref1 = &mut data; // mutable ref to the *outside*
            let ptr2 = mref1.get(); // get a raw pointer to the insides
            let sref3 = &*mref1; // get a shared ref to the *outside*

            *ptr2 += 2; // mutable with raw pointer
            opaque_read(&*sref3.get()); // read from the shared ref
            *sref3.get() += 3; // write through the shared ref
            *mref1.get() += 1; // mutate with the mutable ref

            println!("{}", *data.get());
        }
    }

    #[test]
    fn unsafe_cell2() {
        unsafe {
            let mut data = UnsafeCell::new(10);
            let mref1 = &mut data;
            // These two are swapped so borrows are *definitely* totally stacked
            let sref2 = &*mref1;
            // Derive the ptr from the shared ref to be super safe!
            let ptr3 = sref2.get();

            *ptr3 += 3;
            opaque_read(&*sref2.get());
            *sref2.get() += 2;
            *mref1.get() += 1;

            println!("{}", *data.get());
        }
    }

    #[test]
    fn box_pointer() {
        unsafe {
            let mut data = Box::new(10);
            let ptr1 = (&mut *data) as *mut i32;

            *data += 10;
            // error: Undefined Behavior: attempting a read access using <264703> at alloc89399[0x0]
            *ptr1 += 1;

            // Should be 21
            println!("{}", data);
        }
    }

    #[test]
    fn box_pointer1() {
        unsafe {
            let mut data = Box::new(10);
            let ptr1 = (&mut *data) as *mut i32;

            *ptr1 += 1;
            *data += 10;

            // Should be 21
            println!("{}", data);
        }
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);
    }
}
