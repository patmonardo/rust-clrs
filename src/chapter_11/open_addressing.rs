//! Open Addressing (Section 11.4)
//!
//! Hash tables that resolve collisions by open addressing methods:
//! linear probing, quadratic probing, and double hashing.

use std::hash::Hash;

/// Marker for deleted slots in open addressing
#[derive(Debug, Clone, PartialEq)]
pub enum Slot<K, V> {
    Empty,
    Deleted,
    Occupied(K, V),
}

/// Hash table with open addressing
///
/// This corresponds to the hash table implementation from CLRS Section 11.4.
/// Collisions are resolved by probing through the table.
///
/// # Example
/// ```
/// use clrs::chapter_11::open_addressing::{OpenAddressingHashTable, ProbeType};
/// let mut table = OpenAddressingHashTable::new(11, ProbeType::Linear, |k, m| k % m, None);
/// table.insert(42, "value");
/// assert_eq!(table.search(42), Some(&"value"));
/// ```
#[derive(Debug, Clone)]
pub struct OpenAddressingHashTable<K: PartialEq + Clone + Hash, V> {
    arr: Vec<Slot<K, V>>,
    size: usize,
    probe_type: ProbeType,
    hash_fn1: fn(usize, usize) -> usize,
    hash_fn2: Option<fn(usize, usize) -> usize>,
}

/// Type of probing method
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProbeType {
    /// Linear probing: h(k, i) = (h'(k) + i) mod m
    Linear,
    /// Quadratic probing: h(k, i) = (h'(k) + c1*i + c2*i^2) mod m
    Quadratic { c1: usize, c2: usize },
    /// Double hashing: h(k, i) = (h1(k) + i*h2(k)) mod m
    DoubleHashing,
}

impl<K: PartialEq + Clone + Hash, V> OpenAddressingHashTable<K, V> {
    /// Creates a new hash table with open addressing
    ///
    /// # Arguments
    /// * `m` - The size of the hash table
    /// * `probe_type` - The type of probing to use
    /// * `hash_fn1` - The primary hash function
    /// * `hash_fn2` - Optional secondary hash function (required for double hashing)
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_11::open_addressing::{OpenAddressingHashTable, ProbeType};
    /// let table: OpenAddressingHashTable<usize, i32> = OpenAddressingHashTable::new(
    ///     11,
    ///     ProbeType::Linear,
    ///     |k, m| k % m,
    ///     None
    /// );
    /// ```
    pub fn new(
        m: usize,
        probe_type: ProbeType,
        hash_fn1: fn(usize, usize) -> usize,
        hash_fn2: Option<fn(usize, usize) -> usize>,
    ) -> Self {
        let mut arr = Vec::with_capacity(m);
        arr.resize_with(m, || Slot::Empty);
        OpenAddressingHashTable {
            arr,
            size: m,
            probe_type,
            hash_fn1,
            hash_fn2,
        }
    }

    /// Computes the probe sequence for key `k` at probe number `i`
    fn probe(&self, k: &K, i: usize) -> usize {
        // Convert key to usize for hashing (assuming keys can be converted)
        // In practice, we'd use a proper hash function, but for simplicity
        // we'll require K: Into<usize> or use a hash function
        let k_hash = self.key_to_hash(k);
        let h1 = (self.hash_fn1)(k_hash, self.size);
        match self.probe_type {
            ProbeType::Linear => (h1 + i) % self.size,
            ProbeType::Quadratic { c1, c2 } => {
                (h1 + c1 * i + c2 * i * i) % self.size
            }
            ProbeType::DoubleHashing => {
                let h2 = self.hash_fn2
                    .expect("Double hashing requires hash_fn2")
                    (k_hash, self.size);
                (h1 + i * h2) % self.size
            }
        }
    }

    /// Helper to convert key to hash value
    fn key_to_hash(&self, k: &K) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Searches for an element with key `k`
    ///
    /// This corresponds to HASH-SEARCH from CLRS Section 11.4.
    ///
    /// # Arguments
    /// * `k` - The key to search for
    ///
    /// # Returns
    /// A reference to the value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(1/(1-α)) expected for unsuccessful search
    pub fn search(&self, k: &K) -> Option<&V> {
        let mut i = 0;
        loop {
            let j = self.probe(k, i);
            match &self.arr[j] {
                Slot::Empty => return None,
                Slot::Deleted => {
                    i += 1;
                    if i >= self.size {
                        return None;
                    }
                }
                Slot::Occupied(key, value) => {
                    if key == k {
                        return Some(value);
                    }
                    i += 1;
                    if i >= self.size {
                        return None;
                    }
                }
            }
        }
    }

