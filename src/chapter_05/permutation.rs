//! Randomized Permutation Algorithms (Section 5.3)
//!
//! This module implements algorithms for generating random permutations,
//! including RANDOMIZE-IN-PLACE (Fisher-Yates shuffle), PERMUTE-BY-SORTING,
//! and RANDOM-SAMPLE.

use rand::Rng;

/// Randomizes an array in place using the Fisher-Yates shuffle
///
/// This corresponds to RANDOMIZE-IN-PLACE from CLRS Section 5.3.
/// Produces a uniform random permutation of the input array.
///
/// # Arguments
/// * `arr` - Mutable slice to randomize in place
///
/// # Example
/// ```
/// use clrs::chapter_05::randomize_in_place;
/// let mut arr = vec![1, 2, 3, 4, 5];
/// randomize_in_place(&mut arr);
/// // arr is now a random permutation of [1, 2, 3, 4, 5]
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
pub fn randomize_in_place<T>(arr: &mut [T]) {
    let mut rng = rand::thread_rng();
    let n = arr.len();
    
    // CLRS: for i = 1 to n
    for i in 0..n {
        // CLRS: swap A[i] with A[RANDOM(i, n)]
        let j = rng.gen_range(i..n);
        arr.swap(i, j);
    }
}

/// Generates a random permutation by assigning random priorities and sorting
///
/// This corresponds to PERMUTE-BY-SORTING from CLRS Section 5.3.
/// Creates a new array with random priorities and sorts by those priorities.
///
/// # Arguments
/// * `arr` - The array to permute
///
/// # Returns
/// A new vector containing a random permutation of the input
///
/// # Note
/// This method requires O(n log n) time due to sorting, while
/// RANDOMIZE-IN-PLACE requires only O(n) time.
///
/// # Example
/// ```
/// use clrs::chapter_05::permute_by_sorting;
/// let arr = vec![1, 2, 3, 4, 5];
/// let permuted = permute_by_sorting(&arr);
/// // permuted is a random permutation of [1, 2, 3, 4, 5]
/// assert_eq!(permuted.len(), arr.len());
/// ```
///
/// # Complexity
/// - Time: O(n log n) due to sorting
/// - Space: O(n)
pub fn permute_by_sorting<T: Clone>(arr: &[T]) -> Vec<T> {
    let n = arr.len();
    let mut rng = rand::thread_rng();
    
    // CLRS: let P[1..n] be a new array
    // CLRS: for i = 1 to n, P[i] = RANDOM(1, n³)
    let n_cubed = (n * n * n) as i32;
    let mut priorities: Vec<(usize, i32)> = (0..n)
        .map(|i| {
            // Generate random priority in range [1, n³]
            let priority = rng.gen_range(1..=n_cubed);
            (i, priority)
        })
        .collect();
    
    // CLRS: sort A, using P as sort keys
    priorities.sort_by_key(|&(_, priority)| priority);
    
    // Build the permuted array
    priorities.iter().map(|&(idx, _)| arr[idx].clone()).collect()
}

/// Generates a random m-element subset of {1, 2, ..., n}
///
/// This corresponds to RANDOM-SAMPLE from CLRS Exercise 5.3-7.
/// Returns a random m-element subset where each m-subset is equally likely.
///
/// # Arguments
/// * `m` - Size of the subset to generate
/// * `n` - Upper bound of the set {1, 2, ..., n}
///
/// # Returns
/// A sorted vector containing m distinct numbers from [1, n]
///
/// # Panics
/// Panics if m > n
///
/// # Example
/// ```
/// use clrs::chapter_05::random_sample;
/// let sample = random_sample(3, 10);
/// assert_eq!(sample.len(), 3);
/// assert!(sample.iter().all(|&x| x >= 1 && x <= 10));
/// assert_eq!(sample.iter().collect::<std::collections::HashSet<_>>().len(), 3);
/// ```
///
/// # Complexity
/// - Time: O(m) - makes m calls to RANDOM
/// - Space: O(m)
pub fn random_sample(m: usize, n: usize) -> Vec<usize> {
    if m == 0 {
        return vec![];
    }
    
    if m > n {
        panic!("Cannot sample {} elements from set of size {}", m, n);
    }
    
    // CLRS: S = RANDOM-SAMPLE(m - 1, n - 1)
    let mut s = random_sample(m - 1, n - 1);
    
    // CLRS: i = RANDOM(1, n)
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(1..=n);
    
    // CLRS: if i ∈ S, then S = S ∪ {n}, else S = S ∪ {i}
    if s.contains(&i) {
        s.push(n);
    } else {
        s.push(i);
    }
    
    s.sort();
    s
}

