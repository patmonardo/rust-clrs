// Heapsort exposition: narrates how BUILD-MAX-HEAP and HEAPSORT maintain the
// max-heap property, demonstrates loop invariants, and sketches the runtime
// analysis from CLRS Chapter 6.

use clrs::chapter_06::{build_max_heap, left, max_heapify, right};

fn main() {
    println!("=== Heapsort Exposition ===\n");

    let sample = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];

    println!("We start with the unsorted array: {:?}\n", sample);
    println!(
        "Goal: exhibit how the max-heap property acts as the Form/invariant that \
         guides heap construction and heapsort."
    );
    println!();

    demonstrate_build_max_heap(sample.clone());
    demonstrate_heapsort(sample.clone());
    demonstrate_runtime_bounds();
}

fn demonstrate_build_max_heap(mut arr: Vec<i32>) {
    println!("--- BUILD-MAX-HEAP narration ---");
    println!("Leaves already satisfy the property; we fix internal nodes bottom-up.\n");

    if arr.len() <= 1 {
        println!("Array of length {} is already a heap.\n", arr.len());
        return;
    }

    let start = (arr.len() / 2) - 1;
    for (step, i) in (0..=start).rev().enumerate() {
        println!("Step {}: heapify subtree rooted at index {} (value = {})", step + 1, i, arr[i]);
        let before = arr.clone();
        let heap_size = arr.len();
        narrate_heapify(&before, &mut arr, heap_size, i);
        debug_assert!(processed_region_is_heap(&arr, heap_size, i));
        println!("Heap after step {}: {:?}\n", step + 1, arr);
    }

    println!("Resulting max-heap: {:?}\n", arr);
}

fn demonstrate_heapsort(mut arr: Vec<i32>) {
    println!("--- HEAPSORT narration ---");
    println!("Phase 1: build the heap (reusing the narrator above).\n");
    demonstrate_build_max_heap(arr.clone());

    build_max_heap(&mut arr);
    println!("\nPhase 2: repeatedly remove the max, shrinking the heap prefix.\n");

    for heap_size in (1..arr.len()).rev() {
        println!("Heap prefix before extraction (size = {}): {:?}", heap_size + 1, &arr[..=heap_size]);
        debug_assert!(is_max_heap_prefix(&arr, heap_size + 1));

        println!("  Swap max element {} with arr[{}] = {}", arr[0], heap_size, arr[heap_size]);
        arr.swap(0, heap_size);
        println!("  After swap: {:?}", &arr[..=heap_size]);

        max_heapify(&mut arr, heap_size, 0);
        debug_assert!(is_max_heap_prefix(&arr, heap_size));
        println!("  Heap restored on prefix (size = {}): {:?}\n", heap_size, &arr[..heap_size]);
    }

    println!("Sorted result: {:?}\n", arr);
}

fn demonstrate_runtime_bounds() {
    println!("--- Runtime analysis sketch ---");
    println!("Total work of BUILD-MAX-HEAP equals the sum of subtree heights.");

    let sizes = [7_usize, 15, 31, 63];
    println!("  n | Σ heights (exact) | 2n (upper bound)\n-------------------------------");
    for &n in &sizes {
        let exact = sum_of_subtree_heights(n);
        println!("{:>4} | {:>15} | {:>13}", n, exact, 2 * n);
    }
    println!("Observation: Σ heights ≤ 2n ⇒ BUILD-MAX-HEAP runs in Θ(n).\n");

    println!("For HEAPSORT: each extraction heapifies a subtree of size k, contributing O(log k).");
    println!("Summing log k from k = n down to 1 yields Θ(n log n).\n");

    println!("Correctness sketch:");
    println!("  • Invariant (heap prefix): before each extraction, arr[0..heap_size) is a max-heap.");
    println!("  • Each swap moves the maximum to its final position at the end.");
    println!("  • MAX-HEAPIFY plus the inductive invariant ensures the prefix remains a heap.");
    println!("  • After n − 1 extractions the array is fully sorted in nondecreasing order.\n");
}

