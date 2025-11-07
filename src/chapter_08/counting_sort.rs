//! Counting Sort (Section 8.2)
//!
//! Counting sort assumes that each of the n input elements is an integer
//! in the range 0 to k, for some integer k. It determines, for each input
//! element x, the number of elements less than x.

/// Sorts an array of integers using counting sort
///
/// This corresponds to COUNTING-SORT from CLRS Section 8.2.
/// The algorithm assumes that all elements are in the range [0, k].
///
/// # Arguments
/// * `arr` - The array to be sorted (must contain integers in range [0, k])
/// * `k` - The maximum value in the array (all elements must be <= k)
///
/// # Returns
/// A new sorted vector
///
/// # Complexity
/// - Time: Θ(n + k)
/// - Space: Θ(n + k)
///
/// # Example
/// ```
/// use clrs::chapter_08::counting_sort;
/// let arr = vec![6, 0, 2, 0, 1, 3, 4, 6, 1, 3, 2];
/// let sorted = counting_sort(&arr, 6);
/// assert_eq!(sorted, vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
/// ```
pub fn counting_sort(arr: &[usize], k: usize) -> Vec<usize> {
    let n = arr.len();
    
    // CLRS: let C[0..k] be a new array
    let mut c = vec![0; k + 1];
    
    // CLRS: for j = 1 to A.length
    // CLRS: C[A[j]] = C[A[j]] + 1
    for &value in arr {
        if value > k {
            panic!("Element {} exceeds maximum value k = {}", value, k);
        }
        c[value] += 1;
    }
    
    // CLRS: for i = 1 to k
    // CLRS: C[i] = C[i] + C[i - 1]
    // C[i] now contains the number of elements <= i
    for i in 1..=k {
        c[i] += c[i - 1];
    }
    
    // CLRS: let B[1..n] be a new array
    let mut b = vec![0; n];
    
    // CLRS: for j = A.length downto 1
    // Process in reverse to maintain stability
    for &value in arr.iter().rev() {
        // CLRS: B[C[A[j]]] = A[j]
        b[c[value] - 1] = value;
        // CLRS: C[A[j]] = C[A[j]] - 1
        c[value] -= 1;
    }
    
    b
}

/// Sorts an array in-place using counting sort
///
/// This version modifies the input array directly.
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
/// * `k` - The maximum value in the array
///
/// # Example
/// ```
/// use clrs::chapter_08::counting_sort_inplace;
/// let mut arr = vec![6, 0, 2, 0, 1, 3, 4, 6, 1, 3, 2];
/// counting_sort_inplace(&mut arr, 6);
/// assert_eq!(arr, vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
/// ```
pub fn counting_sort_inplace(arr: &mut [usize], k: usize) {
    let sorted = counting_sort(arr, k);
    arr.copy_from_slice(&sorted);
}

/// Preprocesses an array for range queries (Exercise 8.2-4)
///
/// After preprocessing, queries about how many integers fall into
/// a range [a..b] can be answered in O(1) time.
///
/// # Arguments
/// * `arr` - The array to preprocess
/// * `k` - The maximum value in the array
///
/// # Returns
/// A cumulative count array where C[i] contains the number of elements <= i
///
/// # Example
/// ```
/// use clrs::chapter_08::counting_sort_preprocess;
/// let arr = vec![2, 5, 3, 0, 2, 3, 0, 3];
/// let c = counting_sort_preprocess(&arr, 5);
/// // How many elements in range [1..4]?
/// assert_eq!(c[4] - c[0], 6); // Elements 2, 5, 3, 2, 3, 3
/// ```
pub fn counting_sort_preprocess(arr: &[usize], k: usize) -> Vec<usize> {
    let mut c = vec![0; k + 1];
    
    // Count occurrences
    for &value in arr {
        if value > k {
            panic!("Element {} exceeds maximum value k = {}", value, k);
        }
        c[value] += 1;
    }
    
    // Make cumulative
    for i in 1..=k {
        c[i] += c[i - 1];
    }
    
    c
}

/// Queries how many elements fall into range [a..b] (Exercise 8.2-4)
///
/// # Arguments
/// * `c` - The preprocessed cumulative count array
/// * `a` - Lower bound (inclusive)
/// * `b` - Upper bound (inclusive)
///
/// # Returns
/// The number of elements in the range [a..b]
///
/// # Example
/// ```
/// use clrs::chapter_08::{counting_sort_preprocess, counting_sort_query};
/// let arr = vec![2, 5, 3, 0, 2, 3, 0, 3];
/// let c = counting_sort_preprocess(&arr, 5);
/// assert_eq!(counting_sort_query(&c, 1, 4), 6);
/// ```
pub fn counting_sort_query(c: &[usize], a: usize, b: usize) -> usize {
    if a == 0 {
        c[b]
    } else {
        c[b] - c[a - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort_basic() {
        let arr = vec![4, 2, 2, 8, 3, 3, 1];
        let sorted = counting_sort(&arr, 8);
        assert_eq!(sorted, vec![1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_counting_sort_clrs_example() {
        // Example from CLRS 8.2-1
        let arr = vec![6, 0, 2, 0, 1, 3, 4, 6, 1, 3, 2];
        let sorted = counting_sort(&arr, 6);
        assert_eq!(sorted, vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }

    #[test]
    fn test_counting_sort_single_element() {
        let arr = vec![5];
        let sorted = counting_sort(&arr, 5);
        assert_eq!(sorted, vec![5]);
    }

    #[test]
    fn test_counting_sort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        let sorted = counting_sort(&arr, 5);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counting_sort_all_same() {
        let arr = vec![3, 3, 3, 3, 3];
        let sorted = counting_sort(&arr, 3);
        assert_eq!(sorted, vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_counting_sort_inplace() {
        let mut arr = vec![6, 0, 2, 0, 1, 3, 4, 6, 1, 3, 2];
        counting_sort_inplace(&mut arr, 6);
        assert_eq!(arr, vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 6, 6]);
    }

    #[test]
    fn test_counting_sort_preprocess_and_query() {
        let arr = vec![2, 5, 3, 0, 2, 3, 0, 3];
        let c = counting_sort_preprocess(&arr, 5);
        
        // Query range [0..5] (all elements)
        assert_eq!(counting_sort_query(&c, 0, 5), 8);
        
        // Query range [1..4]: elements with values 1, 2, 3, 4
        // Values: 2, 3, 2, 3, 3 = 5 elements (no 1 or 4, but 2 and 3)
        // Actually: 2 appears twice, 3 appears 3 times = 5 total
        // But wait, let's recalculate: arr has 2, 5, 3, 0, 2, 3, 0, 3
        // In range [1..4]: 2, 3, 2, 3, 3 = 5 elements
        assert_eq!(counting_sort_query(&c, 1, 4), 5);
        
        // Query range [3..3]: elements with value 3
        assert_eq!(counting_sort_query(&c, 3, 3), 3);
    }
}

