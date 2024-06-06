pub struct LinkedList<T> {
    head: Option<Node<T>>,
    len: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let mut node = Node::new(element);
        match self.head {
            None => self.head = Some(node),
            Some(_) => {
                node.next = self.head.take().map(Box::new);
                self.head = Some(node)
            }
        };
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head {
            None => None,
            Some(_) => {
                let node = self.head.take();
                node.map(|n| {
                    self.head = n.next.map(|next| *next);
                    self.len -= 1;
                    n.data
                })
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
fn main() {
    let mut list = LinkedList::new();
    list.push_front(10);
    list.push_front(20);
    list.push_front(5);
    println!("size: {}", list.len());
    println!("first: {:?}", list.pop_front());
    println!("second: {:?}", list.pop_front());
    println!("third: {:?}", list.pop_front());
    println!("size: {}", list.len())
}
