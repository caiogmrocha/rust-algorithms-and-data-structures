
pub trait NodeValue: std::cmp::PartialOrd + std::cmp::PartialEq {}

impl<T> NodeValue for T where T: std::cmp::PartialOrd + std::cmp::PartialEq {}

#[derive(Debug)]
pub enum LinkedList<T: NodeValue> {
    Node {
        value: T,
        next: Box<LinkedList<T>>
    },
    Nil,
}

pub fn new<T: NodeValue>(value: T) -> LinkedList<T> {
    LinkedList::Node { value, next: Box::new(LinkedList::Nil) }
}

pub fn run() {
    let mut  list = new(1);

    list.insert(2);
    list.insert(2);
    list.insert(4);

    list.remove(2);
    list.remove(4);

    println!("{:?}", list);
}

impl<T: NodeValue> LinkedList<T> {
    pub fn insert(&mut self, value: T) {
        match self {
            LinkedList::Node { value: _value, next } => {
                next.insert(value);
            },
            LinkedList::Nil => {
                let new_node = LinkedList::Node { value, next: Box::new(LinkedList::Nil) };
                *self = new_node;
            }
        }
    }

    pub fn remove(&mut self, value: T) {
        match self {
            LinkedList::Node { value: v, next } => {
                if *v == value {
                    // Replace the current node with the next node
                    // This will drop the current node
                    // The next node will be moved to the current node
                    *self = std::mem::replace(&mut **next, LinkedList::Nil);
                } else {
                    next.remove(value);
                }
            },
            LinkedList::Nil => (),
        }
    }
}