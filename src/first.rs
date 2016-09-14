use std::mem::replace;


struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}


impl List {
    /// Create an empty list
    pub fn new() -> Self {
        List {head: Link::Empty}
    }

    /// Pust a value onto the start of the list
    pub fn push(&mut self, value: i32) {
        let new_node = Node{
            elem: value,
            next: replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }
}
