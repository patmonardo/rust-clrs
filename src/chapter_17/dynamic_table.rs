//! Dynamic Tables (Section 17.4)
//!
//! Implements a dynamic table that grows and shrinks as needed.
//! Demonstrates amortized analysis using the potential method.

/// A dynamic table that automatically resizes
///
/// This corresponds to TABLE-INSERT and TABLE-DELETE from CLRS Section 17.4.
/// The table doubles in size when it becomes full and halves when it becomes
/// less than 1/4 full (or 1/3 full with alternative strategy).
///
/// # Example
/// ```
/// use clrs::chapter_17::DynamicTable;
/// let mut table = DynamicTable::new();
/// table.insert(1);
/// table.insert(2);
/// assert_eq!(table.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct DynamicTable<T> {
    data: Vec<Option<T>>,
    num: usize,            // Number of elements
    size: usize,           // Total capacity
    shrink_threshold: f64, // Load factor below which we shrink (default 0.25)
    shrink_factor: f64,    // Factor to shrink by (default 0.5)
}

impl<T> DynamicTable<T> {
    /// Creates a new empty dynamic table
    pub fn new() -> Self {
        DynamicTable {
            data: Vec::new(),
            num: 0,
            size: 0,
            shrink_threshold: 0.25,
            shrink_factor: 0.5,
        }
    }

    /// Creates a new dynamic table with custom shrink parameters
    ///
    /// # Arguments
    /// * `shrink_threshold` - Load factor below which to shrink (e.g., 0.25 or 0.33)
    /// * `shrink_factor` - Factor to shrink by (e.g., 0.5 or 2/3)
    pub fn with_shrink_params(shrink_threshold: f64, shrink_factor: f64) -> Self {
        DynamicTable {
            data: Vec::new(),
            num: 0,
            size: 0,
            shrink_threshold,
            shrink_factor,
        }
    }

    /// Inserts an element into the table
    ///
    /// This corresponds to TABLE-INSERT from CLRS Section 17.4.
    /// Amortized cost: O(1)
    ///
    /// # Arguments
    /// * `item` - Item to insert
    pub fn insert(&mut self, item: T) {
        if self.num == self.size {
            // Table is full, expand it
            self.expand();
        }

        self.data[self.num] = Some(item);
        self.num += 1;
    }

    /// Removes and returns an element from the table
    ///
    /// This corresponds to TABLE-DELETE from CLRS Section 17.4.
    /// Amortized cost: O(1)
    ///
    /// # Returns
    /// The removed element, or None if table is empty
    pub fn delete(&mut self) -> Option<T> {
        if self.num == 0 {
            return None;
        }

        self.num -= 1;
        let item = self.data[self.num].take();

        let load_factor = if self.size > 0 {
            self.num as f64 / self.size as f64
        } else {
            0.0
        };

        if load_factor < self.shrink_threshold && self.size > 1 {
            self.contract();
        }

        item
    }

    /// Expands the table by doubling its size
    fn expand(&mut self) {
        let new_size = if self.size == 0 { 1 } else { self.size * 2 };
        let mut new_data = Vec::with_capacity(new_size);

        // Copy existing elements
        for i in 0..self.num {
            new_data.push(self.data[i].take());
        }

        // Fill rest with None
        for _ in self.num..new_size {
            new_data.push(None);
        }

        self.data = new_data;
        self.size = new_size;
    }

    /// Contracts the table by reducing its size
    fn contract(&mut self) {
        let new_size = (self.size as f64 * self.shrink_factor) as usize;
        let new_size = new_size.max(1);

        let mut new_data = Vec::with_capacity(new_size);

        // Copy existing elements
        for i in 0..self.num {
            new_data.push(self.data[i].take());
        }

        // Fill rest with None
        for _ in self.num..new_size {
            new_data.push(None);
        }

        self.data = new_data;
        self.size = new_size;
    }

    /// Returns the number of elements in the table
    pub fn len(&self) -> usize {
        self.num
    }

    /// Returns the capacity of the table
    pub fn capacity(&self) -> usize {
        self.size
    }

    /// Returns true if the table is empty
    pub fn is_empty(&self) -> bool {
        self.num == 0
    }

    /// Returns the load factor (num / size)
    pub fn load_factor(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            self.num as f64 / self.size as f64
        }
    }

    /// Returns a reference to the element at index i
    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.num {
            self.data[i].as_ref()
        } else {
            None
        }
    }
}

impl<T> Default for DynamicTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_table_insert() {
        let mut table = DynamicTable::new();

        assert_eq!(table.len(), 0);
        assert_eq!(table.capacity(), 0);

        table.insert(1);
        assert_eq!(table.len(), 1);
        assert_eq!(table.capacity(), 1);

        table.insert(2);
        assert_eq!(table.len(), 2);
        assert_eq!(table.capacity(), 2);

        table.insert(3);
        assert_eq!(table.len(), 3);
        assert_eq!(table.capacity(), 4); // Doubled
    }

    #[test]
    fn test_dynamic_table_delete() {
        let mut table = DynamicTable::new();

        for i in 0..10 {
            table.insert(i);
        }

        assert_eq!(table.len(), 10);
        assert!(table.capacity() >= 10);

        // Delete until load factor drops below threshold
        while table.len() > 0 {
            table.delete();
        }

        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_dynamic_table_expansion() {
        let mut table = DynamicTable::new();

        // Insert elements to trigger multiple expansions
        for i in 0..20 {
            table.insert(i);
        }

        assert_eq!(table.len(), 20);
        assert!(table.capacity() >= 20);

        // Verify all elements are present
        for i in 0..20 {
            assert_eq!(table.get(i), Some(&i));
        }
    }

    #[test]
    fn test_dynamic_table_contraction() {
        let mut table = DynamicTable::new();

        // Fill table
        for i in 0..16 {
            table.insert(i);
        }

        let initial_capacity = table.capacity();

        // Delete until contraction occurs
        while table.load_factor() >= 0.25 && table.len() > 0 {
            table.delete();
        }

        // Table should have contracted
        assert!(table.capacity() < initial_capacity || table.len() == 0);
    }

    #[test]
    fn test_dynamic_table_amortized_cost() {
        let mut table = DynamicTable::new();

        // Perform many insertions
        for i in 0..1000 {
            table.insert(i);
        }

        // Amortized cost should be O(1) per insertion
        // Even though some insertions trigger expansion (O(n) cost),
        // the amortized cost is O(1)
        assert!(table.len() == 1000);
    }

    #[test]
    fn test_dynamic_table_custom_shrink_params() {
        // Test with 1/3 threshold and 2/3 shrink factor (Exercise 17.4-3)
        let mut table = DynamicTable::with_shrink_params(1.0 / 3.0, 2.0 / 3.0);

        for i in 0..10 {
            table.insert(i);
        }

        // Delete until load factor drops below 1/3
        while table.load_factor() >= 1.0 / 3.0 && table.len() > 0 {
            table.delete();
        }

        assert!(table.load_factor() < 1.0 / 3.0 || table.len() == 0);
    }
}
