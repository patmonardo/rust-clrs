//! Efficiency Measures (Section 1.1)
//!
//! This module provides tools for measuring different aspects of algorithm efficiency:
//! - Speed (time complexity)
//! - Memory efficiency
//! - Code complexity/maintainability

use std::time::Instant;

/// Performance measurement result
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Execution time in nanoseconds
    pub time_ns: u64,
    /// Memory usage estimation (in bytes, if available)
    pub memory_bytes: Option<usize>,
    /// Number of operations performed (if tracked)
    pub operations: Option<usize>,
}

/// Measures the execution time of a function
///
/// # Arguments
/// * `f` - Function to measure
///
/// # Returns
/// PerformanceMetrics with execution time
///
/// # Example
/// ```
/// use clrs::chapter_01::measure_time;
/// let result = measure_time(|| {
///     let mut sum = 0;
///     for i in 0..1000 {
///         sum += i;
///     }
///     sum
/// });
/// assert!(result.time_ns > 0);
/// ```
pub fn measure_time<F, R>(f: F) -> (PerformanceMetrics, R)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    (
        PerformanceMetrics {
            time_ns: elapsed.as_nanos() as u64,
            memory_bytes: None,
            operations: None,
        },
        result,
    )
}

/// Measures both time and tracks operation count
///
/// # Arguments
/// * `f` - Function that returns a tuple of (result, operation_count)
///
/// # Returns
/// PerformanceMetrics with time and operation count
pub fn measure_time_with_ops<F, R>(f: F) -> (PerformanceMetrics, R)
where
    F: FnOnce() -> (R, usize),
{
    let start = Instant::now();
    let (result, ops) = f();
    let elapsed = start.elapsed();

    (
        PerformanceMetrics {
            time_ns: elapsed.as_nanos() as u64,
            memory_bytes: None,
            operations: Some(ops),
        },
        result,
    )
}

/// Compares efficiency across multiple dimensions
#[derive(Debug, Clone)]
pub struct EfficiencyComparison {
    /// Time comparison (smaller is better)
    pub time_ratio: f64,
    /// Memory comparison (smaller is better)
    pub memory_ratio: Option<f64>,
    /// Operations comparison (smaller is better)
    pub operations_ratio: Option<f64>,
}

/// Compares two performance metrics
pub fn compare_performance(a: &PerformanceMetrics, b: &PerformanceMetrics) -> EfficiencyComparison {
    EfficiencyComparison {
        time_ratio: a.time_ns as f64 / b.time_ns as f64,
        memory_ratio: match (a.memory_bytes, b.memory_bytes) {
            (Some(a_mem), Some(b_mem)) => Some(a_mem as f64 / b_mem as f64),
            _ => None,
        },
        operations_ratio: match (a.operations, b.operations) {
            (Some(a_ops), Some(b_ops)) => Some(a_ops as f64 / b_ops as f64),
            _ => None,
        },
    }
}

/// Determines if algorithm A is more efficient than B
pub fn is_more_efficient(comparison: &EfficiencyComparison, threshold: f64) -> bool {
    comparison.time_ratio < threshold
        && comparison.memory_ratio.is_none_or(|r| r < threshold)
        && comparison.operations_ratio.is_none_or(|r| r < threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_time() {
        let (metrics, _) = measure_time(|| {
            let mut sum = 0;
            for i in 0..100 {
                sum += i;
            }
            sum
        });
        assert!(metrics.time_ns > 0);
    }

    #[test]
    fn test_compare_performance() {
        let a = PerformanceMetrics {
            time_ns: 100,
            memory_bytes: Some(1000),
            operations: Some(50),
        };

        let b = PerformanceMetrics {
            time_ns: 200,
            memory_bytes: Some(2000),
            operations: Some(100),
        };

        let comparison = compare_performance(&a, &b);
        assert!(comparison.time_ratio < 1.0); // A is faster
        assert!(comparison.memory_ratio.unwrap() < 1.0); // A uses less memory
        assert!(is_more_efficient(&comparison, 1.0));
    }
}
