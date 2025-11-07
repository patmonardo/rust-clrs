//! Maximum Subarray Problem (Section 4.1)
//!
//! The maximum subarray problem is the task of finding the contiguous subarray
//! within a one-dimensional array of numbers that has the largest sum.

/// Result of maximum subarray computation
///
/// Contains the indices and sum of the maximum subarray.
#[derive(Debug, Clone, PartialEq)]
pub struct MaximumSubarrayResult {
    /// Starting index (0-based) of the maximum subarray
    pub low: usize,
    /// Ending index (0-based, inclusive) of the maximum subarray
    pub high: usize,
    /// Sum of elements in the maximum subarray
    pub sum: i64,
}

/// Finds the maximum subarray that crosses the midpoint
///
/// This corresponds to FIND-MAX-CROSSING-SUBARRAY from CLRS Section 4.1.
/// This is a helper function used by the divide-and-conquer algorithm.
///
/// # Arguments
/// * `arr` - The array to search
/// * `low` - Start index (0-based)
/// * `mid` - Middle index (0-based)
/// * `high` - End index (0-based, inclusive)
///
/// # Returns
/// MaximumSubarrayResult with indices and sum of the maximum crossing subarray
///
/// # Complexity
/// - Time: O(n) where n = high - low + 1
/// - Space: O(1)
fn find_max_crossing_subarray(
    arr: &[i64],
    low: usize,
    mid: usize,
    high: usize,
) -> MaximumSubarrayResult {
    // CLRS: left-sum = -∞
    let mut left_sum = i64::MIN;
    let mut sum = 0;
    let mut max_left = mid;

    // CLRS: for i = mid downto low
    for i in (low..=mid).rev() {
        sum += arr[i];
        // CLRS: if sum > left-sum
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    // CLRS: right-sum = -∞
    let mut right_sum = i64::MIN;
    sum = 0;
    let mut max_right = mid;

    // CLRS: for j = mid + 1 to high
    for j in (mid + 1)..=high {
        sum += arr[j];
        // CLRS: if sum > right-sum
        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }
    }

    // CLRS: return (max-left, max-right, left-sum + right-sum)
    MaximumSubarrayResult {
        low: max_left,
        high: max_right,
        sum: left_sum + right_sum,
    }
}

/// Finds the maximum subarray using divide-and-conquer
///
/// This corresponds to FIND-MAXIMUM-SUBARRAY from CLRS Section 4.1.
///
/// # Arguments
/// * `arr` - The array to search
/// * `low` - Start index (0-based)
/// * `high` - End index (0-based, inclusive)
///
/// # Returns
/// MaximumSubarrayResult with indices and sum of the maximum subarray
///
/// # Example
/// ```
/// use clrs::chapter_04::{find_maximum_subarray, MaximumSubarrayResult};
/// let arr = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
/// let result = find_maximum_subarray(&arr, 0, arr.len() - 1);
/// assert_eq!(result, MaximumSubarrayResult { low: 7, high: 10, sum: 43 });
/// ```
///
/// # Complexity
/// - Time: O(n log n)
/// - Space: O(log n) due to recursion stack
pub fn find_maximum_subarray(arr: &[i64], low: usize, high: usize) -> MaximumSubarrayResult {
    // CLRS: if high == low
    if high == low {
        // CLRS: return (low, high, A[low])
        return MaximumSubarrayResult {
            low,
            high,
            sum: arr[low],
        };
    }

    // CLRS: else mid = floor((low + high) / 2)
    let mid = (low + high) / 2;

    // CLRS: (left-low, left-high, left-sum) = FIND-MAXIMUM-SUBARRAY(A, low, mid)
    let left = find_maximum_subarray(arr, low, mid);

    // CLRS: (right-low, right-high, right-sum) = FIND-MAXIMUM-SUBARRAY(A, mid + 1, high)
    let right = find_maximum_subarray(arr, mid + 1, high);

    // CLRS: (cross-low, cross-high, cross-sum) = FIND-MAX-CROSSING-SUBARRAY(A, low, mid, high)
    let cross = find_max_crossing_subarray(arr, low, mid, high);

    // CLRS: if left-sum >= right-sum and left-sum >= cross-sum
    if left.sum >= right.sum && left.sum >= cross.sum {
        left
    }
    // CLRS: elseif right-sum >= left-sum and right-sum >= cross-sum
    else if right.sum >= left.sum && right.sum >= cross.sum {
        right
    }
    // CLRS: else return (cross-low, cross-high, cross-sum)
    else {
        cross
    }
}

