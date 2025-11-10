//! Exercise 2.3-7 – sort-and-scan two-sum algorithm.
//!
//! Given a set `S` of `n` integers and a target `x`, CLRS asks for an
//! `Θ(n log n)`-time algorithm to determine whether two distinct elements
//! sum to `x`.  The textbook sketch is:
//!  1. Sort `S`.
//!  2. Place pointers `i` (left) and `j` (right) at the ends.
//!  3. Move the pointers inward based on whether the current sum is too
//!     small or too large until a match is found or the pointers cross.
//!
//! This example makes that proof executable.  We:
//!  * show the trace on small arrays,
//!  * assert the loop invariant (“all eliminated pairs are provably
//!    impossible”), and
//!  * measure that the pointer loop performs at most `n − 1` iterations,
//!    so the `Θ(n log n)` cost comes solely from sorting.

use clrs::chapter_02::merge_sort_full;
use std::fmt::Debug;

fn main() {
    println!("Exercise 2.3-7: two-sum via sort-and-scan\n");

    let examples = [
        (vec![7, 11, 2, -5, 4], 6),
        (vec![1, 3, 5, 7, 9, 11], 14),
        (vec![12, -4, 8, 0, 6], 100), // no solution
    ];

    for (arr, target) in examples {
        println!("Input  S = {:?}, target = {}", arr, target);
        match two_sum_trace(&arr, target) {
            Some((i, j, sorted)) => println!(
                "  Found pair: {} + {} = {} (indices {} and {})\n",
                sorted[i], sorted[j], target, i, j
            ),
            None => println!("  No pair sums to {}\n", target),
        }
    }

    println!("Empirical step counts vs. n − 1 bound");
    println!("{:<6} {:>10}", "n", "iterations");
    for n in [4, 8, 16, 32, 64] {
        let (mut data, target) = worst_case_instance(n);
        merge_sort_full(&mut data);
        let iterations = two_pointer_iterations(&data, target);
        println!("{:<6} {:>10}", n, iterations);
    }
}

fn two_sum_trace<T>(arr: &[T], target: T) -> Option<(usize, usize, Vec<T>)>
where
    T: Ord + Copy + Debug + std::ops::Add<Output = T> + PartialEq,
{
    let mut sorted = arr.to_vec();
    merge_sort_full(&mut sorted);
    println!("  Sorted S = {:?}", sorted);

    let mut i = 0;
    let mut j = sorted.len().saturating_sub(1);
    let mut steps = 0;

    while i < j {
        steps += 1;
        assert!(
            invariant_holds(&sorted, target, i, j),
            "loop invariant violated before step {}",
            steps
        );

        let sum = sorted[i] + sorted[j];
        println!(
            "    step {:>2}: i = {:>2}, j = {:>2}, sum = {:>6?}",
            steps, i, j, sum
        );

        if sum == target {
            println!("    -> found sum == target");
            return Some((i, j, sorted));
        } else if sum < target {
            println!("    -> sum too small; increment i");
            i += 1;
        } else {
            println!("    -> sum too large; decrement j");
            j -= 1;
        }
    }

    println!(
        "    loop finished after {} steps without finding a pair",
        steps
    );
    assert!(
        invariant_holds(&sorted, target, i, j),
        "invariant must also hold at termination"
    );
    None
}

fn invariant_holds<T>(sorted: &[T], target: T, i: usize, j: usize) -> bool
where
    T: Copy + Ord + std::ops::Add<Output = T> + PartialEq,
{
    let n = sorted.len();

    for left in 0..i {
        for right in left + 1..n {
            if sorted[left] + sorted[right] == target {
                return false;
            }
        }
    }

    for right in (j + 1)..n {
        for left in 0..=j.min(n.saturating_sub(1)) {
            if sorted[left] + sorted[right] == target {
                return false;
            }
        }
    }

    true
}

fn two_pointer_iterations<T>(sorted: &[T], target: T) -> usize
where
    T: Copy + Ord + std::ops::Add<Output = T> + PartialEq,
{
    let mut i = 0;
    let mut j = sorted.len().saturating_sub(1);
    let mut iterations = 0;

    while i < j {
        iterations += 1;
        let sum = sorted[i] + sorted[j];
        if sum == target {
            break;
        } else if sum < target {
            i += 1;
        } else {
            j -= 1;
        }
    }
    iterations
}

fn worst_case_instance(n: usize) -> (Vec<i32>, i32) {
    let data: Vec<i32> = (0..n as i32).collect();
    let target = -1; // impossible sum given non-negative elements
    (data, target)
}
