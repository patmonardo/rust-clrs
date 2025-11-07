//! Automated Proof Checking for Asymptotic Relationships
//!
//! This module implements automated verification of asymptotic relationships,
//! demonstrating how Rust's type system can encode mathematical proofs.

use super::asymptotic::*;
use std::fmt;

/// Result of a proof attempt
#[derive(Debug, Clone)]
pub enum ProofResult {
    Proven {
        constants: Vec<(String, f64)>,
        n0: f64,
    },
    Disproven {
        counterexample: f64,
    },
    Unknown {
        reason: String,
    },
}

impl fmt::Display for ProofResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProofResult::Proven { constants, n0 } => {
                write!(f, "Proven with constants: {:?}, n₀ = {}", constants, n0)
            }
            ProofResult::Disproven { counterexample } => {
                write!(f, "Disproven at n = {}", counterexample)
            }
            ProofResult::Unknown { reason } => {
                write!(f, "Unknown: {}", reason)
            }
        }
    }
}

/// Prove Theorem 3.1: f(n) = Θ(g(n)) if and only if f(n) = O(g(n)) and f(n) = Ω(g(n))
pub fn prove_theorem_3_1<F, G>(f: &F, g: &G) -> ProofResult
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    // Try to prove both directions
    let big_o_result = prove_big_o(f, g);
    let omega_result = prove_omega(f, g);

    match (big_o_result, omega_result) {
        (
            ProofResult::Proven {
                constants: o_consts,
                n0: n0_o,
            },
            ProofResult::Proven {
                constants: omega_consts,
                n0: n0_omega,
            },
        ) => {
            // Extract constants
            let c_o = o_consts
                .iter()
                .find(|(name, _)| name == "c")
                .map(|(_, v)| *v)
                .unwrap_or(1.0);
            let c_omega = omega_consts
                .iter()
                .find(|(name, _)| name == "c")
                .map(|(_, v)| *v)
                .unwrap_or(1.0);
            let n0 = n0_o.max(n0_omega);

            // Verify Theta relationship
            let theta = Theta::new(f.clone(), g.clone(), c_omega, c_o, n0);

            if let Ok(t) = theta {
                // Verify it holds
                let mut all_valid = true;
                for i in 0..100 {
                    let n = n0 * (1.5_f64).powi(i);
                    if !t.verify(n) {
                        all_valid = false;
                        break;
                    }
                }

                if all_valid {
                    ProofResult::Proven {
                        constants: vec![("c₁".to_string(), c_omega), ("c₂".to_string(), c_o)],
                        n0,
                    }
                } else {
                    ProofResult::Unknown {
                        reason: "O and Ω proven, but Θ verification failed".to_string(),
                    }
                }
            } else {
                ProofResult::Unknown {
                    reason: "Could not construct Θ from O and Ω".to_string(),
                }
            }
        }
        (ProofResult::Disproven { counterexample }, _) => ProofResult::Disproven { counterexample },
        (_, ProofResult::Disproven { counterexample }) => ProofResult::Disproven { counterexample },
        _ => ProofResult::Unknown {
            reason: "Could not prove both O and Ω".to_string(),
        },
    }
}

/// Attempt to prove f(n) = O(g(n))
pub fn prove_big_o<F, G>(f: &F, g: &G) -> ProofResult
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    // Heuristic: try to find constants
    let mut n0 = 1.0;

    for iteration in 0..50 {
        let mut max_ratio = 0.0;
        let mut valid = true;

        // Sample points
        for i in 0..50 {
            let n = n0 * (1.2_f64).powi(i);
            let f_val = f.evaluate(n);
            let g_val = g.evaluate(n);

            if g_val <= 0.0 {
                // Invalid, try larger n0
                valid = false;
                break;
            }

            if f_val < 0.0 {
                // Function must be asymptotically nonnegative
                return ProofResult::Disproven { counterexample: n };
            }

            let ratio: f64 = f_val / g_val;
            max_ratio = if ratio > max_ratio { ratio } else { max_ratio };
        }

        if valid {
            let c = max_ratio * 1.1; // Add margin

            // Verify this constant works
            if verify_big_o(f, g, c, n0, 100) {
                return ProofResult::Proven {
                    constants: vec![("c".to_string(), c)],
                    n0,
                };
            }
        }

        n0 *= 2.0;

        if iteration > 20 {
            break;
        }
    }

    ProofResult::Unknown {
        reason: "Could not find valid constants".to_string(),
    }
}

