use std::mem;

pub struct Node {
    elem: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_new() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_push() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
        for i in &[1, 2, 3] {
            list.push(*i);
        }

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        for i in &[1, 2, 3] {
            list.push(*i);
        }

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
