//! Asymptotic Notation - The Mathematical Foundation
//!
//! This module provides a trait-based system for expressing and proving
//! asymptotic relationships between functions, demonstrating Rust's power
//! for mathematical abstractions.

use std::fmt;

/// A mathematical function that can be evaluated and compared asymptotically
///
/// Note: This trait is designed to work with concrete types rather than trait objects
/// for maximum performance and type safety. For dynamic dispatch, use FunctionWrapper.
pub trait AsymptoticFunction: Clone + fmt::Display {
    /// Evaluate the function at point n
    fn evaluate(&self, n: f64) -> f64;

    /// Get the name/representation of the function
    fn name(&self) -> String;

    /// Check if function is asymptotically nonnegative
    fn is_asymptotically_nonnegative(&self) -> bool {
        // Default: check if function is nonnegative for large n
        let large_n = 1000.0;
        self.evaluate(large_n) >= 0.0 && self.evaluate(large_n * 10.0) >= 0.0
    }
}

// FunctionWrapper moved to functions.rs to avoid circular dependencies

/// Θ-notation: Tight asymptotic bound
///
/// f(n) = Θ(g(n)) means there exist positive constants c₁, c₂, and n₀
/// such that 0 ≤ c₁·g(n) ≤ f(n) ≤ c₂·g(n) for all n ≥ n₀
#[derive(Debug, Clone)]
pub struct Theta<F, G> {
    pub f: F,
    pub g: G,
    pub c1: f64,
    pub c2: f64,
    pub n0: f64,
}

impl<F, G> Theta<F, G>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    pub fn new(f: F, g: G, c1: f64, c2: f64, n0: f64) -> Result<Self, String> {
        if c1 <= 0.0 || c2 <= 0.0 || c1 > c2 || n0 < 0.0 {
            return Err("Invalid constants for Θ-notation".to_string());
        }
        Ok(Theta { f, g, c1, c2, n0 })
    }

    /// Verify the Θ relationship holds
    pub fn verify(&self, n: f64) -> bool {
        if n < self.n0 {
            return false;
        }

        let f_val = self.f.evaluate(n);
        let g_val = self.g.evaluate(n);

        self.c1 * g_val <= f_val && f_val <= self.c2 * g_val
    }

    /// Automatically find constants that satisfy the relationship
    pub fn find_constants(&self, max_iterations: usize) -> Option<(f64, f64, f64)> {
        let mut n0 = 1.0;

        for _ in 0..max_iterations {
            let mut valid = true;
            let mut min_ratio = f64::INFINITY;
            let mut max_ratio = 0.0;

            // Sample points starting from n0
            for i in 0..100 {
                let n = n0 * (1.1_f64).powi(i);
                let f_val = self.f.evaluate(n);
                let g_val = self.g.evaluate(n);

                if g_val <= 0.0 {
                    valid = false;
                    break;
                }

                let ratio: f64 = f_val / g_val;
                min_ratio = if ratio < min_ratio { ratio } else { min_ratio };
                max_ratio = if ratio > max_ratio { ratio } else { max_ratio };
            }

            if valid && min_ratio > 0.0 {
                let c1 = min_ratio * 0.9; // Slight margin
                let c2 = max_ratio * 1.1;

                // Verify these constants work
                if self.verify_with_constants(n0, c1, c2, 50) {
                    return Some((c1, c2, n0));
                }
            }

            n0 *= 2.0;
        }

        None
    }

    fn verify_with_constants(&self, n0: f64, c1: f64, c2: f64, samples: usize) -> bool {
        for i in 0..samples {
            let n = n0 * (2.0_f64).powi(i as i32);
            let f_val = self.f.evaluate(n);
            let g_val = self.g.evaluate(n);

            if g_val <= 0.0 || c1 * g_val > f_val || f_val > c2 * g_val {
                return false;
            }
        }
        true
    }
}

impl<F, G> fmt::Display for Theta<F, G>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = Θ({})", self.f.name(), self.g.name())
    }
}

/// O-notation: Asymptotic upper bound
///
/// f(n) = O(g(n)) means there exist positive constants c and n₀
/// such that 0 ≤ f(n) ≤ c·g(n) for all n ≥ n₀
#[derive(Debug, Clone)]
pub struct BigO<F, G> {
    pub f: F,
    pub g: G,
    pub c: f64,
    pub n0: f64,
}

