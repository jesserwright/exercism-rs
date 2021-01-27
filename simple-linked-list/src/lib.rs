use std::iter::FromIterator;
use std::mem;

#[derive(Debug, Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            Some(node) => {
                let mut len = 1;
                let mut cur_node = node;
                while let Some(ref next_node) = cur_node.next {
                    len += 1;
                    cur_node = next_node;
                }
                len
            }
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(n) => {
                self.head = n.next;
                Some(n.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        if let Some(first_node) = self.head {
            reversed.push(first_node.data);
            let mut curr = first_node.next;
            while let Some(node) = curr {
                reversed.push(node.data);
                curr = node.next;
            }
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v: Vec<T> = vec![];
        if let Some(first_node) = self.head {
            v.push(first_node.data);
            let mut curr = first_node.next;
            while let Some(node) = curr {
                v.push(node.data);
                curr = node.next;
            }
        }
        v.reverse();
        v
    }
}
