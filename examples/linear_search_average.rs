//! Exercise 2.2-3 – Average and worst-case analysis for linear search.
//!
//! We revisit `chapter_02::linear_search` and quantify how many positions are
//! inspected on average under two probability models:
//!   1. **Independent hit probability (`p`) per position.**
//!      Each element matches the sought value with probability `p`, independently.
//!      The search may fail entirely.  CLRS hints at this interpretation when it
//!      speaks about an element being “equally likely to be any element” but does
//!      not forbid absence.
//!   2. **Exactly one match, uniformly random position.**
//!      This is the alternative interpretation presented at the end of the official
//!      solution: the target is guaranteed to exist and each index is equally likely.
//!
//! For each model we print the theoretical expectation, the worst-case bound, and
//! a Monte Carlo estimate to confirm our calculations.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    println!("Exercise 2.2-3: linear search average and worst-case analysis\n");

    println!("Model A: independent success probability per position");
    println!("-----------------------------------------------------");
    for &p in &[0.1, 0.25, 0.5] {
        analyze_probabilistic_hits(32, p, 200_000);
    }

    println!("\nModel B: exactly one match, uniformly random position");
    println!("-----------------------------------------------------");
    for &n in &[8, 16, 32, 64] {
        analyze_uniform_match(n, 200_000);
    }
}

/// Independent success probability per position (may fail entirely).
fn analyze_probabilistic_hits(n: usize, p: f64, trials: usize) {
    let expected_theory = expected_steps_independent_hits(n, p);
    let worst_case = n as f64;
    let simulated = simulate_independent_hits(n, p, trials);

    println!(
        "n = {:>3}, p = {:>4.2}: expected ≈ {:>6.3}, simulated ≈ {:>6.3}, worst-case = {:>5.1}",
        n, p, expected_theory, simulated, worst_case
    );
}

/// Exact theoretical expectation for independent success-probability model.
fn expected_steps_independent_hits(n: usize, p: f64) -> f64 {
    let mut sum = n as f64 * (1.0 - p).powi(n as i32); // failure case
    for k in 1..=n {
        sum += k as f64 * (1.0 - p).powi((k - 1) as i32) * p;
    }
    sum
}

fn simulate_independent_hits(n: usize, p: f64, trials: usize) -> f64 {
    let mut rng = StdRng::seed_from_u64(0x5EED);
    let mut total = 0_f64;
    for _ in 0..trials {
        let mut steps = 0;
        let mut found = false;
        for _ in 0..n {
            steps += 1;
            if rng.gen::<f64>() < p {
                found = true;
                break;
            }
        }
        if !found {
            // failure: we already counted n steps
        }
        total += steps as f64;
    }
    total / trials as f64
}

/// Uniformly random match location (always present).
fn analyze_uniform_match(n: usize, trials: usize) {
    let expected_theory = (n as f64 + 1.0) / 2.0;
    let worst_case = n as f64;
    let simulated = simulate_uniform_match(n, trials);

    println!(
        "n = {:>3}: expected ≈ {:>6.3}, simulated ≈ {:>6.3}, worst-case = {:>5.1}",
        n, expected_theory, simulated, worst_case
    );
}

fn simulate_uniform_match(n: usize, trials: usize) -> f64 {
    let mut rng = StdRng::seed_from_u64(0xD1CE);
    let mut total = 0_f64;
    for _ in 0..trials {
        let target_index = rng.gen_range(0..n);
        total += (target_index + 1) as f64;
    }
    total / trials as f64
}