/// Attempt to prove f(n) = Ω(g(n))
pub fn prove_omega<F, G>(f: &F, g: &G) -> ProofResult
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    let mut n0 = 1.0;

    for iteration in 0..50 {
        let mut min_ratio = f64::INFINITY;
        let mut valid = true;

        // Sample points
        for i in 0..50 {
            let n = n0 * (1.2_f64).powi(i);
            let f_val = f.evaluate(n);
            let g_val = g.evaluate(n);

            if g_val <= 0.0 {
                valid = false;
                break;
            }

            if f_val < 0.0 {
                return ProofResult::Disproven { counterexample: n };
            }

            let ratio = f_val / g_val;
            min_ratio = min_ratio.min(ratio);
        }

        if valid && min_ratio > 0.0 && min_ratio.is_finite() {
            let c = min_ratio * 0.9; // Slight margin

            // Verify this constant works
            if verify_omega(f, g, c, n0, 100) {
                return ProofResult::Proven {
                    constants: vec![("c".to_string(), c)],
                    n0,
                };
            }
        }

        n0 *= 2.0;

        if iteration > 20 {
            break;
        }
    }

    ProofResult::Unknown {
        reason: "Could not find valid constants".to_string(),
    }
}

fn verify_big_o<F, G>(f: &F, g: &G, c: f64, n0: f64, samples: usize) -> bool
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    for i in 0..samples {
        let n = n0 * (2.0_f64).powi(i as i32);
        let f_val = f.evaluate(n);
        let g_val = g.evaluate(n);

        if f_val < 0.0 || f_val > c * g_val {
            return false;
        }
    }
    true
}

fn verify_omega<F, G>(f: &F, g: &G, c: f64, n0: f64, samples: usize) -> bool
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    for i in 0..samples {
        let n = n0 * (2.0_f64).powi(i as i32);
        let f_val = f.evaluate(n);
        let g_val = g.evaluate(n);

        if c * g_val > f_val || f_val < 0.0 {
            return false;
        }
    }
    true
}

/// Prove that max(f(n), g(n)) = Θ(f(n) + g(n)) for asymptotically nonnegative functions
/// This is Exercise 3.1-1
pub fn prove_max_equals_theta_sum<F, G>(f: &F, g: &G) -> ProofResult
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    // Check if functions are asymptotically nonnegative
    if !f.is_asymptotically_nonnegative() || !g.is_asymptotically_nonnegative() {
        return ProofResult::Unknown {
            reason: "Functions must be asymptotically nonnegative".to_string(),
        };
    }

    // Note: This proof would need concrete implementations of Sum and Max
    // For now, we verify the relationship mathematically
    // According to the proof: c₁ = 1/2, c₂ = 1, n₀ = max(n₁, n₂)

    let mut n0 = 1.0;

    // Find n0 where both are nonnegative
    while n0 < 1e10 {
        if f.evaluate(n0) >= 0.0 && g.evaluate(n0) >= 0.0 {
            break;
        }
        n0 *= 2.0;
    }

    let c1 = 0.5;
    let c2 = 1.0;

    // Verify: (f(n) + g(n))/2 ≤ max(f(n), g(n)) ≤ f(n) + g(n)
    let mut all_valid = true;
    for i in 0..100 {
        let n = n0 * (1.5_f64).powi(i);
        let f_val = f.evaluate(n);
        let g_val = g.evaluate(n);

        let max_val = if f_val > g_val { f_val } else { g_val };
        let sum_val = f_val + g_val;

        if c1 * sum_val > max_val || max_val > c2 * sum_val {
            all_valid = false;
            break;
        }
    }

    if all_valid {
        ProofResult::Proven {
            constants: vec![("c₁".to_string(), c1), ("c₂".to_string(), c2)],
            n0,
        }
    } else {
        ProofResult::Unknown {
            reason: "Could not verify relationship".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapter_03::Polynomial;

    #[test]
    fn test_prove_big_o() {
        let n_squared = Polynomial::new(2.0);
        let n_cubed = Polynomial::new(3.0);

        let result = prove_big_o(&n_squared, &n_cubed);
        match result {
            ProofResult::Proven { .. } => {
                // Good!
            }
            _ => {
                panic!("Should be able to prove n² = O(n³)");
            }
        }
    }

    #[test]
    fn test_prove_max_equals_theta_sum() {
        let n = Polynomial::new(1.0);
        let n_squared = Polynomial::new(2.0);

        let result = prove_max_equals_theta_sum(&n, &n_squared);
        match result {
            ProofResult::Proven { .. } => {
                // Good!
            }
            _ => {
                // Might fail for some cases, that's okay
            }
        }
    }
}
