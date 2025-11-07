//! Binary Search (Section 2.3, Exercise 2.3-5)
//!
//! Binary search is a search algorithm that finds the position of a target value
//! within a sorted array by repeatedly dividing the search interval in half.

/// Performs iterative binary search on a sorted array
///
/// This corresponds to ITERATIVE-BINARY-SEARCH from CLRS Exercise 2.3-5.
///
/// # Arguments
/// * `arr` - A sorted array to search
/// * `v` - The value to search for
/// * `low` - Lower bound (0-based index)
/// * `high` - Upper bound (0-based index, inclusive)
///
/// # Returns
/// * `Some(index)` - The 0-based index where v was found
/// * `None` - If v does not appear in the array
///
/// # Example
/// ```
/// use clrs::chapter_02::iterative_binary_search;
/// let arr = vec![1, 3, 5, 7, 9, 11, 13];
/// assert_eq!(iterative_binary_search(&arr, &7, 0, 6), Some(3));
/// assert_eq!(iterative_binary_search(&arr, &10, 0, 6), None);
/// ```
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(1)
pub fn iterative_binary_search<T: Ord>(arr: &[T], v: &T, low: usize, high: usize) -> Option<usize> {
    // Check for empty array or invalid bounds
    if arr.is_empty() || low > high || high >= arr.len() {
        return None;
    }
    
    let mut low = low;
    let mut high = high;
    
    // CLRS: while low ≤ high
    while low <= high {
        // CLRS: mid = floor((low + high) / 2)
        let mid = (low + high) / 2;
        
        // CLRS: if v == A[mid]
        if arr[mid] == *v {
            return Some(mid);
        } else if arr[mid] < *v {
            // CLRS: low = mid + 1
            if mid == usize::MAX {
                return None; // Overflow protection
            }
            low = mid + 1;
        } else {
            // CLRS: high = mid - 1
            // Use checked subtraction to avoid underflow
            high = mid.checked_sub(1)?;
        }
    }
    
    // CLRS: return NIL
    None
}

/// Performs recursive binary search on a sorted array
///
/// This corresponds to RECURSIVE-BINARY-SEARCH from CLRS Exercise 2.3-5.
///
/// # Arguments
/// * `arr` - A sorted array to search
/// * `v` - The value to search for
/// * `low` - Lower bound (0-based index)
/// * `high` - Upper bound (0-based index, inclusive)
///
/// # Returns
/// * `Some(index)` - The 0-based index where v was found
/// * `None` - If v does not appear in the array
///
/// # Example
/// ```
/// use clrs::chapter_02::recursive_binary_search;
/// let arr = vec![1, 3, 5, 7, 9, 11, 13];
/// assert_eq!(recursive_binary_search(&arr, &7, 0, 6), Some(3));
/// assert_eq!(recursive_binary_search(&arr, &10, 0, 6), None);
/// ```
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(log n) due to recursion stack
pub fn recursive_binary_search<T: Ord>(arr: &[T], v: &T, low: usize, high: usize) -> Option<usize> {
    // CLRS: if low > high
    if low > high {
        // CLRS: return NIL
        return None;
    }
    
    // CLRS: mid = floor((low + high) / 2)
    let mid = (low + high) / 2;
    
    // CLRS: if v == A[mid]
    if arr[mid] == *v {
        Some(mid)
    } else if arr[mid] < *v {
        // CLRS: return RECURSIVE-BINARY-SEARCH(A, v, mid + 1, high)
        recursive_binary_search(arr, v, mid + 1, high)
    } else {
        // CLRS: return RECURSIVE-BINARY-SEARCH(A, v, low, mid - 1)
        recursive_binary_search(arr, v, low, mid - 1)
    }
}

/// Convenience function for binary search on entire array
///
/// # Example
/// ```
/// use clrs::chapter_02::binary_search;
/// let arr = vec![1, 3, 5, 7, 9, 11, 13];
/// assert_eq!(binary_search(&arr, &7), Some(3));
/// assert_eq!(binary_search(&arr, &10), None);
/// ```
pub fn binary_search<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    iterative_binary_search(arr, v, 0, arr.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative_binary_search_found() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(iterative_binary_search(&arr, &7, 0, 6), Some(3));
        assert_eq!(iterative_binary_search(&arr, &1, 0, 6), Some(0));
        assert_eq!(iterative_binary_search(&arr, &13, 0, 6), Some(6));
    }

    #[test]
    fn test_iterative_binary_search_not_found() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(iterative_binary_search(&arr, &10, 0, 6), None);
        assert_eq!(iterative_binary_search(&arr, &0, 0, 6), None);
        assert_eq!(iterative_binary_search(&arr, &15, 0, 6), None);
    }

    #[test]
    fn test_iterative_binary_search_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(iterative_binary_search(&arr, &42, 0, 0), None);
        // Empty array should return None regardless of bounds
        assert_eq!(iterative_binary_search(&arr, &42, 0, 0), None);
    }

    #[test]
    fn test_recursive_binary_search() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(recursive_binary_search(&arr, &7, 0, 6), Some(3));
        assert_eq!(recursive_binary_search(&arr, &10, 0, 6), None);
    }

    #[test]
    fn test_binary_search_convenience() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(binary_search(&arr, &7), Some(3));
        assert_eq!(binary_search(&arr, &10), None);
    }
}

