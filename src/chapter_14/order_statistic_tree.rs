//! Order-Statistic Trees (Section 14.1)
//!
//! An order-statistic tree is a red-black tree that is augmented with size
//! information, allowing us to quickly determine the rank of an element and
//! to select an element of a given rank.

use std::cmp::Ordering;

/// Node in an order-statistic tree
///
/// This is a red-black tree node augmented with size information.
#[derive(Debug, Clone)]
pub struct OSTNode<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub size: usize, // Number of nodes in subtree rooted at this node
    pub left: Option<Box<OSTNode<K, V>>>,
    pub right: Option<Box<OSTNode<K, V>>>,
}

/// Order-statistic tree
///
/// This corresponds to the order-statistic tree from CLRS Section 14.1.
/// It supports OS-SELECT and OS-RANK operations in O(lg n) time.
///
/// # Example
/// ```
/// use clrs::chapter_14::OrderStatisticTree;
/// let mut tree = OrderStatisticTree::new();
/// tree.insert(5, "value5");
/// tree.insert(3, "value3");
/// tree.insert(7, "value7");
/// assert_eq!(tree.select(2), Some((&3, &"value3")));
/// ```
#[derive(Debug, Clone)]
pub struct OrderStatisticTree<K: Ord, V> {
    pub root: Option<Box<OSTNode<K, V>>>,
}

impl<K: Ord, V> OrderStatisticTree<K, V> {
    /// Creates a new empty order-statistic tree
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_14::OrderStatisticTree;
    /// let tree: OrderStatisticTree<i32, &str> = OrderStatisticTree::new();
    /// ```
    pub fn new() -> Self {
        OrderStatisticTree { root: None }
    }

    /// Selects the element with the i-th smallest key
    ///
    /// This corresponds to OS-SELECT from CLRS Section 14.1.
    ///
    /// # Arguments
    /// * `i` - The rank (1-indexed)
    ///
    /// # Returns
    /// A reference to the key-value pair of the i-th smallest element, or `None` if i is out of range
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of nodes
    pub fn select(&self, i: usize) -> Option<(&K, &V)> {
        if i == 0 || i > self.size() {
            return None;
        }
        Self::select_node(&self.root, i).map(|n| (&n.key, &n.value))
    }

    fn select_node(node: &Option<Box<OSTNode<K, V>>>, i: usize) -> Option<&OSTNode<K, V>> {
        match node {
            None => None,
            Some(n) => {
                let r = n.left_size() + 1;
                match i.cmp(&r) {
                    Ordering::Equal => Some(n),
                    Ordering::Less => Self::select_node(&n.left, i),
                    Ordering::Greater => Self::select_node(&n.right, i - r),
                }
            }
        }
    }

    /// Determines the rank of an element with key k
    ///
    /// This corresponds to OS-RANK from CLRS Section 14.1.
    ///
    /// # Arguments
    /// * `k` - The key to find the rank of
    ///
    /// # Returns
    /// The rank (1-indexed) of the key, or `None` if the key is not found
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of nodes
    pub fn rank(&self, k: &K) -> Option<usize> {
        Self::rank_node(&self.root, k, 0)
    }

    fn rank_node(node: &Option<Box<OSTNode<K, V>>>, k: &K, mut r: usize) -> Option<usize> {
        match node {
            None => None,
            Some(n) => match k.cmp(&n.key) {
                Ordering::Equal => Some(r + n.left_size() + 1),
                Ordering::Less => Self::rank_node(&n.left, k, r),
                Ordering::Greater => {
                    r += n.left_size() + 1;
                    Self::rank_node(&n.right, k, r)
                }
            },
        }
    }

    /// Determines the rank of a key (alternative implementation)
    ///
    /// This corresponds to OS-KEY-RANK from CLRS Exercise 14.1-4.
    ///
    /// # Arguments
    /// * `k` - The key to find the rank of
    ///
    /// # Returns
    /// The rank (1-indexed) of the key, or `None` if the key is not found
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of nodes
    pub fn key_rank(&self, k: &K) -> Option<usize> {
        Self::key_rank_node(&self.root, k)
    }

