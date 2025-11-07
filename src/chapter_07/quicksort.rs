//! Quicksort Algorithm (Section 7.2)
//!
//! This module contains the quicksort algorithm that uses PARTITION
//! to sort arrays in place.

use super::partition::partition;

/// Sorts an array using quicksort
///
/// This corresponds to QUICKSORT from CLRS Section 7.2.
/// The algorithm uses divide-and-conquer: it partitions the array
/// around a pivot, then recursively sorts the subarrays.
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Complexity
/// - Best case: O(n lg n)
/// - Average case: O(n lg n)
/// - Worst case: O(n²) when array is already sorted or reverse sorted
/// - Space: O(lg n) for recursion stack
///
/// # Example
/// ```
/// use clrs::chapter_07::quicksort;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// quicksort(&mut arr, 0, 7);
/// assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn quicksort<T: Ord>(arr: &mut [T], p: usize, r: usize) {
    // CLRS: if p < r
    if p < r {
        // CLRS: q = PARTITION(A, p, r)
        let q = partition(arr, p, r);

        // CLRS: QUICKSORT(A, p, q - 1)
        if q > 0 {
            quicksort(arr, p, q - 1);
        }

        // CLRS: QUICKSORT(A, q + 1, r)
        quicksort(arr, q + 1, r);
    }
}

/// Sorts an array using quicksort (nonincreasing order)
///
/// Modified version of QUICKSORT to sort in nonincreasing order (Exercise 7.1-4).
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Example
/// ```
/// use clrs::chapter_07::quicksort_nonincreasing;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// quicksort_nonincreasing(&mut arr, 0, 7);
/// assert_eq!(arr, vec![9, 6, 5, 4, 3, 2, 1, 1]);
/// ```
pub fn quicksort_nonincreasing<T: Ord>(arr: &mut [T], p: usize, r: usize) {
    if p < r {
        use super::partition::partition_nonincreasing;
        let q = partition_nonincreasing(arr, p, r);
        if q > 0 {
            quicksort_nonincreasing(arr, p, q - 1);
        }
        quicksort_nonincreasing(arr, q + 1, r);
    }
}

/// Convenience function for quicksort on entire array
///
/// # Example
/// ```
/// use clrs::chapter_07::quicksort_full;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// quicksort_full(&mut arr);
/// assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn quicksort_full<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    quicksort(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort_empty() {
        let mut arr: Vec<i32> = vec![];
        quicksort_full(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_quicksort_single() {
        let mut arr = vec![42];
        quicksort_full(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_quicksort_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_quicksort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_quicksort_nonincreasing() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort_nonincreasing(&mut arr, 0, 7);
        assert_eq!(arr, vec![9, 6, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn test_quicksort_subarray() {
        let mut arr = vec![9, 3, 1, 4, 1, 5, 2, 6, 8];
        quicksort(&mut arr, 1, 7);
        // Only subarray [1..7] should be sorted
        assert_eq!(arr[1..=7], vec![1, 1, 2, 3, 4, 5, 6]);
        assert_eq!(arr[0], 9);
        assert_eq!(arr[8], 8);
    }
}

