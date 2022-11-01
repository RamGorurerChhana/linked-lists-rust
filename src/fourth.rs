use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // adds a new node to the front of the list
    pub fn push_front(&mut self, elem: T) {
        // create a new node
        let new_node = Node::new(elem);
        let new_node = Rc::new(RefCell::new(new_node));
        // take out the head replacing with None
        let mut old_head = self.head.take();
        // prev of old_head now points to the new_node
        old_head.as_mut().map(|node| {
            node.borrow_mut().prev = Some(Rc::clone(&new_node));
        });
        // next of new_node now points to the old_head
        new_node.borrow_mut().next = old_head.clone();
        // set new_node as the head of the list
        self.head = Some(new_node);
        // if tail is none that means adding node for the first time
        // then tail now points to the new_node
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }

    // adds a new node to the back of the list
    pub fn push_back(&mut self, elem: T) {
        // if tail of the list is empty then it is same as push_front
        if self.tail.is_none() {
            return self.push_front(elem);
        }
        // create a new node
        let new_node = Node::new(elem);
        let new_node = Rc::new(RefCell::new(new_node));
        // take out the head replacing with None
        let old_tail = self.tail.take().unwrap();
        // next of old_tail now points to new_node
        old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
        // prev of new_node points to old_tail
        new_node.borrow_mut().prev = Some(old_tail.clone());
        // set new_node as the tail of the list
        self.tail = Some(new_node);
    }

    // removes a node from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        // take out the head replacing with None
        let old_head = self.head.take();
        old_head.map(|node| {
            // take out the next of old_head and replace with None
            let mut next_node = node.borrow_mut().next.take();
            // prev of next_node now points to None
            next_node.as_mut().map(|node| {
                node.borrow_mut().prev = None;
            });
            // set next_node as the head of the list
            self.head = next_node;
            // if head of the list is None that means all nodes are popped
            // then tail also points to None
            if self.head.is_none() {
                self.tail = None;
            }
            // take out the elem from the node and return it
            Rc::try_unwrap(node).unwrap().into_inner().elem
        })
    }

    // removes a node from back of the list
    pub fn pop_back(&mut self) -> Option<T> {
        // take out the head replacing with None
        let old_tail = self.tail.take();
        old_tail.map(|node| {
            // take out the prev of old_tail and replace with None
            let mut prev_node = node.borrow_mut().prev.take();
            // next of prev_node now points to None
            prev_node.as_mut().map(|prv| {
                prv.borrow_mut().next = None;
            });
            // set prev_node as the tail of the list
            self.tail = prev_node;
            // if tail of the list is None that means all nodes are popped
            // then head also points to None
            if self.tail.is_none() {
                self.head = None;
            }
            // take out the elem from the node and return it
            Rc::try_unwrap(node).unwrap().into_inner().elem
        })
    }

    // returns the reference to the first element in the list from front
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    // returns the reference to the last element of the list
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    // returns mutable reference to the first element in the list
    pub fn peek_mut_front(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    // returns mutable reference to the last element in the list
    pub fn peek_mut_back(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.tail.take();
        let mut curr = self.head.take();
        while let Some(node) = curr {
            curr = Rc::try_unwrap(node).ok().and_then(|node| {
                node.borrow_mut().prev.take();
                node.borrow_mut().next.take()
            });
        }
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
            prev: None,
        }
    }
}

impl<T: Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("elem", &self.elem)
            .field("next", &self.next)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_1() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        assert!(list.peek_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_list_2() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        assert!(list.peek_back().is_none());
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(&*list.peek_back().unwrap(), &3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_list_3() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_list_4() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_list_5() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_mut_front().is_none());
        assert!(list.peek_mut_back().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&*list.peek_mut_front().unwrap(), &3);
        assert_eq!(&*list.peek_mut_back().unwrap(), &1);
        list.peek_mut_front().map(|mut node| *node += 1);
        list.peek_mut_back().map(|mut node| *node += 1);
        assert_eq!(&*list.peek_front().unwrap(), &4);
        assert_eq!(&*list.peek_back().unwrap(), &2);
    }
}
