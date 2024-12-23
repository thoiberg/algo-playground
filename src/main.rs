// TODO: replace Box with Rc<RefCell<>> as per
//      https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/

use std::{fmt::Display, ops::Deref};

fn main() {
    println!("Hello, world!");

    let mut root = Node::new(
        2,
        Some(Node::new(1, None, None)),
        Some(Node::new(3, None, None)),
    );

    let present_node = root.find(3);
    let missing_node = root.find(4);

    println!("3 is present?: {}", present_node.is_some());
    println!("4 is present?: {}", missing_node.is_some());

    root.add(4);
}
#[derive(Debug)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + std::fmt::Debug,
{
    fn new(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Box<Self> {
        Box::new(Self {
            val: value,
            left,
            right,
        })
    }

    fn find(&self, value: T) -> Option<&Node<T>> {
        if self.val == value {
            return Some(self);
        }

        if self.val > value {
            if let Some(left) = self.left.as_ref() {
                return left.find(value);
            }
        } else if let Some(right) = self.right.as_ref() {
            return right.find(value);
        }

        None
    }

    fn add(&mut self, new_value: T) {
        if self.val > new_value {
            if self.left.is_some() {
                let left = self.left.as_mut().unwrap();
                left.add(new_value);
            } else {
                self.left = Some(Node::new(new_value, None, None));
            }
        } else {
            if self.right.is_some() {
                let right = self.right.as_mut().unwrap();
                right.add(new_value);
            } else {
                self.right = Some(Node::new(new_value, None, None));
            }
        }
    }

    fn delete(&mut self, value: T) {
        // if node is leaf, then just delete
        // if node has one child, then replace with child
        // if node has two children, then use the right most (??) one (will need to check the textbook)

        // let boop = *(self.left.unwrap());

        if self.val > value {
            if let Some(left) = self.left.as_mut() {
                if left.val == value {
                    if left.left.is_none() && left.right.is_none() {
                        // leaf node
                        self.left = None
                    } else if left.left.is_some() && left.right.is_some() {
                        // both children
                    } else {
                        let child = *left.left.unwrap();

                        // self.left = Some(child.to_owned());
                        // single child
                    }
                } else {
                    left.delete(value);
                }
            }
        } else if let Some(right) = self.right.as_mut() {
            right.delete(value);
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_node_find() {
        let root = Node::new(
            2,
            Some(Node::new(1, None, None)),
            Some(Node::new(3, None, None)),
        );

        let present_node = root.find(3);
        let missing_node = root.find(4);

        assert!(present_node.is_some());
        assert!(missing_node.is_none());
    }

    #[test]
    fn test_node_add() {
        let mut root = Node::new(
            3,
            Some(Node::new(2, None, None)),
            Some(Node::new(4, None, None)),
        );

        assert!(root.find(5).is_none());
        root.add(5);
        assert!(root.find(5).is_some());

        assert!(root.find(1).is_none());
        root.add(1);
        assert!(root.find(1).is_some());
    }

    #[test]
    fn test_delete_leaf_node() {
        let mut root = Node::new(
            3,
            Some(Node::new(2, None, None)),
            Some(Node::new(4, None, None)),
        );

        assert!(root.find(2).is_some());
        root.delete(2);
        assert!(root.find(2).is_none());
    }

    #[test]
    fn test_delete_node_with_single_child() {
        let mut root = Node::new(
            3,
            Some(Node::new(2, Some(Node::new(1, None, None)), None)),
            Some(Node::new(4, None, None)),
        );

        assert!(root.find(2).is_some());
        assert_eq!(root.find(3).unwrap().left.as_ref().unwrap().val, 2);
        root.delete(2);
        assert!(root.find(2).is_none());
        assert_eq!(root.find(3).unwrap().left.as_ref().unwrap().val, 1);
    }

    #[test]
    fn test_delete_node_with_both_children() {
        let mut root = Node::new(
            5,
            Some(Node::new(
                3,
                Some(Node::new(2, Some(Node::new(1, None, None)), None)),
                Some(Node::new(4, None, None)),
            )),
            Some(Node::new(6, None, None)),
        );

        assert!(root.find(3).is_some());
        assert_eq!(root.find(5).unwrap().left.as_ref().unwrap().val, 3);
        root.delete(3);
        assert!(root.find(3).is_none());
        assert_eq!(root.find(5).unwrap().left.as_ref().unwrap().val, 1);
    }
}