impl<F, G> BigO<F, G>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    pub fn new(f: F, g: G, c: f64, n0: f64) -> Result<Self, String> {
        if c <= 0.0 || n0 < 0.0 {
            return Err("Invalid constants for O-notation".to_string());
        }
        Ok(BigO { f, g, c, n0 })
    }

    pub fn verify(&self, n: f64) -> bool {
        if n < self.n0 {
            return false;
        }

        let f_val = self.f.evaluate(n);
        let g_val = self.g.evaluate(n);

        f_val >= 0.0 && f_val <= self.c * g_val
    }
}

impl<F, G> fmt::Display for BigO<F, G>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = O({})", self.f.name(), self.g.name())
    }
}

/// Ω-notation: Asymptotic lower bound
///
/// f(n) = Ω(g(n)) means there exist positive constants c and n₀
/// such that 0 ≤ c·g(n) ≤ f(n) for all n ≥ n₀
#[derive(Debug, Clone)]
pub struct Omega<F, G> {
    pub f: F,
    pub g: G,
    pub c: f64,
    pub n0: f64,
}

impl<F, G> Omega<F, G>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    pub fn new(f: F, g: G, c: f64, n0: f64) -> Result<Self, String> {
        if c <= 0.0 || n0 < 0.0 {
            return Err("Invalid constants for Ω-notation".to_string());
        }
        Ok(Omega { f, g, c, n0 })
    }

    pub fn verify(&self, n: f64) -> bool {
        if n < self.n0 {
            return false;
        }

        let f_val = self.f.evaluate(n);
        let g_val = self.g.evaluate(n);

        self.c * g_val <= f_val && f_val >= 0.0
    }
}

impl<F, G> fmt::Display for Omega<F, G>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = Ω({})", self.f.name(), self.g.name())
    }
}

/// Theorem 3.1: f(n) = Θ(g(n)) if and only if f(n) = O(g(n)) and f(n) = Ω(g(n))
pub fn prove_theta_from_o_and_omega<F, G>(
    f: &F,
    g: &G,
    big_o: &BigO<F, G>,
    omega: &Omega<F, G>,
) -> Option<Theta<F, G>>
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    // If we have both O and Ω, we can construct Θ
    let c1 = omega.c;
    let c2 = big_o.c;
    let n0 = big_o.n0.max(omega.n0);

    if c1 <= c2 {
        Theta::new(f.clone(), g.clone(), c1, c2, n0).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapter_03::functions::*;

    #[test]
    fn test_theta_verification() {
        use crate::chapter_03::functions::*;
        let n_squared = Polynomial::new(2.0);

        // Create a simple n² + n approximation by using a polynomial with slightly adjusted evaluation
        // For testing, we'll directly compare n² with itself (which should be Θ)
        let theta = Theta::new(
            n_squared.clone(),
            n_squared.clone(),
            0.5, // c1
            2.0, // c2
            1.0, // n0
        )
        .unwrap();

        // Should hold for large n
        assert!(theta.verify(10.0));
        assert!(theta.verify(100.0));
        assert!(theta.verify(1000.0));
    }

    #[test]
    fn test_big_o_verification() {
        let n_squared = Polynomial::new(2.0);
        let n_cubed = Polynomial::new(3.0);

        // n² = O(n³)
        let big_o = BigO::new(n_squared.clone(), n_cubed.clone(), 1.0, 1.0).unwrap();

        assert!(big_o.verify(10.0));
        assert!(big_o.verify(100.0));
    }

    #[test]
    fn test_theorem_3_1() {
        use crate::chapter_03::functions::*;
        let n_cubed = Polynomial::new(3.0);

        // Test: n³ = O(n³) and n³ = Ω(n³)
        let big_o = BigO::new(n_cubed.clone(), n_cubed.clone(), 1.0, 1.0).unwrap();

        let omega = Omega::new(n_cubed.clone(), n_cubed.clone(), 0.5, 1.0).unwrap();

        // Should be able to construct Θ from O and Ω
        let theta = prove_theta_from_o_and_omega(&n_cubed, &n_cubed, &big_o, &omega);

        assert!(theta.is_some());
        if let Some(t) = theta {
            assert!(t.verify(100.0));
        }
    }
}
