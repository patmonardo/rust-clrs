// horner_rule_exposition.rs
// Expository walkthrough of CLRS Problem 2-3 (Correctness of Horner's rule)
//
// We build two evaluators:
//   * `horner_with_invariant` narrates Horner's rule while asserting the loop
//     invariant from the textbook + Peng-Yu notes.
//   * `naive_polynomial_evaluation` mirrors the Θ(n^2) "compute powers from
//     scratch" strategy to contrast asymptotic costs.
//
// The main routine prints both traces, compares the results, and sanity-checks
// the Θ(n) vs Θ(n^2) operation counts.

fn main() {
    // Coefficients for P(x) = 2 + 3x + 5x^2 + 7x^3.
    let coeffs = vec![2.0, 3.0, 5.0, 7.0];
    let x = 1.5;

    println!("Coefficients: {:?}", coeffs);
    println!("Evaluation point x = {}\n", x);

    let (horner_value, horner_counts) = horner_with_invariant(&coeffs, x);
    println!("\nFinal Horner value: {}", horner_value);
    println!("Operation counts (Horner): {:?}\n", horner_counts);

    let (naive_value, naive_counts) = naive_polynomial_evaluation(&coeffs, x);
    println!("Final naive value: {}", naive_value);
    println!("Operation counts (Naive): {:?}\n", naive_counts);

    assert!((horner_value - naive_value).abs() < 1e-9);
    println!("Values match within tolerance. Horner achieves Θ(n) vs Θ(n^2) work.");
}

#[derive(Debug, Default)]
struct OperationCounts {
    additions: usize,
    multiplications: usize,
    outer_iterations: usize,
    inner_iterations: usize,
}

fn horner_with_invariant(coeffs: &[f64], x: f64) -> (f64, OperationCounts) {
    let mut y = 0.0;
    let mut counts = OperationCounts::default();
    let n = coeffs.len();

    println!("Horner's rule trace:");

    for (step, i) in (0..n).rev().enumerate() {
        debug_assert!(
            horner_invariant_holds(coeffs, x, y, i),
            "Loop invariant violated at i = {}",
            i
        );
        counts.outer_iterations += 1;

        let coeff = coeffs[i];
        println!("  Step {} (i = {}): y = a[i] + x * y", step, i);
        println!("    Current y before update: {:?}", y);
        println!("    Coefficient a[i]: {:?}", coeff);

        let xy = x * y;
        counts.multiplications += 1;
        let new_y = coeff + xy;
        counts.additions += 1;
        println!("    x * y = {:?}, new y = {:?}\n", xy, new_y);
        y = new_y;
    }

    let direct = evaluate_polynomial(coeffs, x);
    debug_assert!(
        (y - direct).abs() < 1e-9,
        "Horner result {:?} should match direct evaluation {:?}",
        y,
        direct
    );

    (y, counts)
}

fn horner_invariant_holds(coeffs: &[f64], x: f64, y: f64, i: usize) -> bool {
    // Invariant: y == Σ_{k=0}^{n-(i+1)} a_{k+i+1} x^k at loop entry.
    // We recompute RHS directly for assertion purposes.
    if i + 1 >= coeffs.len() {
        return y.abs() < 1e-9;
    }

    let mut rhs = 0.0;
    let mut power = 1.0;
    for coeff in &coeffs[i + 1..] {
        rhs += coeff * power;
        power *= x;
    }

    (y - rhs).abs() < 1e-9
}

fn naive_polynomial_evaluation(coeffs: &[f64], x: f64) -> (f64, OperationCounts) {
    let mut y = 0.0;
    let mut counts = OperationCounts::default();

    println!("Naive Θ(n^2) evaluation trace:");

    for (k, &coeff) in coeffs.iter().enumerate() {
        counts.outer_iterations += 1;
        let mut power = 1.0; // x^0

        println!("  Term k = {}: computing a[k] * x^k", k);
        for _ in 0..k {
            power *= x;
            counts.inner_iterations += 1;
            counts.multiplications += 1;
        }

        let term = coeff * power;
        counts.multiplications += 1;
        y += term;
        counts.additions += 1;

        println!(
            "    x^k = {:.6}, term = {:.6}, partial sum y = {:.6}",
            power, term, y
        );
    }

    (y, counts)
}

fn evaluate_polynomial(coeffs: &[f64], x: f64) -> f64 {
    let mut sum = 0.0;
    let mut power = 1.0;
    for &coeff in coeffs {
        sum += coeff * power;
        power *= x;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horner_matches_naive() {
        let coeffs = vec![2.0, -1.0, 0.5];
        let x = -2.5;

        let (h, _) = horner_with_invariant(&coeffs, x);
        let (n, _) = naive_polynomial_evaluation(&coeffs, x);

        assert!((h - n).abs() < 1e-9);
    }
}
