//! Exercise 2.1-3 loop invariant demonstration for linear search.
//!
//! The exercise asks us to provide pseudocode for linear search and then *prove*
//! it correct with the three loop-invariant properties (initialization,
//! maintenance, termination). The core algorithm is already implemented in
//! `chapter_02::linear_search`; this example focuses on the proof scaffolding.
//!
//! We instrument each iteration to show:
//! 1. **Initialization** – before the first iteration the examined prefix is empty.
//! 2. **Maintenance** – if the invariant holds before an iteration, it still holds
//!    after we inspect `arr[i]`.
//! 3. **Termination** – once the loop exits, either we've returned the position
//!    where `target` was found, or the invariant tells us the target is absent.

use clrs::chapter_02::linear_search;
use std::fmt::Debug;

fn main() {
    explain_proof_structure();

    let arr = [31, 41, 59, 26, 41, 58];
    println!("\nCase 1: target = 59 is present (iterates until index 2)");
    let result = linear_search_with_invariant(&arr, &59);
    println!("Result from instrumented search: {result:?}");
    println!(
        "Result from library search:      {:?}",
        linear_search(&arr, &59)
    );

    let arr_absent = [10, 20, 30];
    println!("\nCase 2: target = 5 is absent (loop runs to completion)");
    let result = linear_search_with_invariant(&arr_absent, &5);
    println!("Result from instrumented search: {result:?}");
    println!(
        "Result from library search:      {:?}",
        linear_search(&arr_absent, &5)
    );
}

fn explain_proof_structure() {
    println!("Exercise 2.1-3 loop invariant:");
    println!(
        "  At the start of each iteration with index i, the prefix arr[0..i) \
         contains no occurrence of target."
    );
    println!("We check the three properties:");
    println!("  • Initialization: before the first iteration i = 0 and the prefix is empty.");
    println!(
        "  • Maintenance: if the invariant holds before inspecting arr[i], then after \
         processing we either return (if arr[i] == target) or advance i+1 while preserving \
         that the prefix still lacks target."
    );
    println!(
        "  • Termination: once the loop finishes, i == arr.len(), so the entire array \
         is the prefix; the invariant therefore proves target is absent and returning \
         None is correct."
    );
}

fn linear_search_with_invariant<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: PartialEq + Debug,
{
    println!("  Start of search over array: {arr:?}");
    let mut i = 0;

    while i < arr.len() {
        assert!(
            no_target_in_prefix(arr, target, i),
            "Invariant violated before examining index {i}"
        );
        println!(
            "    Invariant holds before iteration {i}: prefix = {:?}",
            &arr[..i]
        );

        if arr[i] == *target {
            println!(
                "    Found target at index {i}; prefix still lacks target, so returning Some({i})."
            );
            return Some(i);
        } else {
            println!(
                "    arr[{i}] = {:?} != {:?}; extending prefix to include this element.",
                arr[i], target
            );
            i += 1;
        }
    }

    assert!(
        i == arr.len(),
        "Loop terminated early without returning but index != len"
    );
    assert!(
        no_target_in_prefix(arr, target, i),
        "Invariant violated at termination"
    );
    println!(
        "    Loop terminated with i = {}; entire array examined; invariant ensures target absent.",
        i
    );
    None
}

fn no_target_in_prefix<T>(arr: &[T], target: &T, end: usize) -> bool
where
    T: PartialEq,
{
    arr[..end].iter().all(|x| x != target)
}
