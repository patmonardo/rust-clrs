//! Disjoint Set (Union-Find) implementation (Section 21)
//!
//! This translation follows the CLRS union-find structure, providing
//! `make_set`, `find_set`, and `union` operations with union by rank and
//! path compression.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct Node<T> {
    parent: usize,
    rank: usize,
    value: T,
}

/// Union-Find structure over values of type `T`.
#[derive(Debug, Clone, Default)]
pub struct DisjointSet<T>
where
    T: Eq + Hash + Clone,
{
    nodes: Vec<Node<T>>,
    index: HashMap<T, usize>,
}

impl<T> DisjointSet<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates an empty disjoint set structure.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Inserts a new singleton set containing `value`.
    ///
    /// Returns `false` if the value was already present.
    pub fn make_set(&mut self, value: T) -> bool {
        if self.index.contains_key(&value) {
            return false;
        }

        let id = self.nodes.len();
        self.nodes.push(Node {
            parent: id,
            rank: 0,
            value: value.clone(),
        });
        self.index.insert(value, id);
        true
    }

    /// Finds the representative of the set containing `value`, applying path compression.
    pub fn find_set(&mut self, value: &T) -> Option<T> {
        let id = *self.index.get(value)?;
        let root = self.find(id);
        Some(self.nodes[root].value.clone())
    }

    /// Checks whether two values belong to the same set.
    pub fn are_connected(&mut self, x: &T, y: &T) -> bool {
        match (self.index.get(x).cloned(), self.index.get(y).cloned()) {
            (Some(ix), Some(iy)) => self.find(ix) == self.find(iy),
            _ => false,
        }
    }

    /// Performs the union of the sets containing `x` and `y`.
    ///
    /// Returns `true` if the sets were distinct and `false` if they were already merged
    /// or if either element is missing from the structure.
    pub fn union(&mut self, x: &T, y: &T) -> bool {
        let (Some(mut x_id), Some(mut y_id)) =
            (self.index.get(x).cloned(), self.index.get(y).cloned())
        else {
            return false;
        };

        x_id = self.find(x_id);
        y_id = self.find(y_id);

        if x_id == y_id {
            return false;
        }

        self.link(x_id, y_id);
        true
    }

    fn find(&mut self, id: usize) -> usize {
        if self.nodes[id].parent != id {
            let root = self.find(self.nodes[id].parent);
            self.nodes[id].parent = root;
        }
        self.nodes[id].parent
    }

    fn link(&mut self, x_root: usize, y_root: usize) {
        if self.nodes[x_root].rank > self.nodes[y_root].rank {
            self.nodes[y_root].parent = x_root;
        } else if self.nodes[x_root].rank < self.nodes[y_root].rank {
            self.nodes[x_root].parent = y_root;
        } else {
            self.nodes[y_root].parent = x_root;
            self.nodes[x_root].rank += 1;
        }
    }

    /// Returns the number of disjoint sets currently stored.
    pub fn set_count(&mut self) -> usize {
        let mut roots = HashMap::new();
        for id in 0..self.nodes.len() {
            let root = self.find(id);
            roots.entry(root).or_insert(());
        }
        roots.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_set_and_find() {
        let mut ds = DisjointSet::new();
        ds.make_set("a");
        ds.make_set("b");
        assert_eq!(ds.find_set(&"a"), Some("a"));
        assert_eq!(ds.find_set(&"b"), Some("b"));
        assert_eq!(ds.find_set(&"c"), None);
    }

    #[test]
    fn test_union_and_connected() {
        let mut ds = DisjointSet::new();
        for value in [1, 2, 3, 4, 5] {
            ds.make_set(value);
        }

        assert!(ds.union(&1, &2));
        assert!(ds.are_connected(&1, &2));
        assert!(!ds.are_connected(&1, &3));

        ds.union(&3, &4);
        ds.union(&2, &3);
        assert!(ds.are_connected(&1, &4));
        assert!(ds.union(&4, &5));
        assert!(ds.are_connected(&1, &5));
        assert!(!ds.union(&1, &5)); // already united
    }

    #[test]
    fn test_path_compression_effectiveness() {
        let mut ds = DisjointSet::new();
        for value in 0..10 {
            ds.make_set(value);
        }

        // Create a chain by uniting sequentially
        for value in 1..10 {
            ds.union(&(value - 1), &value);
        }

        // Find operations should compress paths
        for value in 0..10 {
            let representative = ds.find_set(&value).unwrap();
            assert_eq!(representative, 0);
        }

        // After path compression, every node should have same parent
        let unique_roots: std::collections::HashSet<_> = (0..10).map(|id| ds.find(id)).collect();
        assert_eq!(unique_roots.len(), 1);
    }
}
