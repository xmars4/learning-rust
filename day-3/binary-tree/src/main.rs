use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Sub;
/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    fn len(&self) -> usize {
        match &self.0 {
            Some(node) => 1 + node.left.len() + node.right.len(),
            None => 0,
        }
    }
}

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree {
            root: Subtree(None),
        }
    }

    fn insert(&mut self, value: T) {
        let node_value = self.root.0.as_mut();
        match node_value {
            Some(node) => {
                let compare_result = node.value.cmp(&value);
                match compare_result {
                    Less => {
                        node.left = Subtree(Some(Box::new(Node {
                            value,
                            left: Subtree(None),
                            right: Subtree(None),
                        })))
                    }
                    Greater => {
                        node.right = Subtree(Some(Box::new(Node {
                            value,
                            left: Subtree(None),
                            right: Subtree(None),
                        })))
                    }
                    _ => (),
                }
            }
            None => {
                self.root.0 = Some(Box::new(Node {
                    value,
                    left: Subtree(None),
                    right: Subtree(None),
                }))
            }
        }
    }

    fn len(&self) -> usize {
        self.root.len()
    }

    fn has(&self, value: &T) -> bool {
        //
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
