/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::vec::*;
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }

    fn add(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = None;
        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });

        match self.end {
            None => self.start = new_node_ptr,
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = new_node_ptr;
            },
        }

        self.end = new_node_ptr;
        self.length += 1;
    }
}

impl<T> LinkedList<T> {
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        // Merge list_a into merged_list
        while let Some(mut node_a) = list_a.start {
            let next_a = unsafe { node_a.as_mut() }.next.take();
            merged_list.add(unsafe { Box::from_raw(node_a.as_ptr()) }.val);
            list_a.start = next_a;
        }

        // Merge list_b into merged_list
        while let Some(mut node_b) = list_b.start {
            let next_b = unsafe { node_b.as_mut() }.next.take();
            merged_list.add(unsafe { Box::from_raw(node_b.as_ptr()) }.val);
            list_b.start = next_b;
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe {
                write!(f, "{} ", (*node_ptr.as_ptr()).val)?;
                current = (*node_ptr.as_ptr()).next;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_merge_linked_list() {
        let mut list_a = LinkedList::new();
        list_a.add(1);
        list_a.add(3);
        list_a.add(5);

        let mut list_b = LinkedList::new();
        list_b.add(2);
        list_b.add(4);
        list_b.add(6);

        let merged_list = LinkedList::merge(list_a, list_b);
        assert_eq!(format!("{}", merged_list), "1 3 5 2 4 6 ");
    }
}