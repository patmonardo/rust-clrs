//! Linear Search (Section 2.1, Exercise 2.1-3)
//!
//! Linear search scans through the sequence, looking for a value v.

/// Performs linear search on an array
///
/// This corresponds to LINEAR-SEARCH from CLRS Exercise 2.1-3.
///
/// # Arguments
/// * `arr` - The array to search
/// * `v` - The value to search for
///
/// # Returns
/// * `Some(index)` - The 0-based index where v was found
/// * `None` - If v does not appear in the array (corresponds to NIL in CLRS)
///
/// # Example
/// ```
/// use clrs::chapter_02::linear_search;
/// let arr = vec![31, 41, 59, 26, 41, 58];
/// assert_eq!(linear_search(&arr, 59), Some(2));
/// assert_eq!(linear_search(&arr, 100), None);
/// ```
///
/// # Complexity
/// - Time: O(n) worst case, O(1) best case (if v is at index 0)
/// - Space: O(1)
///
/// # Loop Invariant
/// At the start of each iteration, the subarray `arr[0..i-1]` consists of elements
/// that are different than `v`.
pub fn linear_search<T: PartialEq>(arr: &[T], v: &T) -> Option<usize> {
    // CLRS uses 1-based indexing: for i = 1 to A.length
    // Rust uses 0-based, so we iterate from 0 to length-1
    for (i, element) in arr.iter().enumerate() {
        if element == v {
            // CLRS returns 1-based index i, but we return 0-based
            return Some(i);
        }
    }
    // Return None instead of NIL
    None
}

/// Performs linear search and returns 1-based index (CLRS style)
///
/// This version returns 1-based indices to match CLRS pseudocode exactly.
/// The first element is at index 1, second at index 2, etc.
///
/// # Returns
/// * `Some(index)` - The 1-based index where v was found (1..=n)
/// * `None` - If v does not appear in the array (corresponds to NIL)
pub fn linear_search_1based<T: PartialEq>(arr: &[T], v: &T) -> Option<usize> {
    for i in 0..arr.len() {
        if &arr[i] == v {
            // Return 1-based index
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_found() {
        let arr = vec![31, 41, 59, 26, 41, 58];
        assert_eq!(linear_search(&arr, &59), Some(2));
        assert_eq!(linear_search(&arr, &31), Some(0));
        assert_eq!(linear_search(&arr, &58), Some(5));
    }

    #[test]
    fn test_linear_search_not_found() {
        let arr = vec![31, 41, 59, 26, 41, 58];
        assert_eq!(linear_search(&arr, &100), None);
        assert_eq!(linear_search(&arr, &0), None);
    }

    #[test]
    fn test_linear_search_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(linear_search(&arr, &42), None);
    }

    #[test]
    fn test_linear_search_1based() {
        let arr = vec![31, 41, 59, 26, 41, 58];
        assert_eq!(linear_search_1based(&arr, &59), Some(3)); // 1-based
        assert_eq!(linear_search_1based(&arr, &31), Some(1)); // 1-based
        assert_eq!(linear_search_1based(&arr, &100), None);
    }

    #[test]
    fn test_linear_search_duplicates() {
        let arr = vec![31, 41, 59, 26, 41, 58];
        // Returns first occurrence
        assert_eq!(linear_search(&arr, &41), Some(1));
    }
}
