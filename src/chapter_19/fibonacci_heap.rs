//! Fibonacci Heaps (Section 19)
//!
//! This file contains a faithful, safe Rust implementation of the Fibonacci
//! heap structure described in CLRS. The implementation follows the textbook's
//! structure closely, while wrapping node pointers in safe `Rc<RefCell<_>>`
//! handles. Only the operations required by later chapters are provided:
//! creation, insertion, union, finding the minimum, extracting the minimum,
//! and decreasing a key.

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

type NodeRef<K, V> = Rc<RefCell<FibNode<K, V>>>;

#[derive(Debug)]
struct FibNode<K: Ord + Clone, V> {
    key: K,
    value: Option<V>,
    degree: usize,
    mark: bool,
    parent: Option<Weak<RefCell<FibNode<K, V>>>>,
    child: Option<NodeRef<K, V>>,
    left: Option<NodeRef<K, V>>,
    right: Option<NodeRef<K, V>>,
}

impl<K: Ord + Clone, V> FibNode<K, V> {
    fn new(key: K, value: V) -> NodeRef<K, V> {
        let node = Rc::new(RefCell::new(Self {
            key,
            value: Some(value),
            degree: 0,
            mark: false,
            parent: None,
            child: None,
            left: None,
            right: None,
        }));

        {
            let mut node_mut = node.borrow_mut();
            node_mut.left = Some(node.clone());
            node_mut.right = Some(node.clone());
        }

        node
    }

    fn as_handle(node: &NodeRef<K, V>) -> FibNodeHandle<K, V> {
        FibNodeHandle {
            node: Rc::downgrade(node),
        }
    }
}

/// A lightweight handle that clients can store in order to call
/// `decrease_key` later on a particular node.
#[derive(Clone)]
pub struct FibNodeHandle<K: Ord + Clone, V> {
    node: Weak<RefCell<FibNode<K, V>>>,
}

impl<K: Ord + Clone, V> FibNodeHandle<K, V> {
    fn upgrade(&self) -> Option<NodeRef<K, V>> {
        self.node.upgrade()
    }
}

impl<K: Ord + Clone + Debug, V: Debug> Debug for FibNodeHandle<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.node.upgrade() {
            Some(node) => {
                let node_ref = node.borrow();
                f.debug_struct("FibNodeHandle")
                    .field("key", &node_ref.key)
                    .field("has_value", &node_ref.value.is_some())
                    .field("degree", &node_ref.degree)
                    .finish()
            }
            None => write!(f, "FibNodeHandle(invalid)"),
        }
    }
}

/// Fibonacci Heap implementation with safe ownership semantics.
#[derive(Debug, Default)]
pub struct FibonacciHeap<K: Ord + Clone, V> {
    min: Option<NodeRef<K, V>>,
    total_nodes: usize,
}

impl<K: Ord + Clone, V> FibonacciHeap<K, V> {
    /// Creates an empty heap.
    pub fn new() -> Self {
        Self {
            min: None,
            total_nodes: 0,
        }
    }

    /// Returns `true` if the heap has no elements.
    pub fn is_empty(&self) -> bool {
        self.total_nodes == 0
    }

    /// Returns the number of nodes currently stored in the heap.
    pub fn len(&self) -> usize {
        self.total_nodes
    }

    /// Returns the minimum key and value pair without removing it.
    pub fn minimum(&self) -> Option<(K, V)>
    where
        V: Clone,
    {
        self.min.as_ref().and_then(|node| {
            let node_ref = node.borrow();
            node_ref
                .value
                .as_ref()
                .cloned()
                .map(|value| (node_ref.key.clone(), value))
        })
    }

    /// Inserts a new key-value pair and returns a node handle that can be used
    /// later in `decrease_key`.
    pub fn insert(&mut self, key: K, value: V) -> FibNodeHandle<K, V> {
        let node = FibNode::new(key.clone(), value);
        self.total_nodes += 1;

        self.add_to_root_list(node.clone());

        FibNode::as_handle(&node)
    }

