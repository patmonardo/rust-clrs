//! Heap Operations (Sections 6.1-6.3)
//!
//! This module contains basic heap operations including parent/child indexing,
//! MAX-HEAPIFY, BUILD-MAX-HEAP, and related functions.

/// Returns the index of the parent of node i in a 1-based heap
///
/// This corresponds to PARENT(i) from CLRS.
/// In CLRS, arrays are 1-based, but we use 0-based indexing.
/// So for 0-based: parent(i) = (i - 1) / 2
///
/// # Arguments
/// * `i` - The index of the node (0-based)
///
/// # Example
/// ```
/// use clrs::chapter_06::parent;
/// assert_eq!(parent(1), 0);
/// assert_eq!(parent(2), 0);
/// assert_eq!(parent(3), 1);
/// ```
#[inline]
pub fn parent(i: usize) -> usize {
    if i == 0 {
        0
    } else {
        (i - 1) / 2
    }
}

/// Returns the index of the left child of node i in a 1-based heap
///
/// This corresponds to LEFT(i) from CLRS.
/// In CLRS, arrays are 1-based: LEFT(i) = 2i
/// For 0-based: left(i) = 2i + 1
///
/// # Arguments
/// * `i` - The index of the node (0-based)
///
/// # Example
/// ```
/// use clrs::chapter_06::left;
/// assert_eq!(left(0), 1);
/// assert_eq!(left(1), 3);
/// ```
#[inline]
pub fn left(i: usize) -> usize {
    2 * i + 1
}

/// Returns the index of the right child of node i in a 1-based heap
///
/// This corresponds to RIGHT(i) from CLRS.
/// In CLRS, arrays are 1-based: RIGHT(i) = 2i + 1
/// For 0-based: right(i) = 2i + 2
///
/// # Arguments
/// * `i` - The index of the node (0-based)
///
/// # Example
/// ```
/// use clrs::chapter_06::right;
/// assert_eq!(right(0), 2);
/// assert_eq!(right(1), 4);
/// ```
#[inline]
pub fn right(i: usize) -> usize {
    2 * i + 2
}

/// Maintains the max-heap property for a subtree rooted at index i
///
/// This corresponds to MAX-HEAPIFY from CLRS Section 6.2.
/// Assumes that the subtrees rooted at LEFT(i) and RIGHT(i) are max-heaps,
/// but A[i] might be smaller than its children.
///
/// # Arguments
/// * `arr` - The array representing the heap
/// * `heap_size` - The size of the heap (may be smaller than array length)
/// * `i` - The index of the root of the subtree (0-based)
///
/// # Complexity
/// - Time: O(lg n) where n is the heap size
/// - Space: O(lg n) for recursive version, O(1) for iterative version
///
/// # Example
/// ```
/// use clrs::chapter_06::max_heapify;
/// let mut arr = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
/// max_heapify(&mut arr, arr.len(), 1);
/// // After heapify, the subtree rooted at index 1 should satisfy max-heap property
/// ```
pub fn max_heapify<T: Ord>(arr: &mut [T], heap_size: usize, i: usize) {
    let mut largest = i;
    let l = left(i);
    let r = right(i);

    // CLRS: if l <= A.heap-size and A[l] > A[i]
    if l < heap_size && arr[l] > arr[largest] {
        largest = l;
    }

    // CLRS: if r <= A.heap-size and A[r] > A[largest]
    if r < heap_size && arr[r] > arr[largest] {
        largest = r;
    }

    // CLRS: if largest != i
    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, heap_size, largest);
    }
}

/// Iterative version of MAX-HEAPIFY (Exercise 6.2-5)
///
/// This is more efficient in terms of constant factors as it avoids recursion.
///
/// # Arguments
/// * `arr` - The array representing the heap
/// * `heap_size` - The size of the heap
/// * `i` - The index of the root of the subtree (0-based)
///
/// # Example
/// ```
/// use clrs::chapter_06::max_heapify_iterative;
/// let mut arr = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
/// max_heapify_iterative(&mut arr, arr.len(), 1);
/// ```
pub fn max_heapify_iterative<T: Ord>(arr: &mut [T], heap_size: usize, mut i: usize) {
    loop {
        let mut largest = i;
        let l = left(i);
        let r = right(i);

        if l < heap_size && arr[l] > arr[largest] {
            largest = l;
        }

        if r < heap_size && arr[r] > arr[largest] {
            largest = r;
        }

        if largest == i {
            return;
        }

        arr.swap(i, largest);
        i = largest;
    }
}

/// Maintains the min-heap property for a subtree rooted at index i
///
/// This corresponds to MIN-HEAPIFY from CLRS Exercise 6.2-2.
///
/// # Arguments
/// * `arr` - The array representing the heap
/// * `heap_size` - The size of the heap
/// * `i` - The index of the root of the subtree (0-based)
///
/// # Example
/// ```
/// use clrs::chapter_06::min_heapify;
/// let mut arr = vec![1, 10, 3, 14, 7, 9, 16, 2, 8, 4];
/// min_heapify(&mut arr, arr.len(), 1);
/// ```
pub fn min_heapify<T: Ord>(arr: &mut [T], heap_size: usize, i: usize) {
    let mut smallest = i;
    let l = left(i);
    let r = right(i);

    if l < heap_size && arr[l] < arr[smallest] {
        smallest = l;
    }

    if r < heap_size && arr[r] < arr[smallest] {
        smallest = r;
    }

    if smallest != i {
        arr.swap(i, smallest);
        min_heapify(arr, heap_size, smallest);
    }
}

