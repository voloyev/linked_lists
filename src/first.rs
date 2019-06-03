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
