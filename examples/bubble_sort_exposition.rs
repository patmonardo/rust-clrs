// bubble_sort_exposition.rs
// Expository walkthrough of CLRS Problem 2-2 (Correctness of bubblesort)
//
// We fuse the loop invariants articulated in Peng-Yu's solution notes with the
// presentation in the CLRS text.  The example performs bubble sort while
// asserting the two key invariants:
//   1. Inner loop: at the start of each iteration with indices (i, j), the
//      smallest element in the suffix A[i..] sits at or before position j.
//   2. Outer loop: at the start of iteration i, the prefix A[..i] already holds
//      the i smallest elements in sorted order.
//
// Running the example prints the array state as we “bubble up” small elements
// and finishes by confirming the Θ(n^2) runtime shape.

use std::fmt::Debug;

fn main() {
    let mut data = vec![5, 1, 4, 2, 8, 3];
    println!("Original array:     {:?}", data);
    let pass_counts = bubble_sort_with_invariants(&mut data);
    println!("Sorted array:       {:?}", data);
    println!(
        "Pass statistics:    {{ outer_passes: {}, comparisons: {}, swaps: {} }}",
        pass_counts.outer_passes, pass_counts.comparisons, pass_counts.swaps
    );

    // A second run on an already-sorted array demonstrates the Θ(n^2) best case.
    let mut sorted_again = data.clone();
    let pass_counts_sorted = bubble_sort_with_invariants(&mut sorted_again);
    println!("Already-sorted run: sorted = {:?}", sorted_again);
    println!(
        "Pass statistics:    {{ outer_passes: {}, comparisons: {}, swaps: {} }}",
        pass_counts_sorted.outer_passes, pass_counts_sorted.comparisons, pass_counts_sorted.swaps
    );
}

#[derive(Debug, Default)]
struct PassCounts {
    outer_passes: usize,
    comparisons: usize,
    swaps: usize,
}

fn bubble_sort_with_invariants<T>(arr: &mut [T]) -> PassCounts
where
    T: Ord + Clone + Debug,
{
    let mut counts = PassCounts::default();
    let n = arr.len();

    for i in 0..n {
        debug_assert!(
            outer_invariant_holds(arr, i),
            "outer invariant violated before iteration i={i}"
        );
        counts.outer_passes += 1;
        println!("\nOuter iteration i={}: current array = {:?}", i, arr);

        for j in ((i + 1)..n).rev() {
            debug_assert!(
                inner_invariant_holds(arr, i, j),
                "inner invariant violated at (i={}, j={})",
                i,
                j
            );
            counts.comparisons += 1;
            println!(
                "  Comparing positions {} and {}: {:?} vs {:?}",
                j - 1,
                j,
                arr[j - 1],
                arr[j]
            );

            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                counts.swaps += 1;
                println!("    Swapped -> {:?}", arr);
            }
        }
    }

    debug_assert!(
        outer_invariant_holds(arr, n),
        "outer invariant violated after completion"
    );
    counts
}

fn inner_invariant_holds<T>(arr: &[T], i: usize, j: usize) -> bool
where
    T: Ord,
{
    // Smallest element in arr[i..] must be no farther right than j.
    if i >= arr.len() {
        return true;
    }
    let suffix = &arr[i..];
    let (min_offset, _) = suffix
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .expect("non-empty suffix ensures min exists");
    i + min_offset <= j
}

fn outer_invariant_holds<T>(arr: &[T], i: usize) -> bool
where
    T: Ord + Clone,
{
    // Prefix arr[..i] should be sorted and equal to the i smallest elements.
    if i <= 1 {
        return true;
    }
    let prefix = &arr[..i];
    if !prefix.windows(2).all(|w| w[0] <= w[1]) {
        return false;
    }
    let mut sorted = arr.to_vec();
    sorted.sort();
    prefix == &sorted[..i]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_handles_reverse() {
        let mut data = vec![5, 4, 3, 2, 1];
        bubble_sort_with_invariants(&mut data);
        assert!(data.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn bubble_sort_handles_duplicates() {
        let mut data = vec![3, 1, 2, 1, 3];
        bubble_sort_with_invariants(&mut data);
        assert!(data.windows(2).all(|w| w[0] <= w[1]));
    }
}
