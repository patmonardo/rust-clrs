//! Merge Sort (Section 2.3)
//!
//! Merge sort is a divide-and-conquer algorithm that divides the array in half,
//! recursively sorts the halves, and then merges them together.

/// Merges two sorted subarrays into a single sorted array
///
/// This corresponds to MERGE from CLRS Section 2.3.
/// The procedure merges A[p..q] and A[q+1..r] into A[p..r].
///
/// # Arguments
/// * `arr` - The array containing the subarrays to merge
/// * `p` - Start index of first subarray (0-based)
/// * `q` - End index of first subarray (0-based, inclusive)
/// * `r` - End index of second subarray (0-based, inclusive)
///
/// # Complexity
/// - Time: O(n) where n = r - p + 1
/// - Space: O(n)
pub fn merge<T: Ord + Clone>(arr: &mut [T], p: usize, q: usize, r: usize) {
    // CLRS: n1 = q - p + 1 (1-based: p..q inclusive)
    let n1 = q - p + 1;
    // CLRS: n2 = r - q (1-based: q+1..r inclusive)
    let n2 = r - q;

    // CLRS: let L[1..n1] and R[1..n2] be new arrays
    let mut left = Vec::with_capacity(n1);
    let mut right = Vec::with_capacity(n2);

    // CLRS: for i = 1 to n1, L[i] = A[p + i - 1]
    // In 0-based: L[i] = A[p + i]
    for i in 0..n1 {
        left.push(arr[p + i].clone());
    }

    // CLRS: for j = 1 to n2, R[j] = A[q + j]
    // In 0-based: R[j] = A[q + 1 + j]
    for j in 0..n2 {
        right.push(arr[q + 1 + j].clone());
    }

    // Merge using sentinels approach (simpler) or the version without sentinels
    // Using sentinels: add infinity to the end of each array
    // For now, we'll use the version without sentinels (Exercise 2.3-2)
    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    // CLRS: for k = p to r
    while i < n1 && j < n2 {
        // CLRS: if L[i] ≤ R[j]
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements
    while i < n1 {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < n2 {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// Sorts an array using merge sort
///
/// This corresponds to MERGE-SORT from CLRS Section 2.3.
///
/// # Arguments
/// * `arr` - A mutable vector to be sorted in-place
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Example
/// ```
/// use clrs::chapter_02::merge_sort;
/// let mut arr = vec![3, 41, 52, 26, 38, 57, 9, 49];
/// merge_sort(&mut arr, 0, 7);
/// assert_eq!(arr, vec![3, 9, 26, 38, 41, 49, 52, 57]);
/// ```
///
/// # Complexity
/// - Time: O(n log n) for all cases
/// - Space: O(n)
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T], p: usize, r: usize) {
    // CLRS: if p < r
    if p < r {
        // CLRS: q = floor((p + r) / 2)
        let q = (p + r) / 2;

        // CLRS: MERGE-SORT(A, p, q)
        merge_sort(arr, p, q);

        // CLRS: MERGE-SORT(A, q + 1, r)
        merge_sort(arr, q + 1, r);

        // CLRS: MERGE(A, p, q, r)
        merge(arr, p, q, r);
    }
}

/// Convenience function for merge sort on entire array
///
/// # Example
/// ```
/// use clrs::chapter_02::merge_sort_full;
/// let mut arr = vec![3, 41, 52, 26, 38, 57, 9, 49];
/// merge_sort_full(&mut arr);
/// assert_eq!(arr, vec![3, 9, 26, 38, 41, 49, 52, 57]);
/// ```
pub fn merge_sort_full<T: Ord + Clone>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    merge_sort(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort_full(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_merge_sort_single() {
        let mut arr = vec![42];
        merge_sort_full(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_merge_sort_example() {
        let mut arr = vec![3, 41, 52, 26, 38, 57, 9, 49];
        merge_sort_full(&mut arr);
        assert_eq!(arr, vec![3, 9, 26, 38, 41, 49, 52, 57]);
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort_full(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort_full(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge() {
        let mut arr = vec![1, 3, 5, 2, 4, 6];
        merge(&mut arr, 0, 2, 5);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}
