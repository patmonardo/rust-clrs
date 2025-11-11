// Quicksort exposition: narrates PARTITION, highlights its loop invariant, and
// walks through the recursive structure of QUICKSORT (CLRS Chapter 7).

use clrs::chapter_07::{partition, quicksort_full};

fn main() {
    println!("=== Quicksort Exposition ===\n");

    let sample = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];

    demonstrate_partition(&sample);
    demonstrate_recursive_structure(&sample);
    demonstrate_worst_case_pivots();
    outline_correctness_arguments();
    runtime_discussion();
}

fn demonstrate_partition(sample: &[i32]) {
    println!("--- PARTITION trace (CLRS 7.1 sample) ---");
    println!("Input slice: {:?}\n", sample);

    let mut narrated = sample.to_vec();
    let p = 0;
    let r = narrated.len() - 1;
    let q = narrated_partition(&mut narrated, p, r, 1);

    println!("After narrated PARTITION: {:?}", narrated);
    println!(
        "Pivot settled at index q = {} (value = {})\n",
        q, narrated[q]
    );

    // Sanity-check against the library implementation.
    let mut baseline = sample.to_vec();
    let baseline_q = partition(&mut baseline, p, r);
    debug_assert_eq!(q, baseline_q);
    debug_assert_eq!(narrated, baseline);
    println!("Narration agrees with `chapter_07::partition`.\n");
}

fn demonstrate_recursive_structure(sample: &[i32]) {
    println!("--- QUICKSORT recursion trace (same sample) ---");
    let mut arr = sample.to_vec();
    let last = arr.len() - 1;
    narrated_quicksort(&mut arr, 0, last, 0);
    println!("Sorted result: {:?}\n", arr);

    let mut baseline = sample.to_vec();
    quicksort_full(&mut baseline);
    debug_assert_eq!(arr, baseline);
}

fn demonstrate_worst_case_pivots() {
    println!("--- Worst-case pivot behaviour ---");
    let worst = vec![1, 2, 3, 4, 5, 6];
    println!("Already sorted input: {:?}\n", worst);

    let mut working = worst.clone();
    let mut log = Vec::new();
    let last = working.len() - 1;
    collect_pivot_positions(&mut working, 0, last, &mut log);

    for (p, r, q) in &log {
        let left = if *q > *p { q - p } else { 0 };
        let right = r - q;
        println!(
            "  PARTITION call with p = {:>2}, r = {:>2} chooses pivot at {:>2}; \
             left size = {:>2}, right size = {:>2}",
            p, r, q, left, right
        );
    }
    println!(
        "\nBecause the pivot is always the largest element (last position), the right subproblem \
         is empty and we recurse down a single chain of length n.\n"
    );
}

fn outline_correctness_arguments() {
    println!("--- Correctness skeleton ---");
    println!("• PARTITION loop invariant:");
    println!("  - `arr[p..=i]` holds elements ≤ pivot.");
    println!("  - `arr[i+1..j]` (the elements already scanned and not swapped) are > pivot.");
    println!("  - `arr[r]` keeps the pivot value until the final swap.");
    println!("• After the loop, swapping the pivot into position `i + 1` yields:");
    println!("  - Every index < q has value ≤ pivot, every index > q has value > pivot.");
    println!("• QUICKSORT correctness follows by structural induction:");
    println!("  - Base case |A| ≤ 1: already sorted.");
    println!("  - Inductive step: partition places the pivot in final position; recursively sorted subarrays stay disjoint and cover the rest.");
    println!(
        "  - The algorithm therefore returns the array in nondecreasing order without discarding or duplicating elements.\n"
    );
}

