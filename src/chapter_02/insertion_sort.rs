//! Insertion Sort (Section 2.1)
//!
//! Insertion sort is a simple sorting algorithm that builds the final sorted array
//! one item at a time. It is much less efficient on large lists than more advanced
//! algorithms such as quicksort, heapsort, or merge sort.

/// Sorts an array using insertion sort (nondecreasing order)
///
/// This corresponds to INSERTION-SORT from CLRS Section 2.1.
///
/// # Arguments
/// * `arr` - A mutable vector to be sorted in-place
///
/// # Example
/// ```
/// use clrs::chapter_02::insertion_sort;
/// let mut arr = vec![31, 41, 59, 26, 41, 58];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, vec![26, 31, 41, 41, 58, 59]);
/// ```
///
/// # Complexity
/// - Time: O(n²) worst case, O(n) best case (already sorted)
/// - Space: O(1)
pub fn insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    // CLRS uses 1-based indexing: for j = 2 to A.length
    // Rust uses 0-based, so we iterate from index 1 to length-1
    let n = arr.len();

    if n <= 1 {
        return;
    }

    // j corresponds to CLRS j (1-based), so j = 2..=n becomes 1..n in 0-based
    for j in 1..n {
        // key = A[j] in CLRS (1-based index j)
        let key = arr[j].clone();

        // i = j - 1 in CLRS (1-based)
        let mut i = j;

        // while i > 0 and A[i] > key
        // In CLRS: while i > 0 (1-based), which means while i >= 1 (0-based: i > 0)
        while i > 0 && arr[i - 1] > key {
            // A[i + 1] = A[i] in CLRS (1-based)
            // In 0-based: arr[i] = arr[i-1]
            arr[i] = arr[i - 1].clone();
            i -= 1;
        }

        // A[i + 1] = key in CLRS (1-based)
        // In 0-based: arr[i] = key
        arr[i] = key;
    }
}

/// Sorts an array using insertion sort (nonincreasing order)
///
/// This corresponds to Exercise 2.1-2, rewriting INSERTION-SORT
/// to sort into nonincreasing instead of nondecreasing order.
///
/// # Arguments
/// * `arr` - A mutable vector to be sorted in-place
///
/// # Example
/// ```
/// use clrs::chapter_02::insertion_sort_decreasing;
/// let mut arr = vec![31, 41, 59, 26, 41, 58];
/// insertion_sort_decreasing(&mut arr);
/// assert_eq!(arr, vec![59, 58, 41, 41, 31, 26]);
/// ```
pub fn insertion_sort_decreasing<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    if n <= 1 {
        return;
    }

    for j in 1..n {
        let key = arr[j].clone();
        let mut i = j;

        // Changed from > to < for nonincreasing order
        while i > 0 && arr[i - 1] < key {
            arr[i] = arr[i - 1].clone();
            i -= 1;
        }

        arr[i] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_insertion_sort_single() {
        let mut arr = vec![42];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_insertion_sort_example() {
        let mut arr = vec![31, 41, 59, 26, 41, 58];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![26, 31, 41, 41, 58, 59]);
    }

    #[test]
    fn test_insertion_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_decreasing() {
        let mut arr = vec![31, 41, 59, 26, 41, 58];
        insertion_sort_decreasing(&mut arr);
        assert_eq!(arr, vec![59, 58, 41, 41, 31, 26]);
    }
}
