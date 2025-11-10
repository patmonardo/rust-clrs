//! Exercise 2.3-3 – solving the merge-sort recurrence by induction.
//!
//! The merge-sort pseudocode in Section 2.3 yields the recurrence
//! `T(2) = 2` and `T(n) = 2 T(n/2) + n` for powers of two.  CLRS asks us
//! to prove that the closed form is `T(n) = n log₂ n`.  This example
//! prints the recurrence table, outlines the induction step algebraically,
//! and double-checks the identity for several powers of two.

fn main() {
    println!("Exercise 2.3-3: merge sort recurrence T(n) = 2T(n/2) + n\n");
    println!("For powers of two we expect T(n) = n · log₂ n.\n");

    let powers: Vec<usize> = (1..=8).map(|k| 1 << k).collect();
    println!(
        "{:<6} {:>12} {:>12} {:>12}",
        "n", "T(n)", "n·log₂ n", "difference"
    );
    for &n in &powers {
        let recurrence = t_of_n(n);
        let n_log_n = n_log2_n(n);
        println!(
            "{:<6} {:>12} {:>12} {:>12}",
            n,
            recurrence,
            n_log_n,
            (recurrence as isize - n_log_n as isize)
        );
    }

    println!("\nInduction outline for arbitrary power-of-two n:");
    print_induction_outline();
}

fn t_of_n(n: usize) -> usize {
    assert!(n.is_power_of_two(), "n must be a power of two");
    if n == 2 {
        2
    } else {
        2 * t_of_n(n / 2) + n
    }
}

fn n_log2_n(n: usize) -> usize {
    assert!(n.is_power_of_two(), "n must be a power of two");
    let log_n = n.ilog2() as usize;
    n * log_n
}

fn print_induction_outline() {
    println!("  Base case:");
    println!("    T(2) = 2 (given) and 2 · log₂ 2 = 2 · 1 = 2.");
    println!("  Inductive step:");
    println!("    Assume for n/2 (with n a power of two > 2) that");
    println!("      T(n/2) = (n/2) · log₂(n/2).");
    println!("    Then using the recurrence:");
    println!("      T(n) = 2 · T(n/2) + n");
    println!("           = 2 · (n/2 · log₂(n/2)) + n");
    println!("           = n · log₂(n/2) + n");
    println!("           = n · (log₂ n − 1) + n");
    println!("           = n · log₂ n.");
    println!("    Thus the proposition holds for n, completing the induction.");
    println!("\nThis matches the Θ(n log n) running time claimed for merge sort.");
}
