#![allow(missing_docs)]

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

/// Make a type alias for Option<Box<_>>
type Link<T> = Option<Box<Node<T>>>;

/// A generic node that will hold our value plus a pointer
/// to the next node in the chain
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    /// Add a new element of to the front of our list
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Pop a value off the front of our list
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    /// Get a reference to the first element without
    /// actually removing it from the list
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Get a mutable reference to the first element without
    /// actually removing it from the list
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
}
