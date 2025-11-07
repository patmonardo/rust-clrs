//! Random Number Utilities
//!
//! This module provides random number generation utilities including
//! implementations of RANDOM functions and unbiased random generators.

use rand::Rng;

/// Generates a random number between 0 and 1 (inclusive)
///
/// This corresponds to RANDOM(0, 1) from CLRS.
/// Returns 0 with probability 1/2 and 1 with probability 1/2.
///
/// # Returns
/// * `0` with probability 1/2
/// * `1` with probability 1/2
pub fn random_0_1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=1)
}

/// Generates a random number in the range [a, b] using RANDOM(0, 1)
///
/// This corresponds to RANDOM(a, b) from CLRS Exercise 5.1-2.
/// This implementation uses only calls to RANDOM(0, 1).
///
/// # Arguments
/// * `a` - Lower bound (inclusive)
/// * `b` - Upper bound (inclusive)
///
/// # Returns
/// A random number in the range [a, b]
///
/// # Complexity
/// Expected time: O(⌈lg(b - a)⌉)
pub fn random_range(a: i32, b: i32) -> i32 {
    let range = (b - a) as u32;

    if range == 0 {
        return a;
    }

    let bits = (range as f64).log2().ceil() as u32;

    loop {
        let mut result = 0u32;

        // Build a random number in [0, 2^bits) by calling RANDOM(0, 1) bits times
        for i in 0..bits {
            let bit = random_0_1();
            result |= bit << i;
        }

        // If result is within valid range, return it
        if result <= range {
            return a + result as i32;
        }
        // Otherwise, try again (rejection sampling)
    }
}

/// Generates an unbiased random bit from a biased random generator
///
/// This corresponds to UNBIASED-RANDOM from CLRS Exercise 5.1-3.
/// Uses BIASED-RANDOM as a subroutine to produce unbiased output.
///
/// # Arguments
/// * `biased_random` - A mutable reference to a function that returns 0 or 1 with unknown probability p
///
/// # Returns
/// * `0` with probability 1/2
/// * `1` with probability 1/2
///
/// # Strategy
/// Calls biased_random twice. If results differ (01 or 10), returns the first value.
/// If results are the same (00 or 11), tries again.
///
/// # Complexity
/// Expected time: Θ(1 / (2p(1 - p))) where p is the bias probability
pub fn unbiased_random<F>(biased_random: &mut F) -> u32
where
    F: FnMut() -> u32,
{
    loop {
        let x = biased_random();
        let y = biased_random();

        // If x != y, we have either 01 or 10, both with probability p(1-p)
        // This gives us unbiased output
        if x != y {
            return x;
        }
        // If x == y, we have 00 or 11, so we try again
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_random_0_1() {
        // Test that random_0_1 returns only 0 or 1
        for _ in 0..100 {
            let result = random_0_1();
            assert!(result == 0 || result == 1);
        }
    }

    #[test]
    fn test_random_range() {
        // Test that random_range returns values within the specified range
        for _ in 0..100 {
            let result = random_range(5, 10);
            assert!(result >= 5 && result <= 10);
        }

        for _ in 0..100 {
            let result = random_range(-10, -5);
            assert!(result >= -10 && result <= -5);
        }
    }

    #[test]
    fn test_random_range_single_value() {
        let result = random_range(42, 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_unbiased_random() {
        // Create a biased random function that returns 1 with probability 0.75
        use std::cell::Cell;
        let mut biased_rng = rand::thread_rng();
        let call_count = Cell::new(0);

        let mut biased_random = || {
            call_count.set(call_count.get() + 1);
            if biased_rng.gen::<f64>() < 0.75 {
                1
            } else {
                0
            }
        };

        // Test that unbiased_random produces roughly equal 0s and 1s
        let mut counts = HashMap::new();
        let num_samples = 1000;

        for _ in 0..num_samples {
            let result = unbiased_random(&mut biased_random);
            *counts.entry(result).or_insert(0) += 1;
        }

        // Should have roughly equal distribution
        let zeros = counts.get(&0).unwrap_or(&0);
        let ones = counts.get(&1).unwrap_or(&0);

        // With 1000 samples, we expect roughly 500 of each
        // Allow for some variance (say, within 20%)
        assert!(
            *zeros > 350 && *zeros < 650,
            "Zeros: {}, should be ~500",
            zeros
        );
        assert!(*ones > 350 && *ones < 650, "Ones: {}, should be ~500", ones);
    }

    #[test]
    fn test_unbiased_random_heavily_biased() {
        // Test with a very biased function (returns 1 99% of the time)
        let mut biased_rng = rand::thread_rng();

        let mut biased_random = || {
            if biased_rng.gen::<f64>() < 0.99 {
                1
            } else {
                0
            }
        };

        let mut counts = HashMap::new();
        let num_samples = 1000;

        for _ in 0..num_samples {
            let result = unbiased_random(&mut biased_random);
            *counts.entry(result).or_insert(0) += 1;
        }

        // Should still produce roughly unbiased output
        let zeros = counts.get(&0).unwrap_or(&0);
        let ones = counts.get(&1).unwrap_or(&0);

        assert!(*zeros > 350 && *zeros < 650);
        assert!(*ones > 350 && *ones < 650);
    }
}
