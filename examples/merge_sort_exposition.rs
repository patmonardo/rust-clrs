// Merge sort exposition: narrates divide–conquer–combine, traces MERGE loop
// invariants, and confirms postconditions for each stage (CLRS Section 2.3).

use clrs::chapter_02::merge_sort_full;

fn main() {
    println!("=== Merge Sort Exposition ===\n");

    let sample = vec![8, 4, 7, 3, 5, 2, 9, 1];
    demonstrate_merge_sort(sample);
    runtime_outline();
}

fn demonstrate_merge_sort(mut arr: Vec<i32>) {
    println!("Input array: {:?}\n", arr);
    narrated_merge_sort_full(&mut arr);
    println!("Final sorted result: {:?}\n", arr);

    let mut baseline = arr.clone();
    merge_sort_full(&mut baseline);
    debug_assert_eq!(arr, baseline);
    println!("Narrated run matches `chapter_02::merge_sort_full`.\n");
}

fn narrated_merge_sort_full(arr: &mut [i32]) {
    match arr.len() {
        0 => {
            println!("Trivial empty array – nothing to do.\n");
        }
        1 => {
            println!("Singleton array already sorted ⇒ {:?}\n", arr);
        }
        len => {
            narrated_merge_sort(arr, 0, len - 1, 0);
        }
    }
}

fn narrated_merge_sort(arr: &mut [i32], p: usize, r: usize, depth: usize) {
    if p >= r {
        let indent = "  ".repeat(depth);
        println!(
            "{indent}Base case reached at index {p}: singleton {:?}",
            arr[p]
        );
        return;
    }

    let indent = "  ".repeat(depth);
    let q = (p + r) / 2;
    println!("{indent}Divide: sort indices [{p}, {r}] by splitting at q = {q}.");

    narrated_merge_sort(arr, p, q, depth + 1);
    debug_assert!(is_sorted(&arr[p..=q]));
    println!("{indent}  Left half [{p}, {q}] sorted ⇒ {:?}", &arr[p..=q]);

    narrated_merge_sort(arr, q + 1, r, depth + 1);
    debug_assert!(is_sorted(&arr[q + 1..=r]));
    println!(
        "{indent}  Right half [{q_plus}, {r}] sorted ⇒ {:?}",
        &arr[q + 1..=r],
        q_plus = q + 1
    );

    println!("{indent}Combine: MERGE the two sorted halves.");
    narrated_merge(arr, p, q, r, depth + 1);
    debug_assert!(is_sorted(&arr[p..=r]));
    println!("{indent}After combine: {:?}\n", &arr[p..=r]);
}

fn narrated_merge(arr: &mut [i32], p: usize, q: usize, r: usize, depth: usize) {
    let indent = "  ".repeat(depth);
    let left = arr[p..=q].to_vec();
    let right = arr[q + 1..=r].to_vec();

    println!(
        "{indent}MERGE on [{p}, {r}]: left = {:?}, right = {:?}",
        left, right
    );

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < left.len() && j < right.len() {
        println!(
            "{indent}  Compare left[{i}] = {} and right[{j}] = {}",
            left[i], right[j]
        );
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
            println!(
                "{indent}    Take left value ⇒ arr[{k}] = {}; i -> {i}, j -> {j}",
                arr[k]
            );
        } else {
            arr[k] = right[j];
            j += 1;
            println!(
                "{indent}    Take right value ⇒ arr[{k}] = {}; i -> {i}, j -> {j}",
                arr[k]
            );
        }
        k += 1;
        debug_assert!(merge_loop_invariant(arr, p, k, &left, i, &right, j));
        println!("{indent}    Prefix sorted: {:?}", &arr[p..k]);
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        println!(
            "{indent}  Exhausted right; copy remaining left value ⇒ arr[{k}] = {}; i -> {i}",
            arr[k]
        );
        k += 1;
        debug_assert!(merge_loop_invariant(arr, p, k, &left, i, &right, j));
        println!("{indent}    Prefix sorted: {:?}", &arr[p..k]);
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        println!(
            "{indent}  Exhausted left; copy remaining right value ⇒ arr[{k}] = {}; j -> {j}",
            arr[k]
        );
        k += 1;
        debug_assert!(merge_loop_invariant(arr, p, k, &left, i, &right, j));
        println!("{indent}    Prefix sorted: {:?}", &arr[p..k]);
    }

    debug_assert_eq!(k, r + 1);
    debug_assert!(merge_postcondition(arr, p, r, &left, &right));
    println!("{indent}MERGE complete ⇒ {:?}", &arr[p..=r]);
}

fn merge_loop_invariant(
    arr: &[i32],
    p: usize,
    k: usize,
    left: &[i32],
    i: usize,
    right: &[i32],
    j: usize,
) -> bool {
    if k < p {
        return false;
    }
    if k - p != i + j {
        return false;
    }
    let expected = merge_prefix(left, i, right, j);
    arr[p..p + expected.len()] == expected
}

fn merge_postcondition(arr: &[i32], p: usize, r: usize, left: &[i32], right: &[i32]) -> bool {
    let merged = merge_prefix(left, left.len(), right, right.len());
    arr[p..=r] == merged && is_sorted(&arr[p..=r])
}

fn merge_prefix(left: &[i32], i: usize, right: &[i32], j: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(i + j);
    let mut li = 0;
    let mut rj = 0;
    while li < i && rj < j {
        if left[li] <= right[rj] {
            result.push(left[li]);
            li += 1;
        } else {
            result.push(right[rj]);
            rj += 1;
        }
    }
    if li < i {
        result.extend_from_slice(&left[li..i]);
    }
    if rj < j {
        result.extend_from_slice(&right[rj..j]);
    }
    result
}

fn is_sorted(slice: &[i32]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}

fn runtime_outline() {
    println!("--- Runtime + correctness outline ---");
    println!("• Divide: constant cost to compute midpoint.");
    println!("• Conquer: two subproblems of size n/2 each.");
    println!("• Combine: MERGE scans each element once ⇒ Θ(n).");
    println!("⇒ Recurrence T(n) = 2T(n/2) + Θ(n) ⇒ Θ(n log n).\n");
    println!("Correctness hinges on the MERGE loop invariant: prefix built so far");
    println!("contains the |prefix| smallest elements of the two sublists in sorted order.");
    println!(
        "Termination follows when both sublists are exhausted, yielding a fully merged array."
    );
}
