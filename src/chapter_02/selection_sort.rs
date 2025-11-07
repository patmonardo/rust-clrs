//! Selection Sort (Section 2.2, Exercise 2.2-2)
//!
//! Selection sort finds the smallest element and exchanges it with the first element,
//! then finds the second smallest and exchanges it with the second element, etc.

/// Sorts an array using selection sort
///
/// This corresponds to the pseudocode from CLRS Exercise 2.2-2.
///
/// # Arguments
/// * `arr` - A mutable vector to be sorted in-place
///
/// # Example
/// ```
/// use clrs::chapter_02::selection_sort;
/// let mut arr = vec![64, 25, 12, 22, 11];
/// selection_sort(&mut arr);
/// assert_eq!(arr, vec![11, 12, 22, 25, 64]);
/// ```
///
/// # Complexity
/// - Time: Θ(n²) for all cases
/// - Space: O(1)
///
/// # Loop Invariant
/// At the start of each iteration, the subarray `arr[0..i]` contains the smallest
/// `i` elements in sorted order.
pub fn selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    if n <= 1 {
        return;
    }

    // CLRS: for i = 1 to n - 1 (1-based)
    // Rust: for i in 0..n-1 (0-based)
    for i in 0..n - 1 {
        // minIndex = i in CLRS (1-based)
        let mut min_index = i;

        // CLRS: for j = i + 1 to n (1-based)
        // Rust: for j in (i+1)..n (0-based)
        for j in (i + 1)..n {
            // CLRS: if A[j] < A[minIndex]
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // CLRS: swap(A[i], A[minIndex])
        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_selection_sort_single() {
        let mut arr = vec![42];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_selection_sort_example() {
        let mut arr = vec![64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_selection_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
