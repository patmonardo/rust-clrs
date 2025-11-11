// inversions_exposition.rs
// Narrative exploration of CLRS Problem 2-4 (Counting inversions).
//
// We follow the outline in Peng-Yu's notes and the textbook:
//   * enumerate inversions directly to build intuition,
//   * relate insertion sort's inner loop to inversion count,
//   * implement a Θ(n log n) merge-based counter.
//
// Running this example prints the inversion set, compares the two
// counting approaches, and asserts agreement.

fn main() {
    let sample = vec![2, 3, 8, 6, 1];
    println!("Sample array: {:?}", sample);

    let inversions = list_inversions(&sample);
    println!("Inversions (i < j, A[i] > A[j]): {:?}", inversions);

    let (_, insertion_count, insertion_swaps) = insertion_sort_with_count(sample.clone());
    println!(
        "Insertion sort counted {} inversions via swaps",
        insertion_count
    );
    println!("Total swaps performed: {}\n", insertion_swaps);

    let (sorted, merge_count) = count_inversions_merge(sample.clone());
    println!("Merge-sort count: {} inversions", merge_count);
    println!("Sorted array from merge-counter: {:?}", sorted);

    assert_eq!(inversions.len() as u64, insertion_count);
    assert_eq!(insertion_count, merge_count);
    println!("All methods agree on the inversion total. Θ(n^2) vs Θ(n log n).\n");

    // Demonstrate worst-case array of size 6.
    let worst = vec![6, 5, 4, 3, 2, 1];
    let (_, worst_count) = count_inversions_merge(worst.clone());
    println!(
        "Worst-case array {:?} has {} inversions (n(n-1)/2)",
        worst, worst_count
    );
}

fn list_inversions(arr: &[i32]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

fn insertion_sort_with_count(mut arr: Vec<i32>) -> (Vec<i32>, u64, u64) {
    let mut inversions = 0_u64;
    let mut swaps = 0_u64;

    println!("Insertion sort trace:");
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j as isize - 1;

        println!(
            "  Inserting arr[{}]={}: prefix before insert {:?}",
            j,
            key,
            &arr[..j]
        );
        while i >= 0 && arr[i as usize] > key {
            arr[(i + 1) as usize] = arr[i as usize];
            i -= 1;
            inversions += 1;
            swaps += 1;
        }
        arr[(i + 1) as usize] = key;
        println!("    After insertion: {:?}", &arr[..=j]);
    }

    (arr, inversions, swaps)
}

fn count_inversions_merge(mut arr: Vec<i32>) -> (Vec<i32>, u64) {
    let n = arr.len();
    if n <= 1 {
        return (arr, 0);
    }
    let temp = arr.clone();
    let count = merge_sort_count(&mut arr, &mut temp.clone(), 0, n);
    (arr, count)
}

fn merge_sort_count(arr: &mut [i32], temp: &mut [i32], left: usize, right: usize) -> u64 {
    if right - left <= 1 {
        return 0;
    }
    let mid = left + (right - left) / 2;
    let left_count = merge_sort_count(arr, temp, left, mid);
    let right_count = merge_sort_count(arr, temp, mid, right);
    let cross = merge_count(arr, temp, left, mid, right);
    left_count + right_count + cross
}

fn merge_count(arr: &mut [i32], temp: &mut [i32], left: usize, mid: usize, right: usize) -> u64 {
    let mut i = left;
    let mut j = mid;
    let mut k = left;
    let mut inversions = 0_u64;

    while i < mid && j < right {
        if arr[i] <= arr[j] {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
            inversions += (mid - i) as u64;
        }
        k += 1;
    }

    while i < mid {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j < right {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    arr[left..right].copy_from_slice(&temp[left..right]);
    inversions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_matches_known_inversions() {
        let sample = vec![2, 3, 8, 6, 1];
        let (sorted, merge_count) = count_inversions_merge(sample.clone());
        let brute = list_inversions(&sample);
        assert_eq!(sorted, vec![1, 2, 3, 6, 8]);
        assert_eq!(merge_count, brute.len() as u64);
    }

    #[test]
    fn worst_case_has_n_choose_2() {
        let worst = vec![6, 5, 4, 3, 2, 1];
        let (_, count) = count_inversions_merge(worst);
        assert_eq!(count, 15);
    }
}
