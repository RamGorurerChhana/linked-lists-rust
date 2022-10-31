#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    // create a blank linked list
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // push an item into the linked list
    pub fn push(&mut self, elem: i32) {
        // create a new node with empty next
        let mut new_node = Node::new(elem);
        // take out self.head and replace with empty temporarily
        let old_head = std::mem::replace(&mut self.head, Link::Empty);
        // next of new_node will now hold whatever self.head was holding
        new_node.next = old_head;
        // set self.head as the new_node
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        // take out self.head and replace with empty temporarily
        let old_head = std::mem::replace(&mut self.head, Link::Empty);
        match old_head {
            // if head is empty then no item to pop, return None
            Link::Empty => None,
            // if head is some node, then replace self.head to next of the node
            Link::More(node) => {
                // set self.head to next
                self.head = node.next;
                // return elem
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = cur_link {
            cur_link = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl Node {
    fn new(elem: i32) -> Self {
        Self {
            elem,
            next: Link::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_1() {
        let mut list = List::new();
        // when pop from empty list it returns None
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        // items are popped in reversed order
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        // returns None when all elements are popped
        assert_eq!(list.pop(), None);
        // when pop from empty list it returns None
        assert_eq!(list.pop(), None);
    }
}