fn narrate_heapify(before: &[i32], arr: &mut [i32], heap_size: usize, i: usize) {
    let decision = heapify_decision(before, heap_size, i);
    match decision {
        HeapifyDecision::AlreadyHeap => {
            println!("  Children are in order; no swap needed.");
        }
        HeapifyDecision::SwapWithLeft { left_index, left_value } => {
            println!(
                "  Left child {} (index {}) exceeds parent {}; swapping and recursing.",
                left_value, left_index, before[i]
            );
        }
        HeapifyDecision::SwapWithRight {
            right_index,
            right_value,
            chosen_value,
        } => {
            println!(
                "  Right child {} (index {}) is the largest child (left child max = {}).",
                right_value, right_index, chosen_value
            );
            println!("  Swap parent {} with right child and recurse.", before[i]);
        }
    }

    max_heapify(arr, heap_size, i);

    if let Some(diff) = describe_diff(before, arr) {
        println!("  Effect of heapify: {}", diff);
    } else {
        println!("  Array unchanged.");
    }
}

fn describe_diff(before: &[i32], after: &[i32]) -> Option<String> {
    if before == after {
        return None;
    }
    let mut changes = Vec::new();
    for i in 0..before.len() {
        if before[i] != after[i] {
            changes.push(format!("arr[{}]: {} → {}", i, before[i], after[i]));
        }
    }
    Some(changes.join(", "))
}

enum HeapifyDecision {
    AlreadyHeap,
    SwapWithLeft { left_index: usize, left_value: i32 },
    SwapWithRight {
        right_index: usize,
        right_value: i32,
        chosen_value: i32,
    },
}

fn heapify_decision(arr: &[i32], heap_size: usize, i: usize) -> HeapifyDecision {
    let l = left(i);
    let r = right(i);
    let mut largest = i;

    if l < heap_size && arr[l] > arr[largest] {
        largest = l;
    }

    if r < heap_size && arr[r] > arr[largest] {
        return HeapifyDecision::SwapWithRight {
            right_index: r,
            right_value: arr[r],
            chosen_value: if largest == i { arr[i] } else { arr[largest] },
        };
    }

    if largest != i {
        HeapifyDecision::SwapWithLeft {
            left_index: l,
            left_value: arr[l],
        }
    } else {
        HeapifyDecision::AlreadyHeap
    }
}

fn is_max_heap_prefix(arr: &[i32], heap_size: usize) -> bool {
    for i in 0..heap_size {
        let l = left(i);
        let r = right(i);
        if l < heap_size && arr[i] < arr[l] {
            return false;
        }
        if r < heap_size && arr[i] < arr[r] {
            return false;
        }
    }
    true
}

fn processed_region_is_heap(arr: &[i32], heap_size: usize, current: usize) -> bool {
    for node in current..heap_size {
        if !is_max_heap_subtree(arr, heap_size, node) {
            return false;
        }
    }
    true
}

fn is_max_heap_subtree(arr: &[i32], heap_size: usize, i: usize) -> bool {
    let l = left(i);
    let r = right(i);
    let mut ok = true;
    if l < heap_size {
        ok &= arr[i] >= arr[l] && is_max_heap_subtree(arr, heap_size, l);
    }
    if r < heap_size {
        ok &= arr[i] >= arr[r] && is_max_heap_subtree(arr, heap_size, r);
    }
    ok
}

fn sum_of_subtree_heights(n: usize) -> usize {
    (0..n).map(|i| node_height(i, n)).sum()
}

fn node_height(i: usize, n: usize) -> usize {
    let l = left(i);
    let r = right(i);
    if l >= n && r >= n {
        return 0;
    }
    let left_height = if l < n { 1 + node_height(l, n) } else { 0 };
    let right_height = if r < n { 1 + node_height(r, n) } else { 0 };
    left_height.max(right_height)
}
