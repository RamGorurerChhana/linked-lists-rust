use std::ptr;

#[derive(Debug)]
pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> List<T> {
    // creates an empty list
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    // adds a new node in the list in the back
    pub fn push(&mut self, elem: T) {
        // first create a box so that data is allocated on the heap and owned by the box
        // then create a raw pointer from the box
        let new_node = Box::new(Node::new(elem));
        let new_node = Box::into_raw(new_node);
        // if tail is null that means list is empty and pushing item for the first time
        if self.tail.is_null() {
            self.head = new_node;
        } else {
            unsafe {
                // next of current tail will now point to the new_node
                (*self.tail).next = new_node;
            }
        }
        // set tail to the new_node
        self.tail = new_node;
    }

    // removes a node from the list
    // remove from the front since it is FIFO
    pub fn pop(&mut self) -> Option<T> {
        // if head is null then return None
        if self.head.is_null() {
            None
        } else {
            unsafe {
                // take head and put into a box so that memory can be cleaned up
                // when the box goes out of scope, it automatically clean up the
                // underlu=ying memory
                let head = Box::from_raw(self.head);
                // current head will move one step and point to the next of current head
                self.head = head.next;
                // if head is becoming null that means all nodes are popped
                // reset tail to null as well
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                // return the element
                Some(head.elem)
            }
        }
    }

    // reutns the reference to the first element from the front
    pub fn peek(&self) -> Option<&T> {
        // if head is null then return None
        if self.head.is_null() {
            None
        } else {
            // dereference head and take referece to the element inside
            unsafe { Some(&(*self.head).elem) }
        }
    }

    // returns mutable reference to the first element from the front
    pub fn peek_mut(&self) -> Option<&mut T> {
        // if head is null then return None
        if self.head.is_null() {
            None
        } else {
            // dereference head and take mutable referece to the element inside
            unsafe { Some(&mut (*self.head).elem) }
        }
    }

    // creates an insatance of IntoIter for the list
    // also takes ownership of the list
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // creates an instance of Iter for the list
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // if head is null then list is empty
        let pointer = if self.head.is_null() {
            None
        } else {
            // dereference head and take reference to the node inside
            unsafe { Some(&(*self.head)) }
        };

        Iter { pointer }
    }

    // creates an instance of IterMut for the list
    pub fn iter_mut<'a>(&'a self) -> IterMut<'a, T> {
        // if head is null then list is empty
        let pointer = if self.head.is_null() {
            None
        } else {
            // dereference head and take reference to the node inside
            unsafe { Some(&mut (*self.head)) }
        };

        IterMut { pointer }
    }
}

// implement Drop for the list to make sure all allocated memory are cleaned up
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: ptr::null_mut(),
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    pointer: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.take().map(|node| {
            self.pointer = if node.next.is_null() {
                None
            } else {
                unsafe { Some(&*node.next) }
            };
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    pointer: Option<&'a mut Node<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.take().map(|node| {
            self.pointer = if node.next.is_null() {
                None
            } else {
                unsafe { Some(&mut *node.next) }
            };
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
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_2() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        list.push(2);
        assert_eq!(list.peek(), Some(&1));
        list.pop();
        assert_eq!(list.peek(), Some(&2));
        list.pop();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_list_3() {
        let mut list = List::new();
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        assert_eq!(list.peek_mut(), Some(&mut 1));
        *list.peek_mut().unwrap() += 1;
        assert_eq!(list.peek_mut(), Some(&mut 2));
        list.pop();
        assert_eq!(list.peek_mut(), None);
    }

    #[test]
    fn test_list_4() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_list_5() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_list_6() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
