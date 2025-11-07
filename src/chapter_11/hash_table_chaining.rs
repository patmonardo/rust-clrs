//! Hash Tables with Chaining (Section 11.2)
//!
//! Hash tables that resolve collisions by chaining elements that hash to
//! the same slot into a linked list.

/// Node in a hash table chain
#[derive(Debug, Clone)]
pub struct HashNode<K, V> {
    pub key: K,
    pub value: V,
}

/// Hash table with chaining
///
/// This corresponds to the hash table implementation from CLRS Section 11.2.
/// Collisions are resolved by chaining elements into linked lists.
///
/// # Example
/// ```
/// use clrs::chapter_11::HashTableChaining;
/// let mut table = HashTableChaining::new(11, |k, m| k % m);
/// table.insert(42, "value");
/// assert_eq!(table.search(42), Some(&"value"));
/// ```
#[derive(Debug, Clone)]
pub struct HashTableChaining<K: PartialEq + Clone, V> {
    arr: Vec<Vec<HashNode<K, V>>>,
    size: usize,
    hash_fn: fn(K, usize) -> usize,
}

impl<K: PartialEq + Clone, V> HashTableChaining<K, V> {
    /// Creates a new hash table with chaining
    ///
    /// # Arguments
    /// * `m` - The size of the hash table
    /// * `hash_fn` - The hash function to use
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_11::HashTableChaining;
    /// let table: HashTableChaining<usize, i32> = HashTableChaining::new(11, |k, m| k % m);
    /// ```
    pub fn new(m: usize, hash_fn: fn(K, usize) -> usize) -> Self {
        let mut arr = Vec::with_capacity(m);
        arr.resize_with(m, Vec::new);
        HashTableChaining {
            arr,
            size: m,
            hash_fn,
        }
    }

    /// Searches for an element with key `k`
    ///
    /// This corresponds to CHAINED-HASH-SEARCH from CLRS Section 11.2.
    ///
    /// # Arguments
    /// * `k` - The key to search for
    ///
    /// # Returns
    /// A reference to the value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(1 + α) where α is the load factor
    pub fn search(&self, k: K) -> Option<&V> {
        let h = (self.hash_fn)(k.clone(), self.size);
        self.arr[h]
            .iter()
            .find(|node| node.key == k)
            .map(|node| &node.value)
    }

    /// Inserts an element with key `k` and value `v`
    ///
    /// This corresponds to CHAINED-HASH-INSERT from CLRS Section 11.2.
    ///
    /// # Arguments
    /// * `k` - The key
    /// * `v` - The value to insert
    ///
    /// # Complexity
    /// - Time: O(1) average case
    pub fn insert(&mut self, k: K, v: V) {
        let h = (self.hash_fn)(k.clone(), self.size);
        let node = HashNode { key: k.clone(), value: v };
        
        // Check if key already exists and update
        if let Some(existing) = self.arr[h].iter_mut().find(|n| n.key == k) {
            existing.value = node.value;
        } else {
            self.arr[h].push(node);
        }
    }

    /// Deletes an element with key `k`
    ///
    /// This corresponds to CHAINED-HASH-DELETE from CLRS Section 11.2.
    ///
    /// # Arguments
    /// * `k` - The key to delete
    ///
    /// # Returns
    /// The deleted value if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(1 + α) average case
    pub fn delete(&mut self, k: K) -> Option<V> {
        let h = (self.hash_fn)(k.clone(), self.size);
        if let Some(pos) = self.arr[h].iter().position(|node| node.key == k) {
            Some(self.arr[h].remove(pos).value)
        } else {
            None
        }
    }
}

// Re-export division_hash from hash_functions module
pub use super::hash_functions::division_hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table_chaining_basic() {
        let mut table = HashTableChaining::new(11, division_hash);
        
        table.insert(5, "value5");
        table.insert(16, "value16"); // 16 % 11 = 5, collision with 5
        
        assert_eq!(table.search(5), Some(&"value5"));
        assert_eq!(table.search(16), Some(&"value16"));
    }

    #[test]
    fn test_hash_table_chaining_clrs_example() {
        // Example from CLRS 11.2-2
        let mut table = HashTableChaining::new(9, division_hash);
        let keys = vec![5, 28, 19, 15, 20, 33, 12, 17, 10];
        
        for key in keys {
            table.insert(key, format!("value{}", key));
        }
        
        // Verify some keys
        assert_eq!(table.search(5), Some(&"value5".to_string()));
        assert_eq!(table.search(28), Some(&"value28".to_string()));
        assert_eq!(table.search(19), Some(&"value19".to_string()));
    }

    #[test]
    fn test_hash_table_chaining_delete() {
        let mut table = HashTableChaining::new(11, division_hash);
        
        table.insert(5, "value5");
        table.insert(16, "value16");
        
        assert_eq!(table.delete(5), Some("value5"));
        assert_eq!(table.search(5), None);
        assert_eq!(table.search(16), Some(&"value16"));
    }
}

