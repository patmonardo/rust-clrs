//! Heapsort Algorithm (Section 6.4)
//!
//! Heapsort uses a heap to sort an array. It first builds a max-heap,
//! then repeatedly extracts the maximum element.

use super::heap::{build_max_heap, max_heapify};

/// Sorts an array using heapsort
///
/// This corresponds to HEAPSORT from CLRS Section 6.4.
/// The algorithm first builds a max-heap, then repeatedly extracts
/// the maximum element and places it at the end of the array.
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
///
/// # Complexity
/// - Time: O(n lg n) for all cases
/// - Space: O(1) if using iterative MAX-HEAPIFY, O(lg n) if recursive
///
/// # Example
/// ```
/// use clrs::chapter_06::heapsort;
/// let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
/// heapsort(&mut arr);
/// assert_eq!(arr, vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
/// ```
pub fn heapsort<T: Ord>(arr: &mut [T]) {
    let heap_size = arr.len();
    
    if heap_size <= 1 {
        return;
    }

    // CLRS: BUILD-MAX-HEAP(A)
    build_max_heap(arr);

    // CLRS: for i = A.length downto 2
    // For 0-based: from heap_size - 1 down to 1
    for i in (1..heap_size).rev() {
        // CLRS: exchange A[1] with A[i]
        arr.swap(0, i);
        
        // CLRS: A.heap-size = A.heap-size - 1
        // CLRS: MAX-HEAPIFY(A, 1)
        max_heapify(arr, i, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapsort_empty() {
        let mut arr: Vec<i32> = vec![];
        heapsort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_heapsort_single() {
        let mut arr = vec![42];
        heapsort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_heapsort_example() {
        let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
    }

    #[test]
    fn test_heapsort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heapsort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heapsort_clrs_example() {
        // Example from CLRS 6.4-1
        let mut arr = vec![5, 13, 2, 25, 7, 17, 20, 8, 4];
        heapsort(&mut arr);
        assert_eq!(arr, vec![2, 4, 5, 7, 8, 13, 17, 20, 25]);
    }

    #[test]
    fn test_heapsort_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}

