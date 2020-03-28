use crate::List::*;

fn main() {
    //create an empty linked list
    let mut list = List::new();

    //prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    //show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

enum List {
    //Cons: tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    //Nil: a node that signifies the end of the linked list
    Nil,
}

//methods can be attached to an enum
impl List {
    fn new() -> List {
        //`Nil` has type `List`
        Self::Nil
    }

    //consume a list, and return the same list with a new element at its front
    fn prepend(self, element: u32) -> List {
        //`Cons` also has type `List`
        Self::Cons(element, Box::new(self))
    }

    //return the length of the list
    fn len(&self) -> u32 {
        //`self` has to be matched, because the behavior of this method depend on the variant of `self`
        //`self` has type `&List`, and `*self` has type `List`, matching on a concrete type `T` is
        //preferred over a match on a reference `&T`
        match *self {
            //can't take ownership of the tail, because `self` is borrowed;
            //instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            //base case: an empty list has zero length
            Nil => 0,
        }
    }

    //return representation of the list as a (heap allocated) `String`
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                //`format!` is similar to `print!`, but returns a heap allocated String
                //instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => String::from("Nil"),
        }
    }
}
