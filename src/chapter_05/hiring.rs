//! The Hiring Problem (Section 5.1)
//!
//! The hiring problem models the process of interviewing and hiring candidates.
//! We want to hire the best candidate, which requires interviewing all candidates
//! and keeping track of the best seen so far.

/// Result of the hiring process
#[derive(Debug, Clone, PartialEq)]
pub struct HiringResult {
    /// Number of times we hired a new candidate
    pub hire_count: usize,
    /// Index of the best candidate hired (0-based)
    pub best_candidate_index: Option<usize>,
}

/// Simulates the hiring process
///
/// This corresponds to HIRE-ASSISTANT from CLRS Section 5.1.
/// We interview candidates and hire a new candidate whenever we find
/// one that is better than all previous candidates.
///
/// # Arguments
/// * `candidates` - Array of candidate scores/ranks (higher is better)
///
/// # Returns
/// HiringResult containing the number of hires and best candidate index
///
/// # Example
/// ```
/// use clrs::chapter_05::{hire_assistant, HiringResult};
/// let candidates = vec![1, 5, 2, 8, 3, 9, 4];
/// let result = hire_assistant(&candidates);
/// assert_eq!(result.hire_count, 4); // First candidate + 3 better ones
/// assert_eq!(result.best_candidate_index, Some(5)); // Candidate 9 is best
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
pub fn hire_assistant(candidates: &[i32]) -> HiringResult {
    if candidates.is_empty() {
        return HiringResult {
            hire_count: 0,
            best_candidate_index: None,
        };
    }

    // CLRS: best = 0 (we assume candidate 0 is the least-qualified dummy candidate)
    // Actually, in CLRS, the first candidate is always hired
    let mut best = 0;
    let mut hire_count = 1; // First candidate is always hired

    // CLRS: for i = 1 to n
    for i in 1..candidates.len() {
        // CLRS: interview candidate i
        // CLRS: if candidate i is better than candidate best
        if candidates[i] > candidates[best] {
            // CLRS: hire candidate i
            best = i;
            hire_count += 1;
        }
    }

    HiringResult {
        hire_count,
        best_candidate_index: Some(best),
    }
}

/// Computes the expected number of hires using indicator random variables
///
/// The expected number of times we hire a new office assistant is
/// O(ln n) when candidates are presented in random order.
///
/// # Arguments
/// * `n` - Number of candidates
///
/// # Returns
/// Expected number of hires (E[hires] = ∑(1/i) for i=1 to n)
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
pub fn expected_hires(n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    // E[hires] = ∑(1/i) for i=1 to n = H_n (n-th harmonic number)
    for i in 1..=n {
        sum += 1.0 / (i as f64);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hire_assistant_increasing_order() {
        // Best candidate is at the end
        let candidates = vec![1, 2, 3, 4, 5];
        let result = hire_assistant(&candidates);
        // We hire candidate 0, then 1, then 2, then 3, then 4
        assert_eq!(result.hire_count, 5);
        assert_eq!(result.best_candidate_index, Some(4));
    }

    #[test]
    fn test_hire_assistant_decreasing_order() {
        // Best candidate is at the beginning
        let candidates = vec![5, 4, 3, 2, 1];
        let result = hire_assistant(&candidates);
        // We only hire candidate 0 (the first one)
        assert_eq!(result.hire_count, 1);
        assert_eq!(result.best_candidate_index, Some(0));
    }

    #[test]
    fn test_hire_assistant_random_order() {
        let candidates = vec![1, 5, 2, 8, 3, 9, 4];
        let result = hire_assistant(&candidates);
        // We hire: 1 (score 1), then 5 (score 5), then 8 (score 8), then 9 (score 9)
        assert_eq!(result.hire_count, 4);
        assert_eq!(result.best_candidate_index, Some(5)); // Index of score 9
    }

    #[test]
    fn test_hire_assistant_single_candidate() {
        let candidates = vec![42];
        let result = hire_assistant(&candidates);
        assert_eq!(result.hire_count, 1);
        assert_eq!(result.best_candidate_index, Some(0));
    }

    #[test]
    fn test_hire_assistant_empty() {
        let candidates: Vec<i32> = vec![];
        let result = hire_assistant(&candidates);
        assert_eq!(result.hire_count, 0);
        assert_eq!(result.best_candidate_index, None);
    }

    #[test]
    fn test_expected_hires() {
        // For n=1, expected hires = 1
        assert!((expected_hires(1) - 1.0).abs() < f64::EPSILON);

        // For n=2, expected hires = 1 + 1/2 = 1.5
        assert!((expected_hires(2) - 1.5).abs() < f64::EPSILON);

        // For n=10, expected hires ≈ 2.93
        let e10 = expected_hires(10);
        assert!(e10 > 2.5 && e10 < 3.5);
    }
}
