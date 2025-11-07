//! B-Trees (Sections 18.1-18.3)
//!
//! A B-tree is a balanced search tree designed for scenarios in which the cost
//! of accessing secondary storage dominates. The implementation mirrors the
//! CLRS pseudocode, supporting search, insertion, and deletion while maintaining
//! the minimum degree (`t`) invariants.

use std::cmp::Ordering;

/// A single node in a B-tree
#[derive(Debug, Clone)]
pub struct BTreeNode<K: Ord, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
    pub children: Vec<Box<BTreeNode<K, V>>>,
    pub leaf: bool,
}

impl<K: Ord, V> BTreeNode<K, V> {
    fn new(leaf: bool) -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }

    fn is_full(&self, min_degree: usize) -> bool {
        self.keys.len() == 2 * min_degree - 1
    }

    fn search(&self, key: &K) -> Option<&V> {
        match self.keys.binary_search(key) {
            Ok(idx) => Some(&self.values[idx]),
            Err(idx) => {
                if self.leaf {
                    None
                } else {
                    self.children[idx].search(key)
                }
            }
        }
    }

    fn insert_non_full(&mut self, key: K, value: V, min_degree: usize) {
        match self.keys.binary_search(&key) {
            Ok(idx) => {
                self.values[idx] = value;
            }
            Err(mut idx) => {
                if self.leaf {
                    self.keys.insert(idx, key);
                    self.values.insert(idx, value);
                } else {
                    if self.children[idx].is_full(min_degree) {
                        self.split_child(idx, min_degree);
                        match self.keys[idx].cmp(&key) {
                            Ordering::Less => idx += 1,
                            Ordering::Equal => {
                                self.values[idx] = value;
                                return;
                            }
                            Ordering::Greater => {}
                        }
                    }
                    self.children[idx].insert_non_full(key, value, min_degree);
                }
            }
        }
    }

    fn split_child(&mut self, idx: usize, min_degree: usize) {
        let (up_key, up_value, new_child) = {
            let child = self.children[idx].as_mut();
            let mut split_keys = child.keys.split_off(min_degree - 1);
            let mut split_values = child.values.split_off(min_degree - 1);

            let promoted_key = split_keys.remove(0);
            let promoted_value = split_values.remove(0);

            let mut new_node = BTreeNode::new(child.leaf);
            new_node.keys = split_keys;
            new_node.values = split_values;

            if !child.leaf {
                let split_children = child.children.split_off(min_degree);
                new_node.children = split_children;
            }

            (promoted_key, promoted_value, Box::new(new_node))
        };

        self.keys.insert(idx, up_key);
        self.values.insert(idx, up_value);
        self.children.insert(idx + 1, new_child);
    }

    fn delete(&mut self, key: &K, min_degree: usize) -> Option<V> {
        match self.keys.binary_search(key) {
            Ok(idx) => {
                if self.leaf {
                    self.keys.remove(idx);
                    Some(self.values.remove(idx))
                } else {
                    self.delete_internal_key(idx, key, min_degree)
                }
            }
            Err(mut idx) => {
                if self.leaf {
                    None
                } else {
                    idx = self.ensure_child_has_min_keys(idx, min_degree);
                    self.children[idx].delete(key, min_degree)
                }
            }
        }
    }

    fn delete_internal_key(&mut self, idx: usize, key: &K, min_degree: usize) -> Option<V> {
        if self.children[idx].keys.len() >= min_degree {
            let (pred_key, pred_value) = self.children[idx].extract_predecessor(min_degree);
            let old_value = std::mem::replace(&mut self.values[idx], pred_value);
            self.keys[idx] = pred_key;
            Some(old_value)
        } else if self.children[idx + 1].keys.len() >= min_degree {
            let (succ_key, succ_value) = self.children[idx + 1].extract_successor(min_degree);
            let old_value = std::mem::replace(&mut self.values[idx], succ_value);
            self.keys[idx] = succ_key;
            Some(old_value)
        } else {
            self.merge_children(idx);
            self.children[idx].delete(key, min_degree)
        }
    }

    fn ensure_child_has_min_keys(&mut self, mut idx: usize, min_degree: usize) -> usize {
        if self.children[idx].keys.len() >= min_degree {
            return idx;
        }

        if idx > 0 && self.children[idx - 1].keys.len() >= min_degree {
            self.borrow_from_prev(idx);
        } else if idx + 1 < self.children.len() && self.children[idx + 1].keys.len() >= min_degree {
            self.borrow_from_next(idx);
        } else {
            if idx + 1 < self.children.len() {
                self.merge_children(idx);
            } else {
                self.merge_children(idx - 1);
                idx -= 1;
            }
        }
        idx
    }

    fn borrow_from_prev(&mut self, idx: usize) {
        let (left_slice, right_slice) = self.children.split_at_mut(idx);
        let child = &mut right_slice[0];
        let left_sibling = &mut left_slice[left_slice.len() - 1];

        let key_from_sibling = left_sibling
            .keys
            .pop()
            .expect("left sibling must have keys");
        let value_from_sibling = left_sibling
            .values
            .pop()
            .expect("left sibling must have values");

        let parent_key = std::mem::replace(&mut self.keys[idx - 1], key_from_sibling);
        let parent_value = std::mem::replace(&mut self.values[idx - 1], value_from_sibling);

        child.keys.insert(0, parent_key);
        child.values.insert(0, parent_value);

        if !left_sibling.leaf {
            let moved_child = left_sibling
                .children
                .pop()
                .expect("left sibling must have child to borrow");
            child.children.insert(0, moved_child);
        }
    }

    fn borrow_from_next(&mut self, idx: usize) {
        let (left_slice, right_slice) = self.children.split_at_mut(idx + 1);
        let child = &mut left_slice[left_slice.len() - 1];
        let right_sibling = &mut right_slice[0];

        let key_from_sibling = right_sibling.keys.remove(0);
        let value_from_sibling = right_sibling.values.remove(0);

        let parent_key = std::mem::replace(&mut self.keys[idx], key_from_sibling);
        let parent_value = std::mem::replace(&mut self.values[idx], value_from_sibling);

        child.keys.push(parent_key);
        child.values.push(parent_value);

        if !right_sibling.leaf {
            let moved_child = right_sibling.children.remove(0);
            child.children.push(moved_child);
        }
    }

    fn merge_children(&mut self, idx: usize) {
        let right_child = self.children.remove(idx + 1);
        let key = self.keys.remove(idx);
        let value = self.values.remove(idx);

        let left_child = self.children[idx].as_mut();
        left_child.keys.push(key);
        left_child.values.push(value);

        let mut right_child = *right_child;
        left_child.keys.extend(right_child.keys.drain(..));
        left_child.values.extend(right_child.values.drain(..));

        if !left_child.leaf {
            left_child.children.extend(right_child.children.drain(..));
        }
    }

    fn extract_predecessor(&mut self, min_degree: usize) -> (K, V) {
        if self.leaf {
            let key = self.keys.pop().expect("predecessor from empty leaf");
            let value = self.values.pop().expect("predecessor from empty leaf");
            (key, value)
        } else {
            let idx = self.ensure_child_has_min_keys(self.children.len() - 1, min_degree);
            self.children[idx].extract_predecessor(min_degree)
        }
    }

    fn extract_successor(&mut self, min_degree: usize) -> (K, V) {
        if self.leaf {
            let key = self.keys.remove(0);
            let value = self.values.remove(0);
            (key, value)
        } else {
            let idx = self.ensure_child_has_min_keys(0, min_degree);
            self.children[idx].extract_successor(min_degree)
        }
    }

    fn traverse<F>(&self, visitor: &mut F)
    where
        F: FnMut(&K, &V),
    {
        for i in 0..self.keys.len() {
            if !self.leaf {
                self.children[i].traverse(visitor);
            }
            visitor(&self.keys[i], &self.values[i]);
        }
        if !self.leaf {
            self.children[self.keys.len()].traverse(visitor);
        }
    }
}

