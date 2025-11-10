//! Exercise 4.1-3 exploration for the maximum-subarray problem.
//!
//! This example program compares the brute-force and recursive (divide-and-conquer)
//! implementations from `chapter_04::maximum_subarray` and looks for the empirical
//! crossover point `n0` where the recursive version overtakes brute force.
//! Afterwards, it evaluates a hybrid strategy that switches to brute force as the
//! base case whenever the subproblem size falls below `n0`, as suggested by the
//! exercise prompt.

use clrs::chapter_04::{
    brute_force_find_maximum_subarray, find_maximum_subarray, iterative_find_maximum_subarray,
    MaximumSubarrayResult,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::hint::black_box;
use std::time::{Duration, Instant};

fn main() {
    let sizes: Vec<usize> = (4..=128).step_by(4).collect();
    let samples_per_size = 32;
    let repeats_per_sample = 32;

    let suite = build_sample_suite(&sizes, samples_per_size);

    println!("Exercise 4.1-3: Maximum Subarray crossover exploration");
    println!("------------------------------------------------------\n");
    println!(
        "Measuring brute-force vs recursive (divide and conquer) with {} samples per size and \
         {} inner repetitions.\n",
        samples_per_size, repeats_per_sample
    );

    let brute_timings = measure_suite(
        &suite,
        repeats_per_sample,
        brute_force_find_maximum_subarray,
    );
    let recursive_timings = measure_suite(&suite, repeats_per_sample, |arr| {
        find_maximum_subarray(arr, 0, arr.len() - 1)
    });
    let iterative_timings =
        measure_suite(&suite, repeats_per_sample, iterative_find_maximum_subarray);

    print_triple_table(
        "Original algorithms",
        &sizes,
        &brute_timings,
        &recursive_timings,
        &iterative_timings,
    );
    let crossover = find_crossover(&sizes, &brute_timings, &recursive_timings);

    if let Some(n0) = crossover {
        println!("\nEmpirical crossover n0 ≈ {n0}");

        println!(
            "\nRe-running with a hybrid algorithm that switches to brute force when \
             subproblem size < {n0}."
        );

        let hybrid_timings = measure_suite(&suite, repeats_per_sample, |arr| {
            hybrid_find_maximum_subarray(arr, 0, arr.len() - 1, n0)
        });

        print_table(
            "Brute force vs hybrid (threshold = n0)",
            &sizes,
            &brute_timings,
            &hybrid_timings,
        );

        if let Some(hybrid_crossover) = find_crossover(&sizes, &brute_timings, &hybrid_timings) {
            println!(
                "\nHybrid crossover n0' ≈ {hybrid_crossover} (compare with original n0 ≈ {n0})"
            );
        } else {
            println!(
                "\nHybrid algorithm dominates brute-force for all tested sizes with threshold {n0}."
            );
        }
    } else {
        println!(
            "\nNo crossover detected within tested range; recursive version never beat brute-force."
        );
    }
}

/// Build random test arrays for each size, reusing the data across all timings.
fn build_sample_suite(sizes: &[usize], samples_per_size: usize) -> Vec<(usize, Vec<Vec<i64>>)> {
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    sizes
        .iter()
        .map(|&size| {
            let arrays = (0..samples_per_size)
                .map(|_| random_array(size, &mut rng))
                .collect();
            (size, arrays)
        })
        .collect()
}

/// Generate a random array with values in a symmetric range so that both positive
/// and negative runs appear.
fn random_array(size: usize, rng: &mut StdRng) -> Vec<i64> {
    const MAX_ABS: i64 = 50;
    (0..size)
        .map(|_| rng.gen_range(-MAX_ABS..=MAX_ABS))
        .collect()
}

/// Measure elapsed time for a suite of arrays using the provided algorithm.
fn measure_suite<F>(suite: &[(usize, Vec<Vec<i64>>)], repeats: usize, mut f: F) -> Vec<Duration>
where
    F: FnMut(&[i64]) -> MaximumSubarrayResult,
{
    suite
        .iter()
        .map(|(_, arrays)| measure_arrays(arrays, repeats, &mut f))
        .collect()
}

fn measure_arrays<F>(arrays: &[Vec<i64>], repeats: usize, f: &mut F) -> Duration
where
    F: FnMut(&[i64]) -> MaximumSubarrayResult,
{
    let mut total = Duration::ZERO;
    for arr in arrays {
        let start = Instant::now();
        for _ in 0..repeats {
            let result = f(arr);
            black_box(result);
        }
        total += start.elapsed();
    }
    total
}

/// Find the first size where algorithm B beats algorithm A.
fn find_crossover(sizes: &[usize], baseline: &[Duration], contender: &[Duration]) -> Option<usize> {
    sizes
        .iter()
        .zip(baseline.iter().zip(contender))
        .find_map(|(&size, (&base, &other))| (other < base).then_some(size))
}

/// Pretty-print a comparison table for two timing series.
fn print_table(title: &str, sizes: &[usize], baseline: &[Duration], contender: &[Duration]) {
    println!("\n{title}");
    println!(
        "{:<6} {:>12} {:>12} {:>9}",
        "n", "baseline (µs)", "contender (µs)", "ratio"
    );
    for ((size, base), other) in sizes.iter().zip(baseline).zip(contender) {
        let base_us = base.as_secs_f64() * 1_000_000.0;
        let other_us = other.as_secs_f64() * 1_000_000.0;
        let ratio = if other_us > 0.0 {
            base_us / other_us
        } else {
            f64::INFINITY
        };
        println!(
            "{:<6} {:>12.1} {:>12.1} {:>9.2}",
            size, base_us, other_us, ratio
        );
    }
}

fn print_triple_table(
    title: &str,
    sizes: &[usize],
    brute: &[Duration],
    recursive: &[Duration],
    iterative: &[Duration],
) {
    println!("\n{title}");
    println!(
        "{:<6} {:>12} {:>12} {:>12}",
        "n", "brute (µs)", "recursive (µs)", "iterative (µs)"
    );
    for (((size, brute_duration), recursive_duration), iterative_duration) in
        sizes.iter().zip(brute).zip(recursive).zip(iterative)
    {
        let brute_us = brute_duration.as_secs_f64() * 1_000_000.0;
        let recursive_us = recursive_duration.as_secs_f64() * 1_000_000.0;
        let iterative_us = iterative_duration.as_secs_f64() * 1_000_000.0;
        println!(
            "{:<6} {:>12.1} {:>12.1} {:>12.1}",
            size, brute_us, recursive_us, iterative_us
        );
    }
}

/// Hybrid divide-and-conquer that switches to brute force below a threshold.
fn hybrid_find_maximum_subarray(
    arr: &[i64],
    low: usize,
    high: usize,
    threshold: usize,
) -> MaximumSubarrayResult {
    assert!(threshold > 0, "threshold must be positive");
    assert!(low <= high, "low must not exceed high");
    let length = high - low + 1;
    if length <= threshold {
        return brute_force_on_slice(arr, low, high);
    }

    if low == high {
        return MaximumSubarrayResult {
            low,
            high,
            sum: arr[low],
        };
    }

    let mid = (low + high) / 2;
    let left = hybrid_find_maximum_subarray(arr, low, mid, threshold);
    let right = hybrid_find_maximum_subarray(arr, mid + 1, high, threshold);
    let cross = find_max_crossing_subarray(arr, low, mid, high);

    if left.sum >= right.sum && left.sum >= cross.sum {
        left
    } else if right.sum >= left.sum && right.sum >= cross.sum {
        right
    } else {
        cross
    }
}

fn brute_force_on_slice(arr: &[i64], low: usize, high: usize) -> MaximumSubarrayResult {
    let mut result = brute_force_find_maximum_subarray(&arr[low..=high]);
    result.low += low;
    result.high += low;
    result
}

fn find_max_crossing_subarray(
    arr: &[i64],
    low: usize,
    mid: usize,
    high: usize,
) -> MaximumSubarrayResult {
    let mut left_sum = i64::MIN;
    let mut sum = 0;
    let mut max_left = mid;

    for i in (low..=mid).rev() {
        sum += arr[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = i64::MIN;
    sum = 0;
    let mut max_right = mid;

    for j in (mid + 1)..=high {
        sum += arr[j];
        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }
    }

    MaximumSubarrayResult {
        low: max_left,
        high: max_right,
        sum: left_sum + right_sum,
    }
}
