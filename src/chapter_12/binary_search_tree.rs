//! Binary Search Trees (Sections 12.1-12.4)
//!
//! A binary search tree is a data structure that supports many dynamic-set
//! operations, including SEARCH, MINIMUM, MAXIMUM, PREDECESSOR, SUCCESSOR,
//! INSERT, and DELETE.

use std::cmp::Ordering;

/// Node in a binary search tree
#[derive(Debug, Clone)]
pub struct BSTNode<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub left: Option<Box<BSTNode<K, V>>>,
    pub right: Option<Box<BSTNode<K, V>>>,
}

/// Binary search tree
///
/// This corresponds to the binary search tree implementation from CLRS Chapter 12.
///
/// # Example
/// ```
/// use clrs::chapter_12::BinarySearchTree;
/// let mut tree = BinarySearchTree::new();
/// tree.insert(5, "value5");
/// tree.insert(3, "value3");
/// tree.insert(7, "value7");
/// assert_eq!(tree.search(5), Some(&"value5"));
/// ```
#[derive(Debug, Clone)]
pub struct BinarySearchTree<K: Ord, V> {
    pub root: Option<Box<BSTNode<K, V>>>,
}

impl<K: Ord, V> BinarySearchTree<K, V> {
    /// Creates a new empty binary search tree
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_12::BinarySearchTree;
    /// let tree: BinarySearchTree<i32, &str> = BinarySearchTree::new();
    /// ```
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Searches for a key in the tree
    ///
    /// This corresponds to TREE-SEARCH from CLRS Section 12.2.
    ///
    /// # Arguments
    /// * `k` - The key to search for
    ///
    /// # Returns
    /// A reference to the value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn search(&self, k: K) -> Option<&V> {
        self.search_node(&self.root, &k)
    }

    fn search_node<'a>(&self, node: &'a Option<Box<BSTNode<K, V>>>, k: &K) -> Option<&'a V> {
        match node {
            None => None,
            Some(n) => match k.cmp(&n.key) {
                Ordering::Equal => Some(&n.value),
                Ordering::Less => self.search_node(&n.left, k),
                Ordering::Greater => self.search_node(&n.right, k),
            },
        }
    }

    /// Finds the minimum key in the tree
    ///
    /// This corresponds to TREE-MINIMUM from CLRS Section 12.2.
    ///
    /// # Returns
    /// A reference to the minimum key-value pair, or `None` if tree is empty
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn minimum(&self) -> Option<(&K, &V)> {
        self.minimum_node(&self.root).map(|n| (&n.key, &n.value))
    }

    fn minimum_node<'a>(&self, node: &'a Option<Box<BSTNode<K, V>>>) -> Option<&'a BSTNode<K, V>> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self.minimum_node(&n.left)
                }
            }
        }
    }

    /// Finds the maximum key in the tree
    ///
    /// This corresponds to TREE-MAXIMUM from CLRS Section 12.2.
    ///
    /// # Returns
    /// A reference to the maximum key-value pair, or `None` if tree is empty
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn maximum(&self) -> Option<(&K, &V)> {
        self.maximum_node(&self.root).map(|n| (&n.key, &n.value))
    }

    fn maximum_node<'a>(&self, node: &'a Option<Box<BSTNode<K, V>>>) -> Option<&'a BSTNode<K, V>> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    self.maximum_node(&n.right)
                }
            }
        }
    }

    /// Finds the successor of a key
    ///
    /// This corresponds to TREE-SUCCESSOR from CLRS Section 12.2.
    ///
    /// # Arguments
    /// * `k` - The key to find the successor for
    ///
    /// # Returns
    /// A reference to the successor key-value pair, or `None` if no successor exists
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn successor(&self, k: &K) -> Option<(&K, &V)> {
        // First find the node with key k
        let node_opt = self.find_node(&self.root, k);
        match node_opt {
            None => None,
            Some(node) => {
                // If right subtree exists, successor is minimum of right subtree
                if node.right.is_some() {
                    self.minimum_node(&node.right).map(|n| (&n.key, &n.value))
                } else {
                    // Otherwise, find the lowest ancestor whose left child is also an ancestor
                    Self::find_successor_ancestor(&self.root, k, None)
                }
            }
        }
    }

    fn find_node<'a>(
        &'a self,
        node: &'a Option<Box<BSTNode<K, V>>>,
        k: &K,
    ) -> Option<&'a BSTNode<K, V>> {
        match node {
            None => None,
            Some(n) => match k.cmp(&n.key) {
                Ordering::Equal => Some(n),
                Ordering::Less => self.find_node(&n.left, k),
                Ordering::Greater => self.find_node(&n.right, k),
            },
        }
    }

    fn find_successor_ancestor<'a>(
        node: &'a Option<Box<BSTNode<K, V>>>,
        k: &K,
        candidate: Option<&'a BSTNode<K, V>>,
    ) -> Option<(&'a K, &'a V)> {
        match node {
            None => candidate.map(|n| (&n.key, &n.value)),
            Some(n) => {
                if k < &n.key {
                    // Current node is a candidate, search left
                    Self::find_successor_ancestor(&n.left, k, Some(n))
                } else {
                    // Search right, keep current candidate
                    Self::find_successor_ancestor(&n.right, k, candidate)
                }
            }
        }
    }

    /// Finds the predecessor of a key
    ///
    /// This corresponds to TREE-PREDECESSOR from CLRS Exercise 12.2-3.
    ///
    /// # Arguments
    /// * `k` - The key to find the predecessor for
    ///
    /// # Returns
    /// A reference to the predecessor key-value pair, or `None` if no predecessor exists
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn predecessor(&self, k: &K) -> Option<(&K, &V)> {
        let node_opt = self.find_node(&self.root, k);
        match node_opt {
            None => None,
            Some(node) => {
                // If left subtree exists, predecessor is maximum of left subtree
                if node.left.is_some() {
                    self.maximum_node(&node.left).map(|n| (&n.key, &n.value))
                } else {
                    // Otherwise, find the lowest ancestor whose right child is also an ancestor
                    Self::find_predecessor_ancestor(&self.root, k, None)
                }
            }
        }
    }

    fn find_predecessor_ancestor<'a>(
        node: &'a Option<Box<BSTNode<K, V>>>,
        k: &K,
        candidate: Option<&'a BSTNode<K, V>>,
    ) -> Option<(&'a K, &'a V)> {
        match node {
            None => candidate.map(|n| (&n.key, &n.value)),
            Some(n) => {
                if k > &n.key {
                    // Current node is a candidate, search right
                    Self::find_predecessor_ancestor(&n.right, k, Some(n))
                } else {
                    // Search left, keep current candidate
                    Self::find_predecessor_ancestor(&n.left, k, candidate)
                }
            }
        }
    }

    /// Inserts a key-value pair into the tree
    ///
    /// This corresponds to TREE-INSERT from CLRS Section 12.3.
    ///
    /// # Arguments
    /// * `k` - The key to insert
    /// * `v` - The value to insert
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn insert(&mut self, k: K, v: V) {
        let new_node = Box::new(BSTNode {
            key: k,
            value: v,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(new_node);
        } else {
            Self::insert_node(&mut self.root, new_node);
        }
    }

    fn insert_node(node: &mut Option<Box<BSTNode<K, V>>>, new_node: Box<BSTNode<K, V>>) {
        match node {
            None => *node = Some(new_node),
            Some(n) => {
                match new_node.key.cmp(&n.key) {
                    Ordering::Less => Self::insert_node(&mut n.left, new_node),
                    Ordering::Greater => Self::insert_node(&mut n.right, new_node),
                    Ordering::Equal => {
                        // Key already exists, update value
                        n.value = new_node.value;
                    }
                }
            }
        }
    }

    /// Deletes a key from the tree
    ///
    /// This corresponds to TREE-DELETE from CLRS Section 12.3.
    ///
    /// # Arguments
    /// * `k` - The key to delete
    ///
    /// # Returns
    /// The deleted value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(h) where h is the height of the tree
    pub fn delete(&mut self, k: &K) -> Option<V> {
        Self::delete_node(&mut self.root, k)
    }

    fn delete_node(node: &mut Option<Box<BSTNode<K, V>>>, k: &K) -> Option<V> {
        match node.take() {
            None => None,
            Some(mut n) => {
                match k.cmp(&n.key) {
                    Ordering::Less => {
                        let result = Self::delete_node(&mut n.left, k);
                        *node = Some(n);
                        result
                    }
                    Ordering::Greater => {
                        let result = Self::delete_node(&mut n.right, k);
                        *node = Some(n);
                        result
                    }
                    Ordering::Equal => {
                        // Found the node to delete
                        let value = n.value;

                        match (n.left.take(), n.right.take()) {
                            (None, None) => {
                                // No children
                                *node = None;
                            }
                            (Some(left), None) => {
                                // Only left child
                                *node = Some(left);
                            }
                            (None, Some(right)) => {
                                // Only right child
                                *node = Some(right);
                            }
                            (Some(left), Some(right)) => {
                                // Two children: find successor (minimum of right subtree)
                                let mut right_opt = Some(right);
                                let (succ_key, succ_value) = Self::extract_minimum(&mut right_opt);
                                *node = Some(Box::new(BSTNode {
                                    key: succ_key,
                                    value: succ_value,
                                    left: Some(left),
                                    right: right_opt,
                                }));
                            }
                        }

                        Some(value)
                    }
                }
            }
        }
    }

    fn extract_minimum(node: &mut Option<Box<BSTNode<K, V>>>) -> (K, V) {
        match node.take() {
            None => panic!("extract_minimum called on None"),
            Some(mut n) => {
                if n.left.is_none() {
                    // This is the minimum
                    let right = n.right.take();
                    let key = n.key;
                    let value = n.value;
                    *node = right;
                    (key, value)
                } else {
                    let (key, value) = Self::extract_minimum(&mut n.left);
                    *node = Some(n);
                    (key, value)
                }
            }
        }
    }

    /// Performs an in-order tree walk
    ///
    /// This corresponds to INORDER-TREE-WALK from CLRS Section 12.1.
    ///
    /// # Arguments
    /// * `visitor` - A closure that processes each key-value pair
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    pub fn inorder_walk<F>(&self, mut visitor: F)
    where
        F: FnMut(&K, &V),
    {
        self.inorder_walk_node(&self.root, &mut visitor);
    }

    fn inorder_walk_node<F>(&self, node: &Option<Box<BSTNode<K, V>>>, visitor: &mut F)
    where
        F: FnMut(&K, &V),
    {
        if let Some(n) = node {
            self.inorder_walk_node(&n.left, visitor);
            visitor(&n.key, &n.value);
            self.inorder_walk_node(&n.right, visitor);
        }
    }

    /// Performs a pre-order tree walk
    ///
    /// This corresponds to PREORDER-TREE-WALK from CLRS Exercise 12.1-4.
    ///
    /// # Arguments
    /// * `visitor` - A closure that processes each key-value pair
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    pub fn preorder_walk<F>(&self, mut visitor: F)
    where
        F: FnMut(&K, &V),
    {
        self.preorder_walk_node(&self.root, &mut visitor);
    }

    fn preorder_walk_node<F>(&self, node: &Option<Box<BSTNode<K, V>>>, visitor: &mut F)
    where
        F: FnMut(&K, &V),
    {
        if let Some(n) = node {
            visitor(&n.key, &n.value);
            self.preorder_walk_node(&n.left, visitor);
            self.preorder_walk_node(&n.right, visitor);
        }
    }

    /// Performs a post-order tree walk
    ///
    /// This corresponds to POSTORDER-TREE-WALK from CLRS Exercise 12.1-4.
    ///
    /// # Arguments
    /// * `visitor` - A closure that processes each key-value pair
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    pub fn postorder_walk<F>(&self, mut visitor: F)
    where
        F: FnMut(&K, &V),
    {
        self.postorder_walk_node(&self.root, &mut visitor);
    }

    fn postorder_walk_node<F>(&self, node: &Option<Box<BSTNode<K, V>>>, visitor: &mut F)
    where
        F: FnMut(&K, &V),
    {
        if let Some(n) = node {
            self.postorder_walk_node(&n.left, visitor);
            self.postorder_walk_node(&n.right, visitor);
            visitor(&n.key, &n.value);
        }
    }
}

