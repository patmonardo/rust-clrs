//! Randomized Quicksort (Section 7.3)
//!
//! This module contains randomized versions of PARTITION and QUICKSORT
//! that use randomization to achieve expected O(n lg n) performance.

use super::partition::partition;
use rand::Rng;

/// Randomly selects a pivot and partitions the subarray A[p..r]
///
/// This corresponds to RANDOMIZED-PARTITION from CLRS Section 7.3.
/// It randomly selects a pivot element to avoid worst-case behavior.
///
/// # Arguments
/// * `arr` - The array to partition
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Returns
/// The index of the pivot element after partitioning
///
/// # Complexity
/// - Time: Θ(n) where n = r - p + 1
/// - Space: O(1)
///
/// # Example
/// ```
/// use clrs::chapter_07::randomized_partition;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let q = randomized_partition(&mut arr, 0, 7);
/// // Pivot is randomly selected, q is its final position
/// ```
pub fn randomized_partition<T: Ord>(arr: &mut [T], p: usize, r: usize) -> usize {
    // CLRS: i = RANDOM(p, r)
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(p..=r);

    // CLRS: exchange A[r] with A[i]
    arr.swap(r, i);

    // CLRS: return PARTITION(A, p, r)
    partition(arr, p, r)
}

/// Sorts an array using randomized quicksort
///
/// This corresponds to RANDOMIZED-QUICKSORT from CLRS Section 7.3.
/// The algorithm uses randomization to avoid worst-case behavior,
/// achieving expected O(n lg n) performance.
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Complexity
/// - Expected time: O(n lg n)
/// - Worst case: O(n²) (rare with randomization)
/// - Space: O(lg n) for recursion stack
///
/// # Example
/// ```
/// use clrs::chapter_07::randomized_quicksort;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// randomized_quicksort(&mut arr, 0, 7);
/// assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn randomized_quicksort<T: Ord>(arr: &mut [T], p: usize, r: usize) {
    // CLRS: if p < r
    if p < r {
        // CLRS: q = RANDOMIZED-PARTITION(A, p, r)
        let q = randomized_partition(arr, p, r);

        // CLRS: RANDOMIZED-QUICKSORT(A, p, q - 1)
        if q > 0 {
            randomized_quicksort(arr, p, q - 1);
        }

        // CLRS: RANDOMIZED-QUICKSORT(A, q + 1, r)
        randomized_quicksort(arr, q + 1, r);
    }
}

/// Convenience function for randomized quicksort on entire array
///
/// # Example
/// ```
/// use clrs::chapter_07::randomized_quicksort_full;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// randomized_quicksort_full(&mut arr);
/// assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn randomized_quicksort_full<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    randomized_quicksort(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_quicksort_empty() {
        let mut arr: Vec<i32> = vec![];
        randomized_quicksort_full(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_randomized_quicksort_single() {
        let mut arr = vec![42];
        randomized_quicksort_full(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_randomized_quicksort_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        randomized_quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_randomized_quicksort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        randomized_quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_randomized_quicksort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        randomized_quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_randomized_quicksort_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        randomized_quicksort_full(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_randomized_partition() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let q = randomized_partition(&mut arr, 0, 7);
        // Verify partition property
        for i in 0..q {
            assert!(arr[i] <= arr[q]);
        }
        for i in (q + 1)..arr.len() {
            assert!(arr[i] > arr[q]);
        }
    }
}

