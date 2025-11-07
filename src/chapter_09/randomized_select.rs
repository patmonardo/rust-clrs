//! Randomized Select (Section 9.2)
//!
//! This module contains RANDOMIZED-SELECT, which finds the ith smallest
//! element in expected linear time.

use crate::chapter_07::randomized_quicksort::randomized_partition;

/// Finds the ith smallest element using randomized select
///
/// This corresponds to RANDOMIZED-SELECT from CLRS Section 9.2.
/// The algorithm uses randomized partition to find the ith order statistic
/// in expected O(n) time.
///
/// # Arguments
/// * `arr` - The array to search (modified in-place)
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
/// * `i` - The order statistic to find (1-based: 1 = minimum, n = maximum)
///
/// # Returns
/// The ith smallest element
///
/// # Complexity
/// - Expected time: O(n)
/// - Worst case: O(n²)
/// - Space: O(1) for iterative version, O(lg n) for recursive
///
/// # Example
/// ```
/// use clrs::chapter_09::randomized_select;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let median = randomized_select(&mut arr, 0, 7, 4);
/// // 4th smallest element in sorted order
/// ```
pub fn randomized_select<T: Ord + Clone>(arr: &mut [T], p: usize, r: usize, i: usize) -> T {
    if p == r {
        return arr[p].clone();
    }

    // CLRS: q = RANDOMIZED-PARTITION(A, p, r)
    let q = randomized_partition(arr, p, r);

    // CLRS: k = q - p + 1
    let k = q - p + 1;

    // CLRS: if i == k
    if i == k {
        // CLRS: return A[q]
        return arr[q].clone();
    }

    // CLRS: else if i < k
    if i < k {
        // CLRS: return RANDOMIZED-SELECT(A, p, q - 1, i)
        randomized_select(arr, p, q - 1, i)
    } else {
        // CLRS: else return RANDOMIZED-SELECT(A, q + 1, r, i - k)
        randomized_select(arr, q + 1, r, i - k)
    }
}

/// Iterative version of RANDOMIZED-SELECT (Exercise 9.2-3)
///
/// This avoids recursion overhead and uses constant space.
///
/// # Arguments
/// * `arr` - The array to search (modified in-place)
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
/// * `i` - The order statistic to find (1-based)
///
/// # Returns
/// The ith smallest element
///
/// # Example
/// ```
/// use clrs::chapter_09::randomized_select_iterative;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let median = randomized_select_iterative(&mut arr, 0, 7, 4);
/// ```
pub fn randomized_select_iterative<T: Ord + Clone>(
    arr: &mut [T],
    mut p: usize,
    mut r: usize,
    mut i: usize,
) -> T {
    loop {
        if p == r {
            return arr[p].clone();
        }

        let q = randomized_partition(arr, p, r);
        let k = q - p + 1;

        if i == k {
            return arr[q].clone();
        } else if i < k {
            r = q - 1;
        } else {
            p = q + 1;
            i -= k;
        }
    }
}

/// Convenience function to find the ith smallest element in entire array
///
/// # Arguments
/// * `arr` - The array to search (modified in-place)
/// * `i` - The order statistic to find (1-based)
///
/// # Returns
/// The ith smallest element
///
/// # Example
/// ```
/// use clrs::chapter_09::randomized_select_full;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let min = randomized_select_full(&mut arr, 1);
/// assert_eq!(min, 1);
/// ```
pub fn randomized_select_full<T: Ord + Clone>(arr: &mut [T], i: usize) -> T {
    if arr.is_empty() {
        panic!("Cannot select from empty array");
    }
    if i == 0 || i > arr.len() {
        panic!("Order statistic i must be between 1 and {}", arr.len());
    }
    randomized_select(arr, 0, arr.len() - 1, i)
}

/// Finds the median using randomized select
///
/// # Arguments
/// * `arr` - The array to search (modified in-place)
///
/// # Returns
/// The median element
///
/// # Example
/// ```
/// use clrs::chapter_09::randomized_median;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let median = randomized_median(&mut arr);
/// ```
pub fn randomized_median<T: Ord + Clone>(arr: &mut [T]) -> T {
    if arr.is_empty() {
        panic!("Cannot find median of empty array");
    }
    let n = arr.len();
    let i = n.div_ceil(2); // Median position
    randomized_select_full(arr, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_select_minimum() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let min = randomized_select_full(&mut arr, 1);
        assert_eq!(min, 1);
    }

    #[test]
    fn test_randomized_select_maximum() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let max = randomized_select_full(&mut arr, 8);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_randomized_select_median() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let median = randomized_median(&mut arr);
        // Median of sorted [1, 1, 2, 3, 4, 5, 6, 9] is 3 or 4
        assert!(median == 3 || median == 4);
    }

    #[test]
    fn test_randomized_select_iterative() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let min = randomized_select_iterative(&mut arr, 0, 7, 1);
        assert_eq!(min, 1);
    }

    #[test]
    fn test_randomized_select_single() {
        let mut arr = vec![42];
        let result = randomized_select_full(&mut arr, 1);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_randomized_select_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let third = randomized_select_full(&mut arr, 3);
        assert_eq!(third, 3);
    }
}