/// Builds a max-heap from an unordered array
///
/// This corresponds to BUILD-MAX-HEAP from CLRS Section 6.3.
/// The procedure builds a max-heap by calling MAX-HEAPIFY in a bottom-up manner.
///
/// # Arguments
/// * `arr` - The array to be converted into a max-heap (modified in-place)
///
/// # Complexity
/// - Time: O(n) where n is the array length
/// - Space: O(lg n) for recursive MAX-HEAPIFY calls
///
/// # Example
/// ```
/// use clrs::chapter_06::build_max_heap;
/// let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
/// build_max_heap(&mut arr);
/// // arr is now a max-heap
/// ```
pub fn build_max_heap<T: Ord>(arr: &mut [T]) {
    let heap_size = arr.len();
    // CLRS: for i = floor(A.length / 2) downto 1
    // For 0-based: from (heap_size / 2 - 1) down to 0
    // But we need to be careful: leaves are at indices >= heap_size / 2
    // So we start from the last parent node
    if heap_size <= 1 {
        return;
    }

    // Start from the last parent node (index of last node's parent)
    let start = (heap_size / 2) - 1;
    for i in (0..=start).rev() {
        max_heapify(arr, heap_size, i);
    }
}

/// Builds a min-heap from an unordered array
///
/// This is similar to BUILD-MAX-HEAP but for min-heaps.
///
/// # Arguments
/// * `arr` - The array to be converted into a min-heap (modified in-place)
///
/// # Example
/// ```
/// use clrs::chapter_06::build_min_heap;
/// let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
/// build_min_heap(&mut arr);
/// // arr is now a min-heap
/// ```
pub fn build_min_heap<T: Ord>(arr: &mut [T]) {
    let heap_size = arr.len();
    if heap_size <= 1 {
        return;
    }

    let start = (heap_size / 2) - 1;
    for i in (0..=start).rev() {
        min_heapify(arr, heap_size, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent() {
        assert_eq!(parent(0), 0); // Root's parent is itself
        assert_eq!(parent(1), 0);
        assert_eq!(parent(2), 0);
        assert_eq!(parent(3), 1);
        assert_eq!(parent(4), 1);
        assert_eq!(parent(5), 2);
    }

    #[test]
    fn test_left() {
        assert_eq!(left(0), 1);
        assert_eq!(left(1), 3);
        assert_eq!(left(2), 5);
        assert_eq!(left(3), 7);
    }

    #[test]
    fn test_right() {
        assert_eq!(right(0), 2);
        assert_eq!(right(1), 4);
        assert_eq!(right(2), 6);
        assert_eq!(right(3), 8);
    }

    #[test]
    fn test_max_heapify() {
        // Example from CLRS: [16, 4, 10, 14, 7, 9, 3, 2, 8, 1]
        // After heapifying index 1, should get: [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]
        let mut arr = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let heap_size = arr.len();
        max_heapify(&mut arr, heap_size, 1);
        // Verify max-heap property is maintained
        assert!(arr[1] >= arr[left(1)]); // 14 >= 8
        assert!(arr[1] >= arr[right(1)]); // 14 >= 7
    }

    #[test]
    fn test_max_heapify_iterative() {
        let mut arr = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let heap_size = arr.len();
        max_heapify_iterative(&mut arr, heap_size, 1);
        assert!(arr[1] >= arr[left(1)]);
        assert!(arr[1] >= arr[right(1)]);
    }

    #[test]
    fn test_build_max_heap() {
        // Example from CLRS 6.3-1
        let mut arr = vec![5, 3, 17, 10, 84, 19, 6, 22, 9];
        build_max_heap(&mut arr);
        // Verify it's a max-heap
        for i in 0..arr.len() {
            let l = left(i);
            let r = right(i);
            if l < arr.len() {
                assert!(arr[i] >= arr[l], "Heap property violated at index {}", i);
            }
            if r < arr.len() {
                assert!(arr[i] >= arr[r], "Heap property violated at index {}", i);
            }
        }
    }

    #[test]
    fn test_build_min_heap() {
        let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_min_heap(&mut arr);
        // Verify it's a min-heap
        for i in 0..arr.len() {
            let l = left(i);
            let r = right(i);
            if l < arr.len() {
                assert!(
                    arr[i] <= arr[l],
                    "Min-heap property violated at index {}",
                    i
                );
            }
            if r < arr.len() {
                assert!(
                    arr[i] <= arr[r],
                    "Min-heap property violated at index {}",
                    i
                );
            }
        }
    }

    #[test]
    fn test_build_max_heap_empty() {
        let mut arr: Vec<i32> = vec![];
        build_max_heap(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_build_max_heap_single() {
        let mut arr = vec![42];
        build_max_heap(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}
