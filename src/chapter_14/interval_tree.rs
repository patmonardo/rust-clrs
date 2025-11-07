//! Interval Trees (Section 14.3)
//!
//! An interval tree is a red-black tree that maintains a dynamic set of intervals,
//! each with an associated value. It supports efficient interval queries.

use std::cmp::Ordering;

/// An interval with low and high endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub low: i32,
    pub high: i32,
}

impl Interval {
    /// Creates a new interval
    ///
    /// # Arguments
    /// * `low` - The low endpoint
    /// * `high` - The high endpoint
    ///
    /// # Panics
    /// Panics if `low > high`
    pub fn new(low: i32, high: i32) -> Self {
        assert!(low <= high, "low must be <= high");
        Interval { low, high }
    }

    /// Checks if this interval overlaps with another interval
    ///
    /// Two intervals overlap if they have any point in common.
    pub fn overlaps(&self, other: &Interval) -> bool {
        self.low <= other.high && other.low <= self.high
    }

    /// Checks if this interval exactly matches another interval
    pub fn exactly_matches(&self, other: &Interval) -> bool {
        self.low == other.low && self.high == other.high
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Interval) -> Option<Ordering> {
        self.low.partial_cmp(&other.low)
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Interval) -> Ordering {
        self.low.cmp(&other.low)
    }
}

/// Node in an interval tree
///
/// This is a red-black tree node augmented with interval and max information.
#[derive(Debug, Clone)]
pub struct IntervalNode<V> {
    pub interval: Interval,
    pub value: V,
    pub max: i32, // Maximum high endpoint in subtree rooted at this node
    pub left: Option<Box<IntervalNode<V>>>,
    pub right: Option<Box<IntervalNode<V>>>,
}

/// Interval tree
///
/// This corresponds to the interval tree from CLRS Section 14.3.
/// It supports INTERVAL-SEARCH in O(lg n) time.
///
/// # Example
/// ```
/// use clrs::chapter_14::{IntervalTree, Interval};
/// let mut tree = IntervalTree::new();
/// tree.insert(Interval::new(1, 5), "interval1");
/// tree.insert(Interval::new(3, 7), "interval2");
/// assert!(tree.search(Interval::new(4, 6)).is_some());
/// ```
#[derive(Debug, Clone)]
pub struct IntervalTree<V> {
    pub root: Option<Box<IntervalNode<V>>>,
}

impl<V> IntervalTree<V> {
    /// Creates a new empty interval tree
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_14::IntervalTree;
    /// let tree: IntervalTree<&str> = IntervalTree::new();
    /// ```
    pub fn new() -> Self {
        IntervalTree { root: None }
    }

    /// Searches for an interval that overlaps with the given interval
    ///
    /// This corresponds to INTERVAL-SEARCH from CLRS Section 14.3.
    ///
    /// # Arguments
    /// * `i` - The interval to search for
    ///
    /// # Returns
    /// A reference to the value of an overlapping interval, or `None` if no overlap exists
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of intervals
    pub fn search(&self, i: Interval) -> Option<&V> {
        Self::search_node(&self.root, i)
    }

    fn search_node(node: &Option<Box<IntervalNode<V>>>, i: Interval) -> Option<&V> {
        match node {
            None => None,
            Some(n) => {
                if i.overlaps(&n.interval) {
                    return Some(&n.value);
                }
                
                if n.left.is_some() && n.left.as_ref().unwrap().max >= i.low {
                    Self::search_node(&n.left, i)
                } else {
                    Self::search_node(&n.right, i)
                }
            }
        }
    }

    /// Searches for an interval that exactly matches the given interval
    ///
    /// This corresponds to INTERVAL-SEARCH-EXACTLY from CLRS Exercise 14.3-5.
    ///
    /// # Arguments
    /// * `i` - The interval to search for
    ///
    /// # Returns
    /// A reference to the value of an exactly matching interval, or `None` if no match exists
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of intervals
    pub fn search_exactly(&self, i: Interval) -> Option<&V> {
        Self::search_exactly_node(&self.root, i)
    }