    /// Melds two heaps together, returning the resulting heap.
    pub fn union(mut self, mut other: Self) -> Self {
        if self.min.is_none() {
            return other;
        }
        if other.min.is_none() {
            return self;
        }

        self.concatenate_root_lists(other.min.take().unwrap());
        self.total_nodes += other.total_nodes;

        self
    }

    /// Extracts the node with minimum key from the heap.
    pub fn extract_min(&mut self) -> Option<(K, V)> {
        let min_node = self.min.take()?;
        let extracted_key = min_node.borrow().key.clone();
        let extracted_value = min_node
            .borrow_mut()
            .value
            .take()
            .expect("node should still hold a value");

        let children = self.collect_children(&min_node);
        let replacement = self.remove_from_root_list(&min_node);
        self.min = replacement;

        for child in children {
            child.borrow_mut().parent = None;
            child.borrow_mut().mark = false;
            self.add_to_root_list(child);
        }

        self.total_nodes -= 1;

        if self.min.is_some() {
            self.consolidate();
        }

        Some((extracted_key, extracted_value))
    }

    /// Decreases the key for a given node handle.
    pub fn decrease_key(&mut self, handle: &FibNodeHandle<K, V>, new_key: K) {
        let node_rc = handle
            .upgrade()
            .expect("Cannot decrease key on a node that no longer exists");

        {
            let mut node = node_rc.borrow_mut();
            if new_key > node.key {
                panic!("new key is greater than current key");
            }
            node.key = new_key.clone();
        }

        let parent = node_rc
            .borrow()
            .parent
            .as_ref()
            .and_then(|weak| weak.upgrade());

        if let Some(parent_rc) = parent {
            if node_rc.borrow().key < parent_rc.borrow().key {
                self.cut(node_rc.clone(), parent_rc.clone());
                self.cascading_cut(parent_rc);
            }
        }

        if let Some(min_node) = &self.min {
            if node_rc.borrow().key < min_node.borrow().key {
                self.min = Some(node_rc);
            }
        } else {
            self.min = Some(node_rc);
        }
    }

    fn add_to_root_list(&mut self, node: NodeRef<K, V>) {
        {
            let mut node_mut = node.borrow_mut();
            node_mut.parent = None;
            node_mut.mark = false;
        }

        if let Some(min_node) = &self.min {
            self.insert_into_list(min_node, &node);
            let node_key = node.borrow().key.clone();
            let min_key = min_node.borrow().key.clone();
            if node_key < min_key {
                self.min = Some(node);
            }
        } else {
            {
                let mut node_mut = node.borrow_mut();
                node_mut.left = Some(node.clone());
                node_mut.right = Some(node.clone());
            }
            self.min = Some(node);
        }
    }

    fn insert_into_list(&self, reference: &NodeRef<K, V>, node: &NodeRef<K, V>) {
        let right = reference.borrow().right.as_ref().unwrap().clone();

        {
            let mut node_mut = node.borrow_mut();
            node_mut.left = Some(reference.clone());
            node_mut.right = Some(right.clone());
        }

        {
            reference.borrow_mut().right = Some(node.clone());
        }

        {
            right.borrow_mut().left = Some(node.clone());
        }
    }

    fn concatenate_root_lists(&mut self, other_min: NodeRef<K, V>) {
        if let Some(self_min) = &self.min {
            let self_right = self_min.borrow().right.as_ref().unwrap().clone();
            let other_left = other_min.borrow().left.as_ref().unwrap().clone();

            {
                self_min.borrow_mut().right = Some(other_min.clone());
            }

            {
                other_min.borrow_mut().left = Some(self_min.clone());
            }

            {
                other_left.borrow_mut().right = Some(self_right.clone());
            }

            {
                self_right.borrow_mut().left = Some(other_left);
            }

            let other_key = other_min.borrow().key.clone();
            let self_key = self_min.borrow().key.clone();
            if other_key < self_key {
                self.min = Some(other_min);
            }
        } else {
            self.min = Some(other_min);
        }
    }

