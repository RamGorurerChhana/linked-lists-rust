use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    // creates an empty list
    pub fn new() -> Self {
        Self { head: None }
    }

    // returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // returns the length of the list
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }
        len
    }

    // creates a new list prepending the node to the old list
    pub fn prepend(&self, elem: T) -> Self {
        let mut new_node = Node::new(elem);
        new_node.next = self.head.clone();
        let head = Some(Rc::new(new_node));
        Self { head }
    }

    // creates a new list by remoing the first item from the old list
    pub fn tail(&self) -> Self {
        // let head = match self.head.as_ref() {
        //     None => None,
        //     Some(node) => node.next.clone(),
        // };

        let head = self.head.as_ref().and_then(|node| node.next.clone());
        Self { head }
    }

    // returns reference to the first element in the list
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<T> {
        let pointer = self.head.as_deref();
        Iter { pointer }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(node) = curr {
            curr = Rc::try_unwrap(node)
                .ok()
                .and_then(|mut node| node.next.take());
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self { elem, next: None }
    }
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    pointer: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.map(|node| {
            self.pointer = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_1() {
        let list = List::new();
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.len(), 0);
        assert_eq!(list.head(), None);
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.len(), 3);
        let new_list = list.tail();
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.len(), 3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(new_list.is_empty(), false);
        assert_eq!(new_list.len(), 2);
        assert_eq!(new_list.head(), Some(&2));
    }

    #[test]
    fn test_iter_1() {
        let list = List::<i32>::new();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
