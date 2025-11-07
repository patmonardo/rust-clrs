//! Select in Worst-Case Linear Time (Section 9.3)
//!
//! This module contains SELECT, which finds the ith smallest element
//! in worst-case O(n) time using the median-of-medians algorithm.

use crate::chapter_07::partition::partition;

/// Finds the median of a small array using insertion sort
///
/// Helper function for SELECT. Sorts the array and returns the median.
fn insertion_sort_median<T: Ord + Clone>(arr: &mut [T]) -> T {
    // Simple insertion sort
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
    
    // Return median
    arr[arr.len() / 2].clone()
}

/// Partitions array into groups of 5 and finds median of medians
///
/// This is the key subroutine of SELECT that finds a good pivot.
fn median_of_medians<T: Ord + Clone>(arr: &mut [T], p: usize, r: usize) -> usize {
    let n = r - p + 1;
    
    // If array is small, just sort and return median index
    if n <= 5 {
        let mut group = arr[p..=r].to_vec();
        insertion_sort_median(&mut group);
        // Find the median value in original array
        let median_val = group[group.len() / 2].clone();
        for i in p..=r {
            if arr[i] == median_val {
                return i;
            }
        }
        return p + n / 2;
    }
    
    // Divide into groups of 5 and find median of each
    let num_groups = n.div_ceil(5);
    let mut medians = Vec::new();
    
    for i in 0..num_groups {
        let start = p + i * 5;
        let end = (start + 4).min(r);
        let mut group = arr[start..=end].to_vec();
        let median = insertion_sort_median(&mut group);
        medians.push(median);
    }
    
    // Recursively find median of medians
    let medians_len = medians.len();
    let median_pos = medians_len.div_ceil(2);
    let median_of_medians_val = select_helper(&mut medians, 0, medians_len - 1, median_pos);
    
    // Find index of median-of-medians in original array
    for i in p..=r {
        if arr[i] == median_of_medians_val {
            return i;
        }
    }
    
    p + n / 2 // Fallback
}

/// Helper function for SELECT that does the actual work
fn select_helper<T: Ord + Clone>(arr: &mut [T], p: usize, r: usize, i: usize) -> T {
    if p == r {
        return arr[p].clone();
    }
    
    // Find median-of-medians pivot
    let pivot_idx = median_of_medians(arr, p, r);
    
    // Swap pivot to end
    arr.swap(pivot_idx, r);
    
    // Partition around pivot
    let q = partition(arr, p, r);
    
    let k = q - p + 1;
    
    if i == k {
        arr[q].clone()
    } else if i < k {
        select_helper(arr, p, q - 1, i)
    } else {
        select_helper(arr, q + 1, r, i - k)
    }
}

/// Finds the ith smallest element in worst-case linear time
///
/// This corresponds to SELECT from CLRS Section 9.3.
/// The algorithm uses median-of-medians to guarantee O(n) worst-case time.
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
/// - Time: O(n) worst-case
/// - Space: O(lg n) for recursion
///
/// # Example
/// ```
/// use clrs::chapter_09::select;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let median = select(&mut arr, 0, 7, 4);
/// // 4th smallest element in sorted order
/// ```
pub fn select<T: Ord + Clone>(arr: &mut [T], p: usize, r: usize, i: usize) -> T {
    if p > r {
        panic!("Invalid range: p > r");
    }
    if i == 0 || i > r - p + 1 {
        panic!("Order statistic i must be between 1 and {}", r - p + 1);
    }
    select_helper(arr, p, r, i)
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
/// use clrs::chapter_09::select_full;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let min = select_full(&mut arr, 1);
/// assert_eq!(min, 1);
/// ```
pub fn select_full<T: Ord + Clone>(arr: &mut [T], i: usize) -> T {
    if arr.is_empty() {
        panic!("Cannot select from empty array");
    }
    if i == 0 || i > arr.len() {
        panic!("Order statistic i must be between 1 and {}", arr.len());
    }
    select(arr, 0, arr.len() - 1, i)
}

/// Finds the median in worst-case linear time
///
/// # Arguments
/// * `arr` - The array to search (modified in-place)
///
/// # Returns
/// The median element
///
/// # Example
/// ```
/// use clrs::chapter_09::median;
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let med = median(&mut arr);
/// ```
pub fn median<T: Ord + Clone>(arr: &mut [T]) -> T {
    if arr.is_empty() {
        panic!("Cannot find median of empty array");
    }
    let n = arr.len();
    let i = n.div_ceil(2); // Median position
    select_full(arr, i)
}

/// Makes quicksort run in O(n lg n) worst-case (Exercise 9.3-3)
///
/// Uses SELECT to find the median as pivot, guaranteeing balanced partitions.
///
/// # Arguments
/// * `arr` - The array to sort (modified in-place)
/// * `p` - Start index (0-based)
/// * `r` - End index (0-based, inclusive)
///
/// # Complexity
/// - Time: O(n lg n) worst-case
/// - Space: O(lg n) for recursion
pub fn quicksort_with_median_pivot<T: Ord + Clone>(arr: &mut [T], p: usize, r: usize) {
    if p < r {
        // Find median to use as pivot
        let n = r - p + 1;
        let median_pos = n.div_ceil(2);
        let _median_val = select(arr, p, r, median_pos);
        
        // The median is now at position p + median_pos - 1
        let pivot_idx = p + median_pos - 1;
        arr.swap(pivot_idx, r);
        
        let q = partition(arr, p, r);
        
        if q > 0 {
            quicksort_with_median_pivot(arr, p, q - 1);
        }
        quicksort_with_median_pivot(arr, q + 1, r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_minimum() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let min = select_full(&mut arr, 1);
        assert_eq!(min, 1);
    }

    #[test]
    fn test_select_maximum() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let max = select_full(&mut arr, 8);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_select_median() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let med = median(&mut arr);
        // Median of sorted [1, 1, 2, 3, 4, 5, 6, 9] is 3 or 4
        assert!(med == 3 || med == 4);
    }

    #[test]
    fn test_select_single() {
        let mut arr = vec![42];
        let result = select_full(&mut arr, 1);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_select_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let third = select_full(&mut arr, 3);
        assert_eq!(third, 3);
    }

    #[test]
    fn test_quicksort_with_median_pivot() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort_with_median_pivot(&mut arr, 0, 7);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
}