    fn remove_from_root_list(&mut self, node: &NodeRef<K, V>) -> Option<NodeRef<K, V>> {
        let left = node.borrow().left.as_ref().unwrap().clone();
        let right = node.borrow().right.as_ref().unwrap().clone();

        if Rc::ptr_eq(&left, node) {
            // Singleton
            node.borrow_mut().left = Some(node.clone());
            node.borrow_mut().right = Some(node.clone());
            None
        } else {
            left.borrow_mut().right = Some(right.clone());
            right.borrow_mut().left = Some(left.clone());
            node.borrow_mut().left = Some(node.clone());
            node.borrow_mut().right = Some(node.clone());
            Some(right)
        }
    }

    fn collect_children(&self, node: &NodeRef<K, V>) -> Vec<NodeRef<K, V>> {
        let mut children = Vec::new();
        let child_opt = node.borrow_mut().child.take();

        if let Some(child) = child_opt {
            let mut current = child.clone();
            loop {
                let next = current.borrow().right.as_ref().unwrap().clone();
                children.push(current.clone());
                {
                    let mut node_mut = current.borrow_mut();
                    node_mut.left = Some(current.clone());
                    node_mut.right = Some(current.clone());
                }
                if Rc::ptr_eq(&next, &child) {
                    break;
                }
                current = next;
            }
        }

        children
    }

    fn consolidate(&mut self) {
        let mut roots = Vec::new();
        if let Some(min_node) = &self.min {
            roots.push(min_node.clone());
            let mut current = { min_node.borrow().right.as_ref().unwrap().clone() };
            while !Rc::ptr_eq(&current, min_node) {
                roots.push(current.clone());
                let next = { current.borrow().right.as_ref().unwrap().clone() };
                current = next;
            }
        }

        let mut degree_table: Vec<Option<NodeRef<K, V>>> = vec![None; self.approx_degree_bound()];

        for node in roots {
            let mut x = node;
            let mut d = x.borrow().degree;

            loop {
                if d >= degree_table.len() {
                    degree_table.resize(d + 1, None);
                }

                if degree_table[d].is_none() {
                    degree_table[d] = Some(x.clone());
                    break;
                }

                let mut y = degree_table[d].take().unwrap();
                if x.borrow().key > y.borrow().key {
                    std::mem::swap(&mut x, &mut y);
                }
                self.link(y, x.clone());
                d = x.borrow().degree;
            }
        }

        self.min = None;
        for entry in degree_table.into_iter().flatten() {
            if self.min.is_none() {
                // Reinitialize the root list with the first entry
                entry.borrow_mut().left = Some(entry.clone());
                entry.borrow_mut().right = Some(entry.clone());
                self.min = Some(entry.clone());
            } else {
                self.add_to_root_list(entry.clone());
                if entry.borrow().key < self.min.as_ref().unwrap().borrow().key {
                    self.min = Some(entry);
                }
            }
        }
    }

    fn link(&self, child: NodeRef<K, V>, parent: NodeRef<K, V>) {
        // Remove child from root list.
        {
            let left = child.borrow().left.as_ref().unwrap().clone();
            let right = child.borrow().right.as_ref().unwrap().clone();
            left.borrow_mut().right = Some(right.clone());
            right.borrow_mut().left = Some(left);
        }

        {
            let mut child_mut = child.borrow_mut();
            child_mut.parent = Some(Rc::downgrade(&parent));
            child_mut.mark = false;
        }

        let maybe_existing_child = {
            let mut parent_mut = parent.borrow_mut();
            match parent_mut.child.take() {
                Some(existing_child) => {
                    parent_mut.child = Some(existing_child.clone());
                    Some(existing_child)
                }
                None => {
                    child.borrow_mut().left = Some(child.clone());
                    child.borrow_mut().right = Some(child.clone());
                    parent_mut.child = Some(child.clone());
                    parent_mut.degree += 1;
                    None
                }
            }
        };

        if let Some(existing_child) = maybe_existing_child {
            self.insert_into_list(&existing_child, &child);
            parent.borrow_mut().degree += 1;
        }
    }

