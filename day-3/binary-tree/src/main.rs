use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::Display;
/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord + Display> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord + Display>(Option<Box<Node<T>>>);

impl<T: Ord + Display> Subtree<T> {
    fn len(&self) -> usize {
        match &self.0 {
            Some(node) => 1 + node.left.len() + node.right.len(),
            None => 0,
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            Some(node) => {
                let compare_result = value.cmp(&node.value);
                match compare_result {
                    Less => node.left.has(value),
                    Greater => node.right.has(value),
                    Equal => true,
                }
            }
            _ => false,
        }
    }

    fn insert(&mut self, value: T) {
        match self.0.as_mut() {
            Some(node) => match value.cmp(&node.value) {
                Less => match node.left {
                    Subtree(None) => {
                        node.left = Subtree(Some(Box::new(Node {
                            value,
                            left: Subtree(None),
                            right: Subtree(None),
                        })))
                    }
                    _ => node.left.insert(value),
                },
                Greater => match node.right {
                    Subtree(None) => {
                        node.right = Subtree(Some(Box::new(Node {
                            value,
                            left: Subtree(None),
                            right: Subtree(None),
                        })))
                    }
                    _ => node.right.insert(value),
                },
                Equal => (),
            },
            None => {
                self.0 = Some(Box::new(Node {
                    value,
                    left: Subtree(None),
                    right: Subtree(None),
                }))
            }
        }
    }
}

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord + Display> {
    root: Subtree<T>,
}

impl<T: Ord + Display> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree {
            root: Subtree(None),
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn len(&self) -> usize {
        self.root.len()
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }
}

// Implement `new`, `insert`, `len`, and `has`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
