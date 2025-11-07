//! Bucket Sort (Section 8.4)
//!
//! Bucket sort assumes that the input is drawn from a uniform distribution
//! and has an average-case running time of O(n).

/// Sorts an array of floats in range [0.0, 1.0) using bucket sort
///
/// This corresponds to BUCKET-SORT from CLRS Section 8.4.
/// The algorithm assumes input is uniformly distributed in [0.0, 1.0).
///
/// # Arguments
/// * `arr` - The array to be sorted (must contain floats in [0.0, 1.0))
///
/// # Returns
/// A new sorted vector
///
/// # Complexity
/// - Average case: Θ(n)
/// - Worst case: Θ(n²) if all elements fall in the same bucket
/// - Space: Θ(n)
///
/// # Example
/// ```
/// use clrs::chapter_08::bucket_sort;
/// let arr = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.20, 0.89, 0.53, 0.71, 0.42];
/// let sorted = bucket_sort(&arr);
/// assert_eq!(sorted, vec![0.13, 0.16, 0.20, 0.39, 0.42, 0.53, 0.64, 0.71, 0.79, 0.89]);
/// ```
pub fn bucket_sort(arr: &[f64]) -> Vec<f64> {
    let n = arr.len();
    
    if n == 0 {
        return vec![];
    }
    
    // CLRS: let B[0..n-1] be a new array
    let mut b: Vec<Vec<f64>> = vec![Vec::new(); n];
    
    // CLRS: for i = 0 to n - 1
    // CLRS: insert A[i] into list B[floor(n * A[i])]
    for &value in arr {
        if !(0.0..1.0).contains(&value) {
            panic!("Element {} is not in range [0.0, 1.0)", value);
        }
        let index = (n as f64 * value) as usize;
        // Handle edge case where value is exactly 1.0 (shouldn't happen per spec)
        let index = index.min(n - 1);
        b[index].push(value);
    }
    
    // CLRS: for i = 0 to n - 1
    // CLRS: sort list B[i] with insertion sort
    for bucket in &mut b {
        insertion_sort_bucket(bucket);
    }
    
    // CLRS: concatenate the lists B[0], B[1], ..., B[n-1] together in order
    b.into_iter().flatten().collect()
}

/// Insertion sort for a bucket (used as subroutine in bucket sort)
///
/// This is optimized for small lists, which is typical in bucket sort.
fn insertion_sort_bucket(arr: &mut [f64]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        arr[j] = key;
    }
}

/// Sorts an array in-place using bucket sort
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
///
/// # Example
/// ```
/// use clrs::chapter_08::bucket_sort_inplace;
/// let mut arr = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.20];
/// bucket_sort_inplace(&mut arr);
/// assert_eq!(arr, vec![0.13, 0.16, 0.20, 0.39, 0.64, 0.79]);
/// ```
pub fn bucket_sort_inplace(arr: &mut [f64]) {
    let sorted = bucket_sort(arr);
    arr.copy_from_slice(&sorted);
}

/// Bucket sort with merge sort for worst-case O(n lg n) (Exercise 8.4-2)
///
/// This version uses merge sort instead of insertion sort to improve
/// worst-case performance from Θ(n²) to O(n lg n).
///
/// # Arguments
/// * `arr` - The array to be sorted (must contain floats in [0.0, 1.0))
///
/// # Returns
/// A new sorted vector
///
/// # Complexity
/// - Worst case: O(n lg n)
/// - Average case: Θ(n)
/// - Space: Θ(n)
pub fn bucket_sort_merge_sort(arr: &[f64]) -> Vec<f64> {
    let n = arr.len();
    
    if n == 0 {
        return vec![];
    }
    
    let mut b: Vec<Vec<f64>> = vec![Vec::new(); n];
    
    for &value in arr {
        if !(0.0..1.0).contains(&value) {
            panic!("Element {} is not in range [0.0, 1.0)", value);
        }
        let index = (n as f64 * value) as usize;
        let index = index.min(n - 1);
        b[index].push(value);
    }
    
    // Sort each bucket using merge sort
    for bucket in &mut b {
        merge_sort_bucket(bucket);
    }
    
    b.into_iter().flatten().collect()
}

/// Merge sort for a bucket (used as subroutine)
fn merge_sort_bucket(arr: &mut [f64]) {
    if arr.len() <= 1 {
        return;
    }
    
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    
    merge_sort_bucket(left);
    merge_sort_bucket(right);
    
    merge_buckets(arr, mid);
}

/// Merge two sorted halves of a bucket
fn merge_buckets(arr: &mut [f64], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort_basic() {
        let arr = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.20, 0.89, 0.53, 0.71, 0.42];
        let sorted = bucket_sort(&arr);
        let expected = vec![0.13, 0.16, 0.20, 0.39, 0.42, 0.53, 0.64, 0.71, 0.79, 0.89];
        
        // Use approximate comparison for floats
        for (a, b) in sorted.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10, "Expected {}, got {}", b, a);
        }
    }

    #[test]
    fn test_bucket_sort_clrs_example() {
        // Example from CLRS 8.4-1
        let arr = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.20, 0.89, 0.53, 0.71, 0.42];
        let sorted = bucket_sort(&arr);
        let expected = vec![0.13, 0.16, 0.20, 0.39, 0.42, 0.53, 0.64, 0.71, 0.79, 0.89];
        
        assert_eq!(sorted.len(), expected.len());
        for (a, b) in sorted.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_bucket_sort_single_element() {
        let arr = vec![0.5];
        let sorted = bucket_sort(&arr);
        assert_eq!(sorted, vec![0.5]);
    }

    #[test]
    fn test_bucket_sort_already_sorted() {
        let arr = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let sorted = bucket_sort(&arr);
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        
        for (a, b) in sorted.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_bucket_sort_inplace() {
        let mut arr = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.20];
        bucket_sort_inplace(&mut arr);
        let expected = vec![0.13, 0.16, 0.20, 0.39, 0.64, 0.79];
        
        for (a, b) in arr.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_bucket_sort_merge_sort() {
        let arr = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.20, 0.89, 0.53, 0.71, 0.42];
        let sorted = bucket_sort_merge_sort(&arr);
        let expected = vec![0.13, 0.16, 0.20, 0.39, 0.42, 0.53, 0.64, 0.71, 0.79, 0.89];
        
        for (a, b) in sorted.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_bucket_sort_empty() {
        let arr: Vec<f64> = vec![];
        let sorted = bucket_sort(&arr);
        assert!(sorted.is_empty());
    }
}