    fn cut(&mut self, node: NodeRef<K, V>, parent: NodeRef<K, V>) {
        self.remove_from_child_list(&parent, &node);
        parent.borrow_mut().degree -= 1;

        self.add_to_root_list(node.clone());
    }

    fn remove_from_child_list(&self, parent: &NodeRef<K, V>, node: &NodeRef<K, V>) {
        let singleton = {
            let node_borrow = node.borrow();
            Rc::ptr_eq(node_borrow.right.as_ref().unwrap(), node)
        };

        if singleton {
            parent.borrow_mut().child = None;
        } else {
            let left = node.borrow().left.as_ref().unwrap().clone();
            let right = node.borrow().right.as_ref().unwrap().clone();
            left.borrow_mut().right = Some(right.clone());
            right.borrow_mut().left = Some(left.clone());

            let update_child = {
                let parent_child = parent.borrow().child.as_ref().unwrap().clone();
                Rc::ptr_eq(&parent_child, node)
            };

            if update_child {
                parent.borrow_mut().child = Some(right.clone());
            }
        }

        {
            let mut node_mut = node.borrow_mut();
            node_mut.left = Some(node.clone());
            node_mut.right = Some(node.clone());
            node_mut.parent = None;
        }
    }

    fn cascading_cut(&mut self, node: NodeRef<K, V>) {
        if let Some(parent_weak) = node.borrow().parent.clone() {
            if let Some(parent) = parent_weak.upgrade() {
                if !node.borrow().mark {
                    node.borrow_mut().mark = true;
                } else {
                    self.cut(node.clone(), parent.clone());
                    self.cascading_cut(parent);
                }
            }
        }
    }

    fn approx_degree_bound(&self) -> usize {
        let n = self.total_nodes.max(1) as f64;
        n.log2().ceil() as usize + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_sorted<K: Ord + Clone, V>(mut heap: FibonacciHeap<K, V>) -> Vec<K> {
        let mut keys = Vec::new();
        while let Some((key, _)) = heap.extract_min() {
            keys.push(key);
        }
        keys
    }

    #[test]
    fn test_insert_and_minimum() {
        let mut heap = FibonacciHeap::new();
        heap.insert(7, "seven");
        heap.insert(3, "three");
        heap.insert(5, "five");

        let (min_key, min_value) = heap.minimum().unwrap();
        assert_eq!(min_key, 3);
        assert_eq!(min_value, "three");
        assert_eq!(heap.len(), 3);
    }

    #[test]
    fn test_extract_min_returns_sorted_keys() {
        let mut heap = FibonacciHeap::new();
        for key in [7, 3, 5, 2, 8, 1, 4, 6] {
            heap.insert(key, key * 10);
        }

        let sorted_keys = collect_sorted(heap);
        assert_eq!(sorted_keys, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_decrease_key_and_extract() {
        let mut heap = FibonacciHeap::new();
        let handles: Vec<_> = (10..20).map(|key| heap.insert(key, key * 2)).collect();

        heap.decrease_key(&handles[5], 1); // decrease key for original key 15
        heap.decrease_key(&handles[7], 0); // decrease key for original key 17

        assert_eq!(heap.extract_min(), Some((0, 34))); // original value 17 * 2
        assert_eq!(heap.extract_min(), Some((1, 30))); // original value 15 * 2
    }

    #[test]
    fn test_union_operation() {
        let mut heap_a = FibonacciHeap::new();
        heap_a.insert(5, "a5");
        heap_a.insert(9, "a9");

        let mut heap_b = FibonacciHeap::new();
        heap_b.insert(2, "b2");
        heap_b.insert(8, "b8");

        let union_heap = heap_a.union(heap_b);
        assert_eq!(union_heap.len(), 4);
        let keys = collect_sorted(union_heap);
        assert_eq!(keys, vec![2, 5, 8, 9]);
    }
}
