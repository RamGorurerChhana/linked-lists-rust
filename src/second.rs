#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    // creates an empty list
    pub fn new() -> Self {
        Self { head: None }
    }

    // returns true if list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // returns length of the list
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = self.head.as_ref();
        while let Some(nxt) = curr {
            len += 1;
            curr = nxt.next.as_ref();
        }
        len
    }

    // push an item into the list
    pub fn push(&mut self, elem: T) {
        // create a new node with empty next
        let mut new_node = Node::new(elem);
        // take out value from self.head replacing with None
        let old_head = self.head.take();
        // next of new_node will now hold whatever self.head was holding
        new_node.next = old_head;
        // set self.head as the new_node
        self.head = Some(Box::new(new_node));
    }

    // pop item from the list
    pub fn pop(&mut self) -> Option<T> {
        // replace self.head with next of the node
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // insert at a position
    // returns Err(usize) if the given index is larger than the list length
    pub fn insert_at(&mut self, index: usize, elem: T) -> Result<(), usize> {
        if index == 0 {
            self.push(elem);
            return Ok(());
        }
        // first shift curr_head upto the given index position
        let mut curr_head = self.head.as_mut();
        // loop one less so that stays on the node
        // just previous to the position of insertion
        for i in 1..index {
            curr_head = curr_head.ok_or(i)?.next.as_mut();
        }
        let mut new_node = Node::new(elem);
        let curr_head = curr_head.ok_or(index)?;
        let node_next = curr_head.next.take();
        new_node.next = node_next;
        curr_head.next = Some(Box::new(new_node));
        Ok(())
    }

    // returns the reference of the first item in the list
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // returns mutable reference of the first item in the list
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // returns IntoIter instance of the list
    // takes ownership of the list
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // returns Iter instance of the list
    pub fn iter(&self) -> Iter<'_, T> {
        let pointer = self.head.as_ref().map(|node| node.as_ref());
        Iter { pointer }
    }

    // returns IterMut instance of the list
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let pointer = self.head.as_mut().map(|node| node.as_mut());
        IterMut { pointer }
    }
}

// Implement Drop trait for List type
// clean up all nodes
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            curr = node.next.take();
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self { elem, next: None }
    }
}

#[derive(Debug)]
pub struct IntoIter<T>(List<T>);

// Implement Iterator for Iter
// This will allow to iterate over the list
// and get back each value in the list
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    pointer: Option<&'a Node<T>>,
}

// Implement Iterator for Iter
// This will allow to iterate over the list
// and get back a references over each item
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.take().map(|node| {
            self.pointer = node.next.as_ref().map(|nxt| nxt.as_ref());
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    pointer: Option<&'a mut Node<T>>,
}

// Implement Iterator for IterMut
// This will allow to iterate over the list
// and get back mutable references over each item
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.take().map(|node| {
            self.pointer = node.next.as_mut().map(|nxt| nxt.as_mut());
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_1() {
        let mut list = List::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        // when pop from empty list it returns None
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        // items are popped in reversed order
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        // returns None when all elements are popped
        assert_eq!(list.pop(), None);
        // when pop from empty list it returns None
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert!(!list.is_empty());
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        let e = list.peek_mut().unwrap();
        *e += 1;
        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.peek_mut(), Some(&mut 4));
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        list.iter_mut().for_each(|e| *e += 1);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_at() {
        let mut list = List::new();
        list.push(0);
        assert!(list.insert_at(1, 1).is_ok());
        assert!(list.insert_at(1, 2).is_ok());
        assert!(list.insert_at(1, 3).is_ok());
        assert!(list.insert_at(1, 4).is_ok());
        assert_eq!(list.len(), 5);
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.insert_at(1, 4).is_err());
        assert!(list.insert_at(0, 0).is_ok());
        assert!(list.insert_at(1, 1).is_ok());
        assert!(list.insert_at(2, 2).is_ok());
        assert!(list.insert_at(3, 3).is_ok());
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
    }
}
