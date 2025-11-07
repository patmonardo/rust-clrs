//! Representing Rooted Trees (Section 10.4)
//!
//! This module contains representations of binary trees and rooted trees.

/// Node in a binary tree
#[derive(Debug, Clone)]
pub struct BinaryTreeNode<T> {
    pub key: T,
    pub left: Option<Box<BinaryTreeNode<T>>>,
    pub right: Option<Box<BinaryTreeNode<T>>>,
}

/// Binary tree
///
/// This corresponds to the binary tree representation from CLRS Section 10.4.
///
/// # Example
/// ```
/// use clrs::chapter_10::{BinaryTree, BinaryTreeNode};
/// let mut tree = BinaryTree::new();
/// tree.root = Some(Box::new(BinaryTreeNode {
///     key: 10,
///     left: Some(Box::new(BinaryTreeNode {
///         key: 5,
///         left: None,
///         right: None,
///     })),
///     right: None,
/// }));
/// ```
#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    pub root: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTree<T> {
    /// Creates a new empty binary tree
    pub fn new() -> Self {
        BinaryTree { root: None }
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Prints all keys in a binary tree using in-order traversal (Exercise 10.4-2)
///
/// This corresponds to PRINT-BINARY-TREE from CLRS Exercise 10.4-2.
///
/// # Arguments
/// * `tree` - The binary tree to print
/// * `visitor` - A closure that processes each key
///
/// # Complexity
/// - Time: O(n)
///
/// # Example
/// ```
/// use clrs::chapter_10::{BinaryTree, print_binary_tree};
/// let tree = BinaryTree::<i32>::new();
/// print_binary_tree(&tree, |key| println!("{}", key));
/// ```
pub fn print_binary_tree<T, F>(tree: &BinaryTree<T>, mut visitor: F)
where
    F: FnMut(&T),
{
    print_binary_tree_aux(tree.root.as_ref(), &mut visitor);
}

fn print_binary_tree_aux<T, F>(node: Option<&Box<BinaryTreeNode<T>>>, visitor: &mut F)
where
    F: FnMut(&T),
{
    if let Some(n) = node {
        print_binary_tree_aux(n.left.as_ref(), visitor);
        visitor(&n.key);
        print_binary_tree_aux(n.right.as_ref(), visitor);
    }
}

/// Prints all keys in a binary tree using iterative in-order traversal (Exercise 10.4-3)
///
/// This corresponds to the nonrecursive PRINT-BINARY-TREE from CLRS Exercise 10.4-3.
///
/// # Arguments
/// * `tree` - The binary tree to print
/// * `visitor` - A closure that processes each key
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(h) where h is the height of the tree
pub fn print_binary_tree_iterative<T, F>(tree: &BinaryTree<T>, mut visitor: F)
where
    F: FnMut(&T),
{
    use std::collections::VecDeque;
    
    let mut stack: VecDeque<&Box<BinaryTreeNode<T>>> = VecDeque::new();
    let mut current = tree.root.as_ref();

    loop {
        // Go to leftmost node
        while let Some(node) = current {
            stack.push_back(node);
            current = node.left.as_ref();
        }

        if stack.is_empty() {
            break;
        }

        // Pop and process
        if let Some(node) = stack.pop_back() {
            visitor(&node.key);
            // Move to right subtree
            current = node.right.as_ref();
        }
    }
}

/// Node in a left-child, right-sibling tree
#[derive(Debug, Clone)]
pub struct LCRSTreeNode<T> {
    pub key: T,
    pub left_child: Option<Box<LCRSTreeNode<T>>>,
    pub right_sibling: Option<Box<LCRSTreeNode<T>>>,
}

/// Left-child, right-sibling tree representation
///
/// This represents an arbitrary rooted tree using left-child, right-sibling representation.
#[derive(Debug, Clone)]
pub struct LCRSTree<T> {
    pub root: Option<Box<LCRSTreeNode<T>>>,
}

impl<T> LCRSTree<T> {
    /// Creates a new empty tree
    pub fn new() -> Self {
        LCRSTree { root: None }
    }
}

impl<T> Default for LCRSTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Prints all keys in a left-child, right-sibling tree (Exercise 10.4-4)
///
/// This corresponds to PRINT-LCRS-TREE from CLRS Exercise 10.4-4.
///
/// # Arguments
/// * `tree` - The tree to print
/// * `visitor` - A closure that processes each key
///
/// # Complexity
/// - Time: O(n)
pub fn print_lcrs_tree<T, F>(tree: &LCRSTree<T>, mut visitor: F)
where
    F: FnMut(&T),
{
    print_lcrs_tree_aux(tree.root.as_ref(), &mut visitor);
}

fn print_lcrs_tree_aux<T, F>(node: Option<&Box<LCRSTreeNode<T>>>, visitor: &mut F)
where
    F: FnMut(&T),
{
    if let Some(n) = node {
        visitor(&n.key);
        
        if let Some(left_child) = &n.left_child {
            print_lcrs_tree_aux(Some(left_child), visitor);
            
            let mut sibling = left_child.right_sibling.as_ref();
            while let Some(s) = sibling {
                print_lcrs_tree_aux(Some(s), visitor);
                sibling = s.right_sibling.as_ref();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let tree = BinaryTree {
            root: Some(Box::new(BinaryTreeNode {
                key: 10,
                left: Some(Box::new(BinaryTreeNode {
                    key: 5,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTreeNode {
                    key: 15,
                    left: None,
                    right: None,
                })),
            })),
        };
        
        let mut keys = Vec::new();
        print_binary_tree(&tree, |key| keys.push(*key));
        assert_eq!(keys, vec![5, 10, 15]);
    }

    #[test]
    fn test_binary_tree_iterative() {
        let tree = BinaryTree {
            root: Some(Box::new(BinaryTreeNode {
                key: 10,
                left: Some(Box::new(BinaryTreeNode {
                    key: 5,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTreeNode {
                    key: 15,
                    left: None,
                    right: None,
                })),
            })),
        };
        
        let mut keys = Vec::new();
        print_binary_tree_iterative(&tree, |key| keys.push(*key));
        // Should produce same in-order traversal: 5, 10, 15
        assert_eq!(keys, vec![5, 10, 15]);
    }

    #[test]
    fn test_lcrs_tree() {
        let tree = LCRSTree {
            root: Some(Box::new(LCRSTreeNode {
                key: 1,
                left_child: Some(Box::new(LCRSTreeNode {
                    key: 2,
                    left_child: None,
                    right_sibling: Some(Box::new(LCRSTreeNode {
                        key: 3,
                        left_child: None,
                        right_sibling: None,
                    })),
                })),
                right_sibling: None,
            })),
        };
        
        let mut keys = Vec::new();
        print_lcrs_tree(&tree, |key| keys.push(*key));
        assert_eq!(keys, vec![1, 2, 3]);
    }
}