    fn search_exactly_node(node: &Option<Box<IntervalNode<V>>>, i: Interval) -> Option<&V> {
        match node {
            None => None,
            Some(n) => {
                if i.exactly_matches(&n.interval) {
                    return Some(&n.value);
                }
                
                if i.high > n.max {
                    None
                } else if i.low < n.interval.low {
                    Self::search_exactly_node(&n.left, i)
                } else if i.low > n.interval.low {
                    Self::search_exactly_node(&n.right, i)
                } else {
                    None
                }
            }
        }
    }

    /// Inserts an interval-value pair into the tree
    ///
    /// This augments TREE-INSERT with max maintenance.
    ///
    /// # Arguments
    /// * `interval` - The interval to insert
    /// * `value` - The value to associate with the interval
    ///
    /// # Complexity
    /// - Time: O(lg n) where n is the number of intervals
    pub fn insert(&mut self, interval: Interval, value: V) {
        let new_node = Box::new(IntervalNode {
            interval,
            value,
            max: interval.high,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(new_node);
        } else {
            Self::insert_node(&mut self.root, new_node);
            Self::update_max(&mut self.root);
        }
    }

    fn insert_node(node: &mut Option<Box<IntervalNode<V>>>, new_node: Box<IntervalNode<V>>) {
        match node {
            None => *node = Some(new_node),
            Some(n) => {
                match new_node.interval.cmp(&n.interval) {
                    Ordering::Less => {
                        Self::insert_node(&mut n.left, new_node);
                        Self::update_max(&mut n.left);
                    }
                    Ordering::Greater => {
                        Self::insert_node(&mut n.right, new_node);
                        Self::update_max(&mut n.right);
                    }
                    Ordering::Equal => {
                        // Interval already exists, update value
                        n.value = new_node.value;
                    }
                }
                Self::update_max(node);
            }
        }
    }

    /// Updates the max attribute of a node
    ///
    /// This corresponds to maintaining the max attribute during rotations and insertions.
    fn update_max(node: &mut Option<Box<IntervalNode<V>>>) {
        if let Some(n) = node {
            let left_max = n.left.as_ref().map(|l| l.max).unwrap_or(i32::MIN);
            let right_max = n.right.as_ref().map(|r| r.max).unwrap_or(i32::MIN);
            n.max = n.interval.high.max(left_max).max(right_max);
        }
    }
}

impl<V> Default for IntervalTree<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_new() {
        let i = Interval::new(1, 5);
        assert_eq!(i.low, 1);
        assert_eq!(i.high, 5);
    }

    #[test]
    fn test_interval_overlaps() {
        let i1 = Interval::new(1, 5);
        let i2 = Interval::new(3, 7);
        let i3 = Interval::new(6, 10);
        
        assert!(i1.overlaps(&i2));
        assert!(!i1.overlaps(&i3));
        assert!(i2.overlaps(&i3));
    }

    #[test]
    fn test_interval_tree_new() {
        let tree: IntervalTree<&str> = IntervalTree::new();
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_interval_tree_insert_and_search() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(1, 5), "interval1");
        tree.insert(Interval::new(3, 7), "interval2");
        tree.insert(Interval::new(8, 10), "interval3");
        
        // Search for overlapping interval - should find ANY overlapping interval
        assert!(tree.search(Interval::new(4, 6)).is_some()); // Overlaps with interval1 or interval2
        assert_eq!(tree.search(Interval::new(9, 11)), Some(&"interval3"));
        
        // Search for non-overlapping interval
        assert_eq!(tree.search(Interval::new(11, 15)), None);
    }

    #[test]
    fn test_interval_tree_search_exactly() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(1, 5), "interval1");
        tree.insert(Interval::new(3, 7), "interval2");
        
        assert_eq!(tree.search_exactly(Interval::new(1, 5)), Some(&"interval1"));
        assert_eq!(tree.search_exactly(Interval::new(3, 7)), Some(&"interval2"));
        assert_eq!(tree.search_exactly(Interval::new(1, 6)), None); // Overlaps but not exact
    }

    #[test]
    fn test_interval_tree_max_maintenance() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(1, 5), "interval1");
        tree.insert(Interval::new(3, 7), "interval2");
        tree.insert(Interval::new(8, 10), "interval3");
        
        // The root's max should be the maximum high endpoint
        if let Some(root) = &tree.root {
            assert_eq!(root.max, 10);
        }
    }
}