/// Alternative implementation: Random sample using RANDOMIZE-IN-PLACE
///
/// This is the straightforward approach: initialize array [1, 2, ..., n],
/// randomize it, and take the first m elements.
///
/// # Arguments
/// * `m` - Size of the subset to generate
/// * `n` - Upper bound of the set {1, 2, ..., n}
///
/// # Returns
/// A vector containing m distinct numbers from [1, n]
///
/// # Complexity
/// - Time: O(n) - must randomize entire array
/// - Space: O(n)
pub fn random_sample_alternative(m: usize, n: usize) -> Vec<usize> {
    if m > n {
        panic!("Cannot sample {} elements from set of size {}", m, n);
    }
    
    // Create array [1, 2, ..., n]
    let mut arr: Vec<usize> = (1..=n).collect();
    
    // Randomize in place
    randomize_in_place(&mut arr);
    
    // Take first m elements
    arr.truncate(m);
    arr.sort();
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_randomize_in_place() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let original = arr.clone();
        randomize_in_place(&mut arr);
        
        // Should contain same elements
        arr.sort();
        assert_eq!(arr, original);
    }

    #[test]
    fn test_randomize_in_place_preserves_elements() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let original_set: HashSet<_> = arr.iter().cloned().collect();
        randomize_in_place(&mut arr);
        let permuted_set: HashSet<_> = arr.iter().cloned().collect();
        
        assert_eq!(original_set, permuted_set);
    }

    #[test]
    fn test_permute_by_sorting() {
        let arr = vec![1, 2, 3, 4, 5];
        let permuted = permute_by_sorting(&arr);
        
        // Should have same length
        assert_eq!(permuted.len(), arr.len());
        
        // Should contain same elements
        let original_set: HashSet<_> = arr.iter().cloned().collect();
        let permuted_set: HashSet<_> = permuted.iter().cloned().collect();
        assert_eq!(original_set, permuted_set);
    }

    #[test]
    fn test_random_sample() {
        let sample = random_sample(3, 10);
        
        assert_eq!(sample.len(), 3);
        assert!(sample.iter().all(|&x| x >= 1 && x <= 10));
        
        // Check all elements are distinct
        let unique: HashSet<_> = sample.iter().cloned().collect();
        assert_eq!(unique.len(), 3);
        
        // Should be sorted
        let mut sorted = sample.clone();
        sorted.sort();
        assert_eq!(sample, sorted);
    }

    #[test]
    fn test_random_sample_zero() {
        let sample = random_sample(0, 10);
        assert!(sample.is_empty());
    }

    #[test]
    fn test_random_sample_full() {
        let sample = random_sample(5, 5);
        assert_eq!(sample.len(), 5);
        let unique: HashSet<_> = sample.iter().cloned().collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    #[should_panic(expected = "Cannot sample")]
    fn test_random_sample_invalid() {
        random_sample(10, 5);
    }

    #[test]
    fn test_random_sample_alternative() {
        let sample = random_sample_alternative(3, 10);
        
        assert_eq!(sample.len(), 3);
        assert!(sample.iter().all(|&x| x >= 1 && x <= 10));
        
        // Check all elements are distinct
        let unique: HashSet<_> = sample.iter().cloned().collect();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn test_both_sample_methods() {
        // Both methods should produce valid samples
        for _ in 0..10 {
            let s1 = random_sample(5, 20);
            let s2 = random_sample_alternative(5, 20);
            
            assert_eq!(s1.len(), 5);
            assert_eq!(s2.len(), 5);
            
            let set1: HashSet<_> = s1.iter().cloned().collect();
            let set2: HashSet<_> = s2.iter().cloned().collect();
            assert_eq!(set1.len(), 5);
            assert_eq!(set2.len(), 5);
        }
    }
}