fn runtime_discussion() {
    println!("--- Runtime discussion ---");
    println!("• Partitioning touches each element exactly once ⇒ Θ(n) on a subarray of length n.");
    println!("• Balanced pivots give the recurrence T(n) = 2T(n/2) + Θ(n) ⇒ Θ(n log n).");
    println!("• Consistently poor pivots (e.g., already sorted input) lead to T(n) = T(n − 1) + Θ(n) ⇒ Θ(n²).");
    println!(
        "• Randomized pivot selection or median-of-three heuristics are practical ways to drive the expectation toward Θ(n log n).\n"
    );
}

fn narrated_quicksort(arr: &mut [i32], p: usize, r: usize, depth: usize) {
    if p >= r {
        return;
    }

    let indent = "  ".repeat(depth);
    println!("{indent}QUICKSORT(p = {p}, r = {r}) on {:?}", &arr[p..=r]);

    let q = narrated_partition(arr, p, r, depth + 1);
    println!(
        "{indent}Pivot settled at {q} (value = {}), recurse on subproblems.",
        arr[q]
    );

    if q > p {
        narrated_quicksort(arr, p, q - 1, depth + 1);
    }
    if q + 1 <= r {
        narrated_quicksort(arr, q + 1, r, depth + 1);
    }
}

fn narrated_partition(arr: &mut [i32], p: usize, r: usize, depth: usize) -> usize {
    let indent = "  ".repeat(depth);
    let pivot = arr[r];
    println!(
        "{indent}PARTITION(p = {p}, r = {r}) pivot = {} (arr[{r}])",
        pivot
    );

    let mut i: isize = p as isize - 1;
    for j in p..r {
        debug_assert!(partition_invariant_holds(arr, p, j, i, pivot, r));
        println!("{indent}  Inspect j = {j}: arr[{j}] = {}", arr[j]);
        if arr[j] <= pivot {
            i += 1;
            println!(
                "{indent}    arr[{j}] ≤ pivot ⇒ increment i to {i} and swap arr[{i}] with arr[{j}]"
            );
            arr.swap(i as usize, j);
            println!("{indent}    Current view: {:?}", &arr[p..=r]);
        } else {
            println!("{indent}    arr[{j}] > pivot ⇒ do nothing");
        }
        debug_assert!(partition_invariant_holds(arr, p, j + 1, i, pivot, r));
    }

    let pivot_slot = (i + 1) as usize;
    println!("{indent}  Final swap: move pivot into arr[{pivot_slot}] (swap with arr[{r}])");
    arr.swap(pivot_slot, r);
    println!("{indent}  After pivot placement: {:?}", &arr[p..=r]);
    debug_assert!(partition_postcondition_holds(arr, p, r, pivot_slot));
    pivot_slot
}

fn partition_invariant_holds(
    arr: &[i32],
    p: usize,
    j: usize,
    i: isize,
    pivot: i32,
    r: usize,
) -> bool {
    if j > r {
        return false;
    }
    if arr[r] != pivot {
        return false;
    }
    if i >= p as isize {
        for k in p..=i as usize {
            if arr[k] > pivot {
                return false;
            }
        }
    }

    let start = ((i + 1).max(p as isize)) as usize;
    if start < j {
        for k in start..j {
            if arr[k] <= pivot {
                return false;
            }
        }
    }
    true
}

fn partition_postcondition_holds(arr: &[i32], p: usize, r: usize, q: usize) -> bool {
    if q < p || q > r {
        return false;
    }
    let pivot = arr[q];
    for k in p..q {
        if arr[k] > pivot {
            return false;
        }
    }
    for k in (q + 1)..=r {
        if arr[k] <= pivot {
            return false;
        }
    }
    true
}

fn collect_pivot_positions(
    arr: &mut [i32],
    p: usize,
    r: usize,
    acc: &mut Vec<(usize, usize, usize)>,
) {
    if p >= r {
        return;
    }
    let q = partition(arr, p, r);
    acc.push((p, r, q));
    if q > p {
        collect_pivot_positions(arr, p, q - 1, acc);
    }
    if q + 1 <= r {
        collect_pivot_positions(arr, q + 1, r, acc);
    }
}
