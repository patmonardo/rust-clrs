//! Direct-Address Tables (Section 11.1)
//!
//! A direct-address table is a simple data structure in which each slot
//! corresponds directly to a key value.

/// Direct-address table
///
/// This corresponds to the direct-address table implementation from CLRS Section 11.1.
/// Each slot in the table corresponds directly to a key value.
///
/// # Example
/// ```
/// use clrs::chapter_11::DirectAddressTable;
/// let mut table = DirectAddressTable::new(100);
/// table.insert(42, "value");
/// assert_eq!(table.search(42), Some(&"value"));
/// ```
#[derive(Debug, Clone)]
pub struct DirectAddressTable<T> {
    arr: Vec<Option<T>>,
    size: usize,
}

impl<T> DirectAddressTable<T> {
    /// Creates a new direct-address table of size `m`
    ///
    /// # Arguments
    /// * `m` - The size of the table (maximum key value + 1)
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_11::DirectAddressTable;
    /// let table: DirectAddressTable<i32> = DirectAddressTable::new(100);
    /// ```
    pub fn new(m: usize) -> Self {
        let mut arr = Vec::with_capacity(m);
        arr.resize_with(m, || None);
        DirectAddressTable { arr, size: m }
    }

    /// Searches for an element with key `k`
    ///
    /// This corresponds to DIRECT-ADDRESS-SEARCH from CLRS Section 11.1.
    ///
    /// # Arguments
    /// * `k` - The key to search for
    ///
    /// # Returns
    /// A reference to the element if found, `None` otherwise
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn search(&self, k: usize) -> Option<&T> {
        if k < self.size {
            self.arr[k].as_ref()
        } else {
            None
        }
    }

    /// Inserts an element with key `k`
    ///
    /// This corresponds to DIRECT-ADDRESS-INSERT from CLRS Section 11.1.
    ///
    /// # Arguments
    /// * `k` - The key
    /// * `x` - The element to insert
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn insert(&mut self, k: usize, x: T) -> Result<(), &'static str> {
        if k >= self.size {
            return Err("key out of range");
        }
        self.arr[k] = Some(x);
        Ok(())
    }

    /// Deletes an element with key `k`
    ///
    /// This corresponds to DIRECT-ADDRESS-DELETE from CLRS Section 11.1.
    ///
    /// # Arguments
    /// * `k` - The key to delete
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn delete(&mut self, k: usize) -> Option<T> {
        if k < self.size {
            self.arr[k].take()
        } else {
            None
        }
    }

    /// Finds the maximum element in the table
    ///
    /// This corresponds to MAXIMUM from CLRS Exercise 11.1-1.
    ///
    /// # Returns
    /// The maximum key that exists in the table, or `None` if the table is empty
    ///
    /// # Complexity
    /// - Time: O(m) where m is the table size
    pub fn maximum(&self) -> Option<usize> {
        (0..self.size).rev().find(|&i| self.arr[i].is_some())
    }
}

/// Bit vector for representing a dynamic set (Exercise 11.1-2)
///
/// A bit vector uses an array of bits to represent a set of distinct elements
/// with no satellite data. All operations run in O(1) time.
///
/// # Example
/// ```
/// use clrs::chapter_11::BitVector;
/// let mut bv = BitVector::new(100);
/// bv.insert(42);
/// assert!(bv.search(42));
/// ```
#[derive(Debug, Clone)]
pub struct BitVector {
    arr: Vec<bool>,
    size: usize,
}

impl BitVector {
    /// Creates a new bit vector of size `m`
    ///
    /// # Arguments
    /// * `m` - The size of the bit vector
    pub fn new(m: usize) -> Self {
        BitVector {
            arr: vec![false; m],
            size: m,
        }
    }

    /// Searches for key `k` in the bit vector
    ///
    /// This corresponds to BITMAP-SEARCH from CLRS Exercise 11.1-2.
    ///
    /// # Arguments
    /// * `k` - The key to search for
    ///
    /// # Returns
    /// `true` if the key is present, `false` otherwise
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn search(&self, k: usize) -> bool {
        if k < self.size {
            self.arr[k]
        } else {
            false
        }
    }

    /// Inserts key `k` into the bit vector
    ///
    /// This corresponds to BITMAP-INSERT from CLRS Exercise 11.1-2.
    ///
    /// # Arguments
    /// * `k` - The key to insert
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn insert(&mut self, k: usize) -> Result<(), &'static str> {
        if k >= self.size {
            return Err("key out of range");
        }
        self.arr[k] = true;
        Ok(())
    }

    /// Deletes key `k` from the bit vector
    ///
    /// This corresponds to BITMAP-DELETE from CLRS Exercise 11.1-2.
    ///
    /// # Arguments
    /// * `k` - The key to delete
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn delete(&mut self, k: usize) -> Result<(), &'static str> {
        if k >= self.size {
            return Err("key out of range");
        }
        self.arr[k] = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_address_basic() {
        let mut table = DirectAddressTable::new(10);

        table.insert(5, "value5").unwrap();
        table.insert(3, "value3").unwrap();

        assert_eq!(table.search(5), Some(&"value5"));
        assert_eq!(table.search(3), Some(&"value3"));
        assert_eq!(table.search(7), None);
    }

    #[test]
    fn test_direct_address_delete() {
        let mut table = DirectAddressTable::new(10);

        table.insert(5, "value5").unwrap();
        assert_eq!(table.delete(5), Some("value5"));
        assert_eq!(table.search(5), None);
    }

    #[test]
    fn test_direct_address_maximum() {
        let mut table = DirectAddressTable::new(10);

        assert_eq!(table.maximum(), None);

        table.insert(5, "value5").unwrap();
        table.insert(3, "value3").unwrap();
        table.insert(8, "value8").unwrap();

        assert_eq!(table.maximum(), Some(8));
    }

    #[test]
    fn test_bit_vector() {
        let mut bv = BitVector::new(100);

        assert!(!bv.search(42));
        bv.insert(42).unwrap();
        assert!(bv.search(42));

        bv.insert(10).unwrap();
        bv.insert(99).unwrap();

        assert!(bv.search(10));
        assert!(bv.search(99));

        bv.delete(42).unwrap();
        assert!(!bv.search(42));
    }
}
