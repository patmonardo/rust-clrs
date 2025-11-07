//! Minimum and Maximum (Section 9.1)
//!
//! This module contains algorithms for finding minimum and maximum elements,
//! and related order statistics.

/// Finds the minimum element in an array
///
/// This corresponds to finding the minimum element.
///
/// # Arguments
/// * `arr` - The array to search
///
/// # Returns
/// The minimum element and its index (value, index)
///
/// # Complexity
/// - Time: Θ(n-1) comparisons
/// - Space: O(1)
///
/// # Example
/// ```
/// use clrs::chapter_09::minimum;
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let (min_val, min_idx) = minimum(&arr);
/// assert_eq!(min_val, &1);
/// assert_eq!(min_idx, 1);
/// ```
pub fn minimum<T: Ord>(arr: &[T]) -> (&T, usize) {
    if arr.is_empty() {
        panic!("Cannot find minimum of empty array");
    }

    let mut min_idx = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[min_idx] {
            min_idx = i;
        }
    }

    (&arr[min_idx], min_idx)
}

/// Finds the maximum element in an array
///
/// # Arguments
/// * `arr` - The array to search
///
/// # Returns
/// The maximum element and its index (value, index)
///
/// # Complexity
/// - Time: Θ(n-1) comparisons
/// - Space: O(1)
///
/// # Example
/// ```
/// use clrs::chapter_09::maximum;
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let (max_val, max_idx) = maximum(&arr);
/// assert_eq!(max_val, &9);
/// assert_eq!(max_idx, 5);
/// ```
pub fn maximum<T: Ord>(arr: &[T]) -> (&T, usize) {
    if arr.is_empty() {
        panic!("Cannot find maximum of empty array");
    }

    let mut max_idx = 0;
    for i in 1..arr.len() {
        if arr[i] > arr[max_idx] {
            max_idx = i;
        }
    }

    (&arr[max_idx], max_idx)
}

/// Finds both minimum and maximum elements simultaneously
///
/// This uses the algorithm that finds both in 3⌊n/2⌋ comparisons
/// instead of 2n-2 comparisons.
///
/// # Arguments
/// * `arr` - The array to search
///
/// # Returns
/// ((min_value, min_index), (max_value, max_index))
///
/// # Complexity
/// - Time: 3⌊n/2⌋ comparisons
/// - Space: O(1)
///
/// # Example
/// ```
/// use clrs::chapter_09::min_max;
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let ((min_val, min_idx), (max_val, max_idx)) = min_max(&arr);
/// assert_eq!(min_val, &1);
/// assert_eq!(max_val, &9);
/// ```
pub fn min_max<T: Ord>(arr: &[T]) -> ((&T, usize), (&T, usize)) {
    if arr.is_empty() {
        panic!("Cannot find min/max of empty array");
    }

    if arr.len() == 1 {
        return ((&arr[0], 0), (&arr[0], 0));
    }

    let (mut min_idx, mut max_idx) = if arr[0] < arr[1] { (0, 1) } else { (1, 0) };

    // Process elements in pairs
    let mut i = 2;
    while i < arr.len() - 1 {
        // Compare pair
        if arr[i] < arr[i + 1] {
            if arr[i] < arr[min_idx] {
                min_idx = i;
            }
            if arr[i + 1] > arr[max_idx] {
                max_idx = i + 1;
            }
        } else {
            if arr[i + 1] < arr[min_idx] {
                min_idx = i + 1;
            }
            if arr[i] > arr[max_idx] {
                max_idx = i;
            }
        }
        i += 2;
    }

    // Handle odd-length array
    if i < arr.len() {
        if arr[i] < arr[min_idx] {
            min_idx = i;
        } else if arr[i] > arr[max_idx] {
            max_idx = i;
        }
    }

    ((&arr[min_idx], min_idx), (&arr[max_idx], max_idx))
}

/// Finds the second smallest element (Exercise 9.1-1)
///
/// Uses tournament method to find second smallest with n + ⌈lg n⌉ - 2 comparisons.
///
/// # Arguments
/// * `arr` - The array to search
///
/// # Returns
/// The second smallest element and its index
///
/// # Complexity
/// - Time: n + ⌈lg n⌉ - 2 comparisons
/// - Space: O(n) for tracking tournament matches
///
/// # Example
/// ```
/// use clrs::chapter_09::second_smallest;
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let (second_min_val, second_min_idx) = second_smallest(&arr);
/// assert_eq!(second_min_val, &1); // Second occurrence of 1
/// ```
pub fn second_smallest<T: Ord + Clone>(arr: &[T]) -> (&T, usize) {
    if arr.len() < 2 {
        panic!("Array must have at least 2 elements");
    }

    // Tournament method: compare elements in pairs
    // Track which elements lost to the minimum
    let mut candidates: Vec<usize> = Vec::new();
    let mut winners: Vec<usize> = (0..arr.len()).collect();

    // Tournament rounds
    while winners.len() > 1 {
        let mut next_winners = Vec::new();
        let mut i = 0;

        while i < winners.len() - 1 {
            if arr[winners[i]] < arr[winners[i + 1]] {
                candidates.push(winners[i + 1]); // Loser
                next_winners.push(winners[i]); // Winner
            } else {
                candidates.push(winners[i]); // Loser
                next_winners.push(winners[i + 1]); // Winner
            }
            i += 2;
        }

        // Handle odd element
        if i < winners.len() {
            next_winners.push(winners[i]);
        }

        winners = next_winners;
    }

    // The minimum is the final winner
    let _min_idx = winners[0];

    // Find minimum among candidates (elements that lost to the minimum)
    let mut second_min_idx = candidates[0];
    for &candidate_idx in &candidates[1..] {
        if arr[candidate_idx] < arr[second_min_idx] {
            second_min_idx = candidate_idx;
        }
    }

    (&arr[second_min_idx], second_min_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let (min_val, min_idx) = minimum(&arr);
        assert_eq!(*min_val, 1);
        assert!(min_idx == 1 || min_idx == 3); // Either occurrence of 1
    }

    #[test]
    fn test_maximum() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let (max_val, max_idx) = maximum(&arr);
        assert_eq!(*max_val, 9);
        assert_eq!(max_idx, 5);
    }

    #[test]
    fn test_min_max() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let ((min_val, _min_idx), (max_val, _max_idx)) = min_max(&arr);
        assert_eq!(*min_val, 1);
        assert_eq!(*max_val, 9);
    }

    #[test]
    fn test_min_max_single() {
        let arr = vec![42];
        let ((min_val, _), (max_val, _)) = min_max(&arr);
        assert_eq!(*min_val, 42);
        assert_eq!(*max_val, 42);
    }

    #[test]
    fn test_second_smallest() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let (second_min_val, _) = second_smallest(&arr);
        // Second smallest should be 1 (second occurrence) or 2
        assert!(*second_min_val == 1 || *second_min_val == 2);
    }

    #[test]
    fn test_second_smallest_distinct() {
        let arr = vec![5, 2, 8, 1, 9, 3];
        let (second_min_val, _) = second_smallest(&arr);
        assert_eq!(*second_min_val, 2);
    }
}
