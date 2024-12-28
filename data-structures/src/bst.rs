pub trait NodeValue: std::cmp::PartialOrd + std::cmp::PartialEq + Copy {}

impl<T> NodeValue for T where T: std::cmp::PartialOrd + std::cmp::PartialEq + Copy {}

#[derive(Debug)]
pub enum BST<T: NodeValue> {
    Node {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Nil,
}

pub fn new<T: NodeValue>(value: T) -> BST<T> {
    BST::Node {
        value,
        left: Box::new(BST::Nil),
        right: Box::new(BST::Nil),
    }
}

impl<T: NodeValue> BST<T> {
    pub fn insert(&mut self, value: T) {
        match self {
            Self::Node { value: v, left, right } => {
                if value < *v {
                    left.insert(value);
                } else {
                    right.insert(value);
                }
            },
            Self::Nil => {
                let new_node = new(value);
                *self = new_node;
            },
        }
    }

    pub fn rightmost(&self) -> &BST<T> {
        match self {
            Self::Node { value: _, left: _, right } if matches!(**right, Self::Nil) => {
                self
            },
            Self::Node { value: _, left: _, right } => {
                right.rightmost()
            },
            Self::Nil => panic!("WTF!?"),
        }
    }

    pub fn remove(&mut self, value: T) {
        match self {
            Self::Node { value: v, left, right }
            if matches!(**left, Self::Nil) && matches!(**right, Self::Nil) && value == *v => {
                *self = Self::Nil;
            },
            
            Self::Node { value: v, left, right }
            if matches!(**right, Self::Nil) && value == *v => {
                *self = std::mem::replace(&mut **left, Self::Nil);
            }
            
            Self::Node { value: v, left, right }
            if matches!(**left, Self::Nil) && value == *v => {
                *self = std::mem::replace(&mut **right, Self::Nil);
            },
            
            Self::Node { value: v, left, right: _ }
            if value == *v => {
                if let Self::Node { value: left_rightmost_value, left: _, right: _ } = left.rightmost() {
                    *v = left_rightmost_value.clone();

                    left.remove(*v);
                }
            },
            
            Self::Node { value: v, left, right }
            if value != *v => {
                if value < *v {
                    left.remove(value);
                } else {
                    right.remove(value);
                }
            },
            
            Self::Nil => (),

            _ => panic!("WTF!?"),
        }
    }

    pub fn pre_order_traversal<F>(&self, f: &F)
    where F: Fn(&T) {
        match self {
            Self::Node { value, left, right } => {
                f(value);
                left.pre_order_traversal(f);
                right.pre_order_traversal(f);
            },
            Self::Nil => (),
        }
    }

    pub fn in_order_traversal<F>(&self, f: &F)
    where F: Fn(&T) {
        match self {
            Self::Node { value, left, right } => {
                left.in_order_traversal(f);
                f(value);
                right.in_order_traversal(f);
            },
            Self::Nil => (),
        }
    }

    pub fn post_order_traversal<F>(&self, f: &F)
    where F: Fn(&T) {
        match self {
            Self::Node { value, left, right } => {
                left.in_order_traversal(f);
                right.in_order_traversal(f);
                f(value);
            },
            Self::Nil => (),
        }
    }

    pub fn level_order_traversal<F>(&self, f: &F)
    where F: Fn(&T) {
        let mut level_nodes = vec![self];

        while let Some(node) = level_nodes.pop() {
            if let BST::Node { value, left, right } = node {
                f(value);
                level_nodes.insert(0, left);
                level_nodes.insert(0, right);
            }
        }
    }
}

#[allow(dead_code)]
#[deprecated(since = "December 25th, 2024 1:20 PM", note = "please use `BST::level_order_traversal` method instead")]
fn level_order_traversal_v1<T, F>(mut level_nodes: Vec<&BST<T>>, mut exp: u32, f: F)
where T: NodeValue, F: Fn(&T) {
    let mut nodes_amount = i32::pow(2, exp);

    if level_nodes.iter().filter(|n| !matches!(***n, BST::Nil)).count() == 0 {
        return;
    }

    while nodes_amount > 0 {
        match level_nodes.pop() {
            Some(node) => {
                match node {
                    BST::Node { value, left, right } => {
                        f(value);
                        level_nodes.insert(0, &**left);
                        level_nodes.insert(0, &**right);
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        nodes_amount -= 1;
    }

    exp += 1;

    level_order_traversal_v1(level_nodes, exp, f);
}