    /// Inserts an element with key `k` and value `v`
    ///
    /// This corresponds to HASH-INSERT from CLRS Section 11.4.
    ///
    /// # Arguments
    /// * `k` - The key
    /// * `v` - The value to insert
    ///
    /// # Returns
    /// The index where the element was inserted, or error if table is full
    ///
    /// # Complexity
    /// - Time: O(1/(1-α)) expected
    pub fn insert(&mut self, k: K, v: V) -> Result<usize, &'static str> {
        let mut i = 0;
        loop {
            let j = self.probe(&k, i);
            match &self.arr[j] {
                Slot::Empty | Slot::Deleted => {
                    self.arr[j] = Slot::Occupied(k, v);
                    return Ok(j);
                }
                Slot::Occupied(key, _) => {
                    if key == &k {
                        // Update existing key
                        self.arr[j] = Slot::Occupied(k, v);
                        return Ok(j);
                    }
                    i += 1;
                    if i >= self.size {
                        return Err("hash table overflow");
                    }
                }
            }
        }
    }

    /// Deletes an element with key `k`
    ///
    /// This corresponds to HASH-DELETE from CLRS Exercise 11.4-2.
    ///
    /// # Arguments
    /// * `k` - The key to delete
    ///
    /// # Returns
    /// The deleted value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(1/(1-α)) expected
    pub fn delete(&mut self, k: &K) -> Option<V> {
        let mut i = 0;
        loop {
            let j = self.probe(k, i);
            match &self.arr[j] {
                Slot::Empty => return None,
                Slot::Deleted => {
                    i += 1;
                    if i >= self.size {
                        return None;
                    }
                }
                Slot::Occupied(key, _) => {
                    if key == k {
                        if let Slot::Occupied(_, value) = std::mem::replace(&mut self.arr[j], Slot::Deleted) {
                            return Some(value);
                        }
                    }
                    i += 1;
                    if i >= self.size {
                        return None;
                    }
                }
            }
        }
    }
}

/// Helper function for linear probing hash table
pub fn linear_probe_hash_fn(k: usize, m: usize) -> usize {
    k % m
}

/// Helper function for double hashing secondary hash function
/// h2(k) = 1 + (k mod (m-1))
pub fn double_hash_h2(k: usize, m: usize) -> usize {
    1 + (k % (m - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_addressing_linear() {
        let mut table = OpenAddressingHashTable::new(
            11,
            ProbeType::Linear,
            linear_probe_hash_fn,
            None,
        );
        
        table.insert(10, "value10").unwrap();
        table.insert(22, "value22").unwrap();
        
        assert_eq!(table.search(&10), Some(&"value10"));
        assert_eq!(table.search(&22), Some(&"value22"));
    }

    #[test]
    fn test_open_addressing_quadratic() {
        let mut table = OpenAddressingHashTable::new(
            11,
            ProbeType::Quadratic { c1: 1, c2: 3 },
            linear_probe_hash_fn,
            None,
        );
        
        table.insert(10, "value10").unwrap();
        table.insert(22, "value22").unwrap();
        table.insert(31, "value31").unwrap();
        
        assert_eq!(table.search(&10), Some(&"value10"));
        assert_eq!(table.search(&22), Some(&"value22"));
    }

    #[test]
    fn test_open_addressing_double_hashing() {
        let mut table = OpenAddressingHashTable::new(
            11,
            ProbeType::DoubleHashing,
            linear_probe_hash_fn,
            Some(double_hash_h2),
        );
        
        table.insert(10, "value10").unwrap();
        table.insert(22, "value22").unwrap();
        table.insert(31, "value31").unwrap();
        
        assert_eq!(table.search(&10), Some(&"value10"));
        assert_eq!(table.search(&22), Some(&"value22"));
    }

    #[test]
    fn test_open_addressing_delete() {
        let mut table = OpenAddressingHashTable::new(
            11,
            ProbeType::Linear,
            linear_probe_hash_fn,
            None,
        );
        
        table.insert(10, "value10").unwrap();
        assert_eq!(table.delete(&10), Some("value10"));
        assert_eq!(table.search(&10), None);
        
        // Should be able to insert again after delete
        table.insert(10, "value10_new").unwrap();
        assert_eq!(table.search(&10), Some(&"value10_new"));
    }

    #[test]
    fn test_open_addressing_clrs_example() {
        // Example from CLRS 11.4-1: keys 10, 22, 31, 4, 15, 28, 17, 88, 59
        let mut table = OpenAddressingHashTable::new(
            11,
            ProbeType::Linear,
            |k, _| k, // h'(k) = k
            None,
        );
        
        let keys = vec![10, 22, 31, 4, 15, 28, 17, 88, 59];
        for key in &keys {
            table.insert(*key, format!("value{}", key)).unwrap();
        }
        
        // Verify all keys are present
        for key in &keys {
            assert!(table.search(key).is_some());
        }
    }
}