    fn key_rank_node(node: &Option<Box<OSTNode<K, V>>>, k: &K) -> Option<usize> {
        match node {
            None => None,
            Some(n) => match k.cmp(&n.key) {
                Ordering::Equal => Some(n.left_size() + 1),
                Ordering::Less => Self::key_rank_node(&n.left, k),
                Ordering::Greater => {
                    let right_rank = Self::key_rank_node(&n.right, k)?;
                    Some(n.left_size() + 1 + right_rank)
                }
            },
        }
    }

    /// Inserts a key-value pair into the tree
    ///
    /// This augments TREE-INSERT from CLRS Section 12.3 with size maintenance.
    ///
    /// # Arguments
    /// * `k` - The key to insert
    /// * `v` - The value to insert
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of nodes
    pub fn insert(&mut self, k: K, v: V) {
        let new_node = Box::new(OSTNode {
            key: k,
            value: v,
            size: 1,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(new_node);
        } else {
            Self::insert_node(&mut self.root, new_node);
        }
    }

    fn insert_node(node: &mut Option<Box<OSTNode<K, V>>>, new_node: Box<OSTNode<K, V>>) {
        match node {
            None => *node = Some(new_node),
            Some(n) => {
                n.size += 1; // Increment size on path
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

    /// Returns the total number of nodes in the tree
    ///
    /// # Returns
    /// The size of the tree
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn size(&self) -> usize {
        self.root.as_ref().map(|n| n.size).unwrap_or(0)
    }

    /// Searches for a key in the tree
    ///
    /// # Arguments
    /// * `k` - The key to search for
    ///
    /// # Returns
    /// A reference to the value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of nodes
    pub fn search(&self, k: K) -> Option<&V> {
        self.search_node(&self.root, &k)
    }

    fn search_node<'a>(&self, node: &'a Option<Box<OSTNode<K, V>>>, k: &K) -> Option<&'a V> {
        match node {
            None => None,
            Some(n) => match k.cmp(&n.key) {
                Ordering::Equal => Some(&n.value),
                Ordering::Less => self.search_node(&n.left, k),
                Ordering::Greater => self.search_node(&n.right, k),
            },
        }
    }
}

impl<K: Ord, V> OSTNode<K, V> {
    fn left_size(&self) -> usize {
        self.left.as_ref().map(|n| n.size).unwrap_or(0)
    }
}

impl<K: Ord, V> Default for OrderStatisticTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ost_new() {
        let tree: OrderStatisticTree<i32, &str> = OrderStatisticTree::new();
        assert_eq!(tree.size(), 0);
    }

    #[test]
    fn test_ost_insert_and_size() {
        let mut tree = OrderStatisticTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");

        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn test_ost_select() {
        let mut tree = OrderStatisticTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(9, "value9");

        assert_eq!(tree.select(1), Some((&1, &"value1")));
        assert_eq!(tree.select(2), Some((&3, &"value3")));
        assert_eq!(tree.select(3), Some((&5, &"value5")));
        assert_eq!(tree.select(4), Some((&7, &"value7")));
        assert_eq!(tree.select(5), Some((&9, &"value9")));
    }

    #[test]
    fn test_ost_rank() {
        let mut tree = OrderStatisticTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");
        tree.insert(1, "value1");
        tree.insert(9, "value9");

        assert_eq!(tree.rank(&1), Some(1));
        assert_eq!(tree.rank(&3), Some(2));
        assert_eq!(tree.rank(&5), Some(3));
        assert_eq!(tree.rank(&7), Some(4));
        assert_eq!(tree.rank(&9), Some(5));
    }

    #[test]
    fn test_ost_key_rank() {
        let mut tree = OrderStatisticTree::new();
        tree.insert(5, "value5");
        tree.insert(3, "value3");
        tree.insert(7, "value7");

        assert_eq!(tree.key_rank(&3), Some(1));
        assert_eq!(tree.key_rank(&5), Some(2));
        assert_eq!(tree.key_rank(&7), Some(3));
    }
}