impl<K: Ord, V> Default for BinarySearchTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_insert_and_search() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");

        assert_eq!(tree.search(5), Some(&"value5"));
        assert_eq!(tree.search(3), Some(&"value3"));
        assert_eq!(tree.search(7), Some(&"value7"));
        assert_eq!(tree.search(10), None);
    }

    #[test]
    fn test_bst_minimum_maximum() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(9, "value9");

        assert_eq!(tree.minimum(), Some((&1, &"value1")));
        assert_eq!(tree.maximum(), Some((&9, &"value9")));
    }

    #[test]
    fn test_bst_successor() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(9, "value9");

        assert_eq!(tree.successor(&5), Some((&7, &"value7")));
        assert_eq!(tree.successor(&3), Some((&5, &"value5")));
        assert_eq!(tree.successor(&7), Some((&9, &"value9")));
        assert_eq!(tree.successor(&9), None);
    }

    #[test]
    fn test_bst_predecessor() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(9, "value9");

        assert_eq!(tree.predecessor(&5), Some((&3, &"value3")));
        assert_eq!(tree.predecessor(&3), Some((&1, &"value1")));
        assert_eq!(tree.predecessor(&7), Some((&5, &"value5")));
        assert_eq!(tree.predecessor(&1), None);
    }

    #[test]
    fn test_bst_delete_no_children() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");

        assert_eq!(tree.delete(&3), Some("value3"));
        assert_eq!(tree.search(3), None);
        assert_eq!(tree.search(5), Some(&"value5"));
    }

    #[test]
    fn test_bst_delete_one_child() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(1, "value1");

        assert_eq!(tree.delete(&3), Some("value3"));
        assert_eq!(tree.search(3), None);
        assert_eq!(tree.search(1), Some(&"value1"));
        assert_eq!(tree.search(5), Some(&"value5"));
    }

    #[test]
    fn test_bst_delete_two_children() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(4, "value4");

        assert_eq!(tree.delete(&3), Some("value3"));
        assert_eq!(tree.search(3), None);
        assert_eq!(tree.search(1), Some(&"value1"));
        assert_eq!(tree.search(4), Some(&"value4"));
        assert_eq!(tree.search(5), Some(&"value5"));
    }

    #[test]
    fn test_bst_inorder_walk() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(9, "value9");

        let mut keys = Vec::new();
        tree.inorder_walk(|k, _| keys.push(*k));
        assert_eq!(keys, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_bst_preorder_walk() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");

        let mut keys = Vec::new();
        tree.preorder_walk(|k, _| keys.push(*k));
        assert_eq!(keys, vec![5, 3, 7]);
    }

    #[test]
    fn test_bst_postorder_walk() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");

        let mut keys = Vec::new();
        tree.postorder_walk(|k, _| keys.push(*k));
        assert_eq!(keys, vec![3, 7, 5]);
    }
}
