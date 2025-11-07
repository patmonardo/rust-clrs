//! Partition Operations (Section 7.1)
//!
//! This module contains the PARTITION procedure that is the core of quicksort.

/// Partitions the subarray A[p..r] around a pivot
///
/// This corresponds to PARTITION from CLRS Section 7.1.
/// The procedure rearranges the array so that:
/// - Elements less than or equal to the pivot are to the left
/// - Elements greater than the pivot are to the right
/// - The pivot is in its final sorted position
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
/// use clrs::chapter_07::partition;
/// let mut arr = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
/// let q = partition(&mut arr, 0, 11);
/// // After partition, arr[q] is in its final position
/// // Elements arr[0..q] are <= pivot
/// // Elements arr[q+1..12] are > pivot
/// ```
pub fn partition<T: Ord>(arr: &mut [T], p: usize, r: usize) -> usize {
    // CLRS: x = A[r] (pivot)
    // We'll use the last element as pivot
    let pivot_idx = r;

    // CLRS: i = p - 1
    let mut i = p as i32 - 1;

    // CLRS: for j = p to r - 1
    for j in p..r {
        // CLRS: if A[j] <= x
        if arr[j] <= arr[pivot_idx] {
            // CLRS: i = i + 1
            i += 1;
            // CLRS: exchange A[i] with A[j]
            arr.swap(i as usize, j);
        }
    }

    // CLRS: exchange A[i + 1] with A[r]
    arr.swap((i + 1) as usize, pivot_idx);

    // CLRS: return i + 1
    (i + 1) as usize
}

/// Partitions the subarray A[p..r] around a pivot (handles equal elements)
///
/// Modified version of PARTITION that returns q = floor((p + r) / 2)
/// when all elements have the same value (Exercise 7.1-2).
///
/// # Arguments
/// * `arr` - The array to partition
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Returns
/// The index of the pivot element after partitioning
///
/// # Example
/// ```
/// use clrs::chapter_07::partition_balanced;
/// let mut arr = vec![5, 5, 5, 5, 5];
/// let q = partition_balanced(&mut arr, 0, 4);
/// assert_eq!(q, 2); // floor((0 + 4) / 2) = 2
/// ```
pub fn partition_balanced<T: Ord>(arr: &mut [T], p: usize, r: usize) -> usize {
    let pivot_idx = r;
    let mut i = p as i32 - 1;
    let mut equal_count = 0;

    for j in p..r {
        if arr[j] < arr[pivot_idx] {
            i += 1;
            arr.swap(i as usize, j);
        } else if arr[j] == arr[pivot_idx] {
            equal_count += 1;
        }
    }

    let final_pos = (i + 1) as usize;
    arr.swap(final_pos, pivot_idx);

    // If all elements are equal, return floor((p + r) / 2)
    if equal_count == r - p {
        (p + r) / 2
    } else {
        final_pos
    }
}

/// Partitions the subarray A[p..r] for nonincreasing order
///
/// Modified version of PARTITION to sort in nonincreasing order (Exercise 7.1-4).
/// Only the comparison condition is flipped.
///
/// # Arguments
/// * `arr` - The array to partition
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Returns
/// The index of the pivot element after partitioning
///
/// # Example
/// ```
/// use clrs::chapter_07::partition_nonincreasing;
/// let mut arr = vec![3, 1, 4, 1, 5, 9];
/// let q = partition_nonincreasing(&mut arr, 0, 5);
/// // Elements arr[0..q] are >= pivot
/// // Elements arr[q+1..6] are < pivot
/// ```
pub fn partition_nonincreasing<T: Ord>(arr: &mut [T], p: usize, r: usize) -> usize {
    let pivot_idx = r;
    let mut i = p as i32 - 1;

    for j in p..r {
        // Flipped condition: >= instead of <=
        if arr[j] >= arr[pivot_idx] {
            i += 1;
            arr.swap(i as usize, j);
        }
    }

    arr.swap((i + 1) as usize, pivot_idx);
    (i + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_basic() {
        let mut arr = vec![2, 8, 7, 1, 3, 5, 6, 4];
        let q = partition(&mut arr, 0, 7);
        assert_eq!(q, 3); // Pivot 4 should be at index 3
                          // Verify partition property
        for i in 0..q {
            assert!(arr[i] <= arr[q]);
        }
        for i in (q + 1)..arr.len() {
            assert!(arr[i] > arr[q]);
        }
    }

    #[test]
    fn test_partition_clrs_example() {
        // Example from CLRS 7.1-1
        let mut arr = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let q = partition(&mut arr, 0, 11);
        // After partition, pivot (11) should be at position q
        assert_eq!(arr[q], 11);
        // Elements before q should be <= 11
        for i in 0..q {
            assert!(arr[i] <= 11);
        }
        // Elements after q should be > 11
        for i in (q + 1)..arr.len() {
            assert!(arr[i] > 11);
        }
    }

    #[test]
    fn test_partition_balanced_all_equal() {
        // Exercise 7.1-2: all elements equal
        let mut arr = vec![5, 5, 5, 5, 5];
        let q = partition_balanced(&mut arr, 0, 4);
        assert_eq!(q, 2); // Should return floor((0 + 4) / 2) = 2
    }

    #[test]
    fn test_partition_nonincreasing() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let q = partition_nonincreasing(&mut arr, 0, 7);
        // Elements before q should be >= pivot
        for i in 0..q {
            assert!(arr[i] >= arr[q]);
        }
        // Elements after q should be < pivot
        for i in (q + 1)..arr.len() {
            assert!(arr[i] < arr[q]);
        }
    }

    #[test]
    fn test_partition_single_element() {
        let mut arr = vec![42];
        let q = partition(&mut arr, 0, 0);
        assert_eq!(q, 0);
    }

    #[test]
    fn test_partition_sorted_increasing() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let q = partition(&mut arr, 0, 4);
        assert_eq!(q, 4); // Pivot 5 should be at the end
    }

    #[test]
    fn test_partition_sorted_decreasing() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let q = partition(&mut arr, 0, 4);
        assert_eq!(q, 0); // Pivot 1 should be at the beginning
    }
}