/// B-tree structure parameterized by key and value types
#[derive(Debug, Clone)]
pub struct BTree<K: Ord, V> {
    pub root: Option<Box<BTreeNode<K, V>>>,
    pub min_degree: usize,
}

impl<K: Ord, V> BTree<K, V> {
    /// Creates a new empty B-tree with the given minimum degree `t`
    ///
    /// # Panics
    ///
    /// Panics if `min_degree < 2`, because a B-tree of degree 1 does not
    /// satisfy the structural constraints.
    pub fn new(min_degree: usize) -> Self {
        assert!(min_degree >= 2, "B-tree minimum degree must be at least 2");
        Self {
            root: None,
            min_degree,
        }
    }

    /// Returns the minimum degree `t` of the tree
    pub fn min_degree(&self) -> usize {
        self.min_degree
    }

    /// Checks whether the tree is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Searches for `key` in the B-tree, returning a reference to the value if found
    pub fn search(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|node| node.search(key))
    }

    /// Returns `true` if the B-tree contains `key`
    pub fn contains(&self, key: &K) -> bool {
        self.search(key).is_some()
    }

    /// Inserts the key-value pair into the B-tree
    ///
    /// If the key already exists, its value is updated.
    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            let mut root = BTreeNode::new(true);
            root.keys.push(key);
            root.values.push(value);
            self.root = Some(Box::new(root));
            return;
        }

        let min_degree = self.min_degree;
        let mut root = self.root.take().expect("root must exist");

        if root.is_full(min_degree) {
            let mut new_root = BTreeNode::new(false);
            new_root.children.push(root);
            new_root.split_child(0, min_degree);
            new_root.insert_non_full(key, value, min_degree);
            self.root = Some(Box::new(new_root));
        } else {
            root.insert_non_full(key, value, min_degree);
            self.root = Some(root);
        }
    }

    /// Deletes `key` from the B-tree, returning the stored value if it existed
    pub fn delete(&mut self, key: &K) -> Option<V> {
        let mut root = match self.root.take() {
            None => return None,
            Some(root) => root,
        };

        let result = root.delete(key, self.min_degree);

        if root.keys.is_empty() {
            if root.leaf {
                self.root = None;
            } else {
                self.root = Some(root.children.remove(0));
            }
        } else {
            self.root = Some(root);
        }

        result
    }

    /// Applies `visitor` to all key-value pairs in sorted (in-order) order
    pub fn traverse_inorder<F>(&self, mut visitor: F)
    where
        F: FnMut(&K, &V),
    {
        if let Some(root) = &self.root {
            root.traverse(&mut visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_insert_search() {
        let mut tree: BTree<i32, i32> = BTree::new(3);
        for i in 0..100 {
            tree.insert(i, i * 10);
        }

        for i in 0..100 {
            assert_eq!(tree.search(&i), Some(&(i * 10)));
        }

        assert_eq!(tree.search(&200), None);
    }

    #[test]
    fn test_btree_update_value() {
        let mut tree: BTree<i32, &str> = BTree::new(3);
        tree.insert(42, "first");
        tree.insert(42, "second");
        assert_eq!(tree.search(&42), Some(&"second"));
        assert!(tree.contains(&42));
    }

    #[test]
    fn test_btree_delete_sequence() {
        let mut tree: BTree<i32, i32> = BTree::new(3);
        for i in 0..128 {
            tree.insert(i, i);
        }

        for i in (0..128).step_by(2) {
            let removed = tree.delete(&i);
            assert_eq!(removed, Some(i));
            assert_eq!(tree.search(&i), None);
        }

        for i in 0..128 {
            if i % 2 == 1 {
                assert_eq!(tree.search(&i), Some(&i));
            }
        }
    }

    #[test]
    fn test_btree_delete_all() {
        let mut tree: BTree<i32, i32> = BTree::new(2);
        for i in 0..50 {
            tree.insert(i, i);
        }

        for i in 0..50 {
            assert_eq!(tree.delete(&i), Some(i));
        }

        assert!(tree.is_empty());
        assert_eq!(tree.search(&5), None);
    }

    #[test]
    fn test_btree_inorder_traversal() {
        let mut tree: BTree<i32, i32> = BTree::new(3);
        let keys = [20, 5, 1, 25, 15, 30, 10, 40, 50, 60, 70, 80];

        for &key in &keys {
            tree.insert(key, key);
        }

        let mut collected = Vec::new();
        tree.traverse_inorder(|k, _| collected.push(*k));

        let mut sorted = keys.to_vec();
        sorted.sort();
        assert_eq!(collected, sorted);
    }
}