/// Finds the maximum subarray using brute force
///
/// This corresponds to BRUTE-FORCE-FIND-MAXIMUM-SUBARRAY from CLRS Exercise 4.1-2.
///
/// # Arguments
/// * `arr` - The array to search
///
/// # Returns
/// MaximumSubarrayResult with indices and sum of the maximum subarray
///
/// # Example
/// ```
/// use clrs::chapter_04::{brute_force_find_maximum_subarray, MaximumSubarrayResult};
/// let arr = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
/// let result = brute_force_find_maximum_subarray(&arr);
/// assert_eq!(result, MaximumSubarrayResult { low: 7, high: 10, sum: 43 });
/// ```
///
/// # Complexity
/// - Time: Θ(n²)
/// - Space: O(1)
pub fn brute_force_find_maximum_subarray(arr: &[i64]) -> MaximumSubarrayResult {
    if arr.is_empty() {
        return MaximumSubarrayResult {
            low: 0,
            high: 0,
            sum: 0,
        };
    }

    let n = arr.len();
    // CLRS: max-sum = -∞
    let mut max_sum = i64::MIN;
    let mut low = 0;
    let mut high = 0;

    // CLRS: for l = 1 to n
    for l in 0..n {
        let mut sum = 0;
        // CLRS: for h = l to n
        for h in l..n {
            // CLRS: sum = sum + A[h]
            sum += arr[h];
            // CLRS: if sum > max-sum
            if sum > max_sum {
                max_sum = sum;
                low = l;
                high = h;
            }
        }
    }

    // CLRS: return (low, high, max-sum)
    MaximumSubarrayResult {
        low,
        high,
        sum: max_sum,
    }
}

/// Finds the maximum subarray using an iterative linear-time algorithm
///
/// This corresponds to ITERATIVE-FIND-MAXIMUM-SUBARRAY from CLRS Exercise 4.1-5.
///
/// # Arguments
/// * `arr` - The array to search
///
/// # Returns
/// MaximumSubarrayResult with indices and sum of the maximum subarray
///
/// # Example
/// ```
/// use clrs::chapter_04::{iterative_find_maximum_subarray, MaximumSubarrayResult};
/// let arr = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
/// let result = iterative_find_maximum_subarray(&arr);
/// assert_eq!(result, MaximumSubarrayResult { low: 7, high: 10, sum: 43 });
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
pub fn iterative_find_maximum_subarray(arr: &[i64]) -> MaximumSubarrayResult {
    if arr.is_empty() {
        return MaximumSubarrayResult {
            low: 0,
            high: 0,
            sum: 0,
        };
    }

    let n = arr.len();
    // CLRS: max-sum = -∞
    let mut max_sum = i64::MIN;
    // CLRS: sum = -∞
    let mut sum = i64::MIN;
    let mut low = 0;
    let mut high = 0;
    let mut current_low = 0;

    // CLRS: for j = 1 to n
    for j in 0..n {
        let current_high = j;
        // CLRS: if sum > 0
        if sum > 0 {
            // CLRS: sum = sum + A[j]
            sum += arr[j];
        } else {
            // CLRS: currentLow = j
            current_low = j;
            // CLRS: sum = A[j]
            sum = arr[j];
        }
        // CLRS: if sum > max-sum
        if sum > max_sum {
            max_sum = sum;
            low = current_low;
            high = current_high;
        }
    }

    // CLRS: return (low, high, max-sum)
    MaximumSubarrayResult {
        low,
        high,
        sum: max_sum,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_subarray_example() {
        // Example from CLRS Section 4.1
        let arr = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let result = find_maximum_subarray(&arr, 0, arr.len() - 1);
        // The maximum subarray is A[7..10] = [18, 20, -7, 12] with sum 43
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 7,
                high: 10,
                sum: 43
            }
        );
    }

    #[test]
    fn test_find_maximum_subarray_single_element() {
        let arr = vec![5];
        let result = find_maximum_subarray(&arr, 0, 0);
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 0,
                high: 0,
                sum: 5
            }
        );
    }

    #[test]
    fn test_find_maximum_subarray_all_negative() {
        // When all elements are negative, should return the least negative (largest) element
        let arr = vec![-5, -3, -8, -1];
        let result = find_maximum_subarray(&arr, 0, arr.len() - 1);
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 3,
                high: 3,
                sum: -1
            }
        );
    }

    #[test]
    fn test_find_maximum_subarray_all_positive() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = find_maximum_subarray(&arr, 0, arr.len() - 1);
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 0,
                high: 4,
                sum: 15
            }
        );
    }

    #[test]
    fn test_brute_force_find_maximum_subarray() {
        let arr = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let result = brute_force_find_maximum_subarray(&arr);
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 7,
                high: 10,
                sum: 43
            }
        );
    }

    #[test]
    fn test_iterative_find_maximum_subarray() {
        let arr = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let result = iterative_find_maximum_subarray(&arr);
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 7,
                high: 10,
                sum: 43
            }
        );
    }

    #[test]
    fn test_all_algorithms_agree() {
        let arr = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let recursive = find_maximum_subarray(&arr, 0, arr.len() - 1);
        let brute_force = brute_force_find_maximum_subarray(&arr);
        let iterative = iterative_find_maximum_subarray(&arr);

        assert_eq!(recursive.sum, brute_force.sum);
        assert_eq!(recursive.sum, iterative.sum);
    }

    #[test]
    fn test_empty_array() {
        let arr: Vec<i64> = vec![];
        let result = iterative_find_maximum_subarray(&arr);
        assert_eq!(result.sum, 0);
    }

    #[test]
    fn test_single_negative_element() {
        let arr = vec![-5];
        let result = find_maximum_subarray(&arr, 0, 0);
        assert_eq!(
            result,
            MaximumSubarrayResult {
                low: 0,
                high: 0,
                sum: -5
            }
        );
    }
}
