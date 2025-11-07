//! Algorithm Comparison Tools (Section 1.2)
//!
//! This module provides utilities for comparing different algorithms
//! and determining when one outperforms another.

/// Compares two algorithms to find when one beats the other
///
/// Solves problems like: "For which values of n does algorithm A beat algorithm B?"
///
/// # Arguments
/// * `time_a` - Function computing time for algorithm A: f(n) -> time
/// * `time_b` - Function computing time for algorithm B: f(n) -> time
/// * `max_n` - Maximum value of n to check
///
/// # Returns
/// Range of n values where algorithm A is faster than B (if any)
///
/// # Example
/// ```
/// use clrs::chapter_01::compare_algorithms;
/// // Insertion sort: 8n², Merge sort: 64n lg n
/// let insertion = |n: f64| 8.0 * n * n;
/// let merge = |n: f64| 64.0 * n * n.log2();
/// let result = compare_algorithms(insertion, merge, 1000.0);
/// // Returns the range where insertion sort is faster
/// ```
pub fn compare_algorithms<F1, F2>(
    time_a: F1,
    time_b: F2,
    max_n: f64,
) -> Option<(f64, f64)>
where
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
{
    let mut start_n: Option<f64> = None;
    let mut end_n: Option<f64> = None;
    let mut prev_a_faster = false;
    
    // Search for the crossover point
    let mut n = 1.0;
    let step = (max_n / 1000.0).max(1.0);
    
    while n <= max_n {
        let time_a_val = time_a(n);
        let time_b_val = time_b(n);
        
        let a_faster = time_a_val < time_b_val;
        
        // Detect transition point
        if a_faster && !prev_a_faster {
            start_n = Some(n);
        } else if !a_faster && prev_a_faster {
            end_n = Some(n);
            break;
        }
        
        prev_a_faster = a_faster;
        n += step;
    }
    
    match (start_n, end_n) {
        (Some(start), Some(end)) => Some((start, end)),
        (Some(start), None) => Some((start, max_n)),
        _ => None,
    }
}

/// Finds the smallest n where algorithm A becomes faster than algorithm B
///
/// Solves problems like: "What is the smallest n such that 100n² < 2^n?"
///
/// # Arguments
/// * `time_a` - Function computing time for algorithm A
/// * `time_b` - Function computing time for algorithm B
/// * `max_n` - Maximum value of n to check
///
/// # Returns
/// Smallest n where A < B, or None if never occurs
///
/// # Example
/// ```
/// use clrs::chapter_01::find_crossover_point;
/// // When does 100n² become faster than 2^n?
/// let poly = |n: f64| 100.0 * n * n;
/// let exp = |n: f64| 2.0_f64.powf(n);
/// let result = find_crossover_point(poly, exp, 100.0);
/// assert_eq!(result, Some(15.0)); // Approximately n = 15
/// ```
pub fn find_crossover_point<F1, F2>(
    time_a: F1,
    time_b: F2,
    max_n: f64,
) -> Option<f64>
where
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
{
    let mut n = 1.0;
    let step = (max_n / 10000.0).max(0.1);
    
    while n <= max_n {
        let time_a_val = time_a(n);
        let time_b_val = time_b(n);
        
        if time_a_val < time_b_val {
            return Some(n);
        }
        
        n += step;
    }
    
    None
}

/// Compares insertion sort vs merge sort as in CLRS Exercise 1.2-2
///
/// Insertion sort: 8n² steps
/// Merge sort: 64n lg n steps
///
/// # Returns
/// Range of n values where insertion sort is faster
pub fn insertion_vs_merge_sort() -> Option<(f64, f64)> {
    let insertion = |n: f64| 8.0 * n * n;
    let merge = |n: f64| {
        if n <= 1.0 {
            0.0
        } else {
            64.0 * n * n.log2()
        }
    };
    compare_algorithms(insertion, merge, 100.0)
}

/// Finds when polynomial beats exponential as in CLRS Exercise 1.2-3
///
/// Finds smallest n where 100n² < 2^n
///
/// # Returns
/// Smallest n value, approximately 15
pub fn polynomial_vs_exponential() -> Option<f64> {
    let polynomial = |n: f64| 100.0 * n * n;
    let exponential = |n: f64| 2.0_f64.powf(n);
    find_crossover_point(polynomial, exponential, 50.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_vs_merge_sort() {
        let result = insertion_vs_merge_sort();
        assert!(result.is_some());
        if let Some((start, end)) = result {
            // According to CLRS, insertion sort beats merge sort for n <= 43
            assert!(start >= 1.0 && start <= 50.0);
            assert!(end >= 40.0 && end <= 50.0);
        }
    }

    #[test]
    fn test_polynomial_vs_exponential() {
        let result = polynomial_vs_exponential();
        assert!(result.is_some());
        if let Some(n) = result {
            // Should be approximately 15
            assert!(n >= 14.0 && n <= 16.0);
        }
    }

    #[test]
    fn test_find_crossover_point() {
        // Simple test: when does n < n²? (never for n > 1)
        let linear = |n: f64| n;
        let quadratic = |n: f64| n * n;
        let result = find_crossover_point(quadratic, linear, 10.0);
        // Quadratic is never faster than linear for n > 1
        assert!(result.is_none() || result.unwrap() < 1.0);
    }

    #[test]
    fn test_compare_algorithms() {
        // Test: when does n² beat n³?
        let n_squared = |n: f64| n * n;
        let n_cubed = |n: f64| n * n * n;
        let result = compare_algorithms(n_squared, n_cubed, 100.0);
        // n² is always faster than n³, so should return full range
        assert!(result.is_some());
    }
}

