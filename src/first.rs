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

    pub fn pop(&mut self) -> Option<i32> {
        match replace(&mut self.head, Link::Empty) {
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            },

            Link::Empty => {
                None
            },
        }
    }
}


impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = current_link {
            current_link = replace(&mut boxed_node.next, Link::Empty);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut my_list = List::new();

        // Check that an empty list behaves right
        assert_eq!(my_list.pop(), None);

        // Add some data
        my_list.push(1);
        my_list.push(2);
        my_list.push(3);

        // Check normal removal
        assert_eq!(my_list.pop(), Some(3));
        assert_eq!(my_list.pop(), Some(2));

        // Add on some more stuff to make sure it's not corrupted
        my_list.push(4);
        my_list.push(5);

        // Then remove them
        assert_eq!(my_list.pop(), Some(5));
        assert_eq!(my_list.pop(), Some(4));

        // And check for exhaustion
        assert_eq!(my_list.pop(), Some(1));
        assert_eq!(my_list.pop(), None);

    }

}
