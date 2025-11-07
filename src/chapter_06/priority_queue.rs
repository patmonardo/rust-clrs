//! Priority Queue Operations (Section 6.5)
//!
//! This module implements a max-priority queue using a max-heap.
//! Priority queues support operations like extracting the maximum,
//! increasing a key, and inserting elements.

use super::heap::{max_heapify, parent};

/// Returns the maximum element of the heap
///
/// This corresponds to HEAP-MAXIMUM from CLRS Section 6.5.
///
/// # Arguments
/// * `arr` - The heap array (non-empty)
///
/// # Returns
/// A reference to the maximum element
///
/// # Panics
/// Panics if the array is empty
///
/// # Example
/// ```
/// use clrs::chapter_06::{build_max_heap, heap_maximum};
/// let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
/// build_max_heap(&mut arr);
/// assert_eq!(heap_maximum(&arr), &16);
/// ```
pub fn heap_maximum<T: Ord>(arr: &[T]) -> &T {
    &arr[0]
}

/// Extracts and returns the maximum element of the heap
///
/// This corresponds to HEAP-EXTRACT-MAX from CLRS Section 6.5.
///
/// # Arguments
/// * `arr` - The heap array
/// * `heap_size` - The current size of the heap (mutable reference)
///
/// # Returns
/// The maximum element
///
/// # Panics
/// Panics if the heap is empty
///
/// # Complexity
/// - Time: O(lg n)
/// - Space: O(lg n) for recursive MAX-HEAPIFY
///
/// # Example
/// ```
/// use clrs::chapter_06::{build_max_heap, heap_extract_max};
/// let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
/// build_max_heap(&mut arr);
/// let mut heap_size = arr.len();
/// let max = heap_extract_max(&mut arr, &mut heap_size);
/// assert_eq!(max, 16);
/// assert_eq!(heap_size, 9);
/// ```
pub fn heap_extract_max<T: Ord + Clone>(arr: &mut [T], heap_size: &mut usize) -> T {
    if *heap_size == 0 {
        panic!("heap underflow");
    }

    // CLRS: max = A[1]
    let max = arr[0].clone();

    // CLRS: A[1] = A[A.heap-size]
    arr[0] = arr[*heap_size - 1].clone();

    // CLRS: A.heap-size = A.heap-size - 1
    *heap_size -= 1;

    // CLRS: MAX-HEAPIFY(A, 1)
    max_heapify(arr, *heap_size, 0);

    max
}

/// Increases the key of element at index i to the new value
///
/// This corresponds to HEAP-INCREASE-KEY from CLRS Section 6.5.
///
/// # Arguments
/// * `arr` - The heap array
/// * `i` - The index of the element whose key is to be increased
/// * `key` - The new key value (must be >= current key)
///
/// # Panics
/// Panics if the new key is smaller than the current key
///
/// # Complexity
/// - Time: O(lg n)
/// - Space: O(1)
///
/// # Example
/// ```
/// use clrs::chapter_06::{build_max_heap, heap_increase_key};
/// let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
/// heap_increase_key(&mut arr, 8, 15);
/// assert_eq!(arr[0], 16); // Still max, but structure changed
/// ```
pub fn heap_increase_key<T: Ord + Clone>(arr: &mut [T], mut i: usize, key: T) {
    // CLRS: if key < A[i]
    if key < arr[i] {
        panic!("new key is smaller than current key");
    }

    // CLRS: A[i] = key
    arr[i] = key.clone();

    // CLRS: while i > 1 and A[PARENT(i)] < A[i]
    while i > 0 && arr[parent(i)] < arr[i] {
        // CLRS: exchange A[i] with A[PARENT(i)]
        let p = parent(i);
        arr.swap(i, p);
        // CLRS: i = PARENT(i)
        i = p;
    }
}

/// Optimized version of HEAP-INCREASE-KEY (Exercise 6.5-6)
///
/// Uses the idea from INSERTION-SORT's inner loop to reduce
/// assignments from 3 per swap to just 1 assignment.
///
/// # Arguments
/// * `arr` - The heap array
/// * `i` - The index of the element whose key is to be increased
/// * `key` - The new key value (must be >= current key)
///
/// # Example
/// ```
/// use clrs::chapter_06::{build_max_heap, heap_increase_key_optimized};
/// let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
/// heap_increase_key_optimized(&mut arr, 8, 15);
/// ```
pub fn heap_increase_key_optimized<T: Ord + Clone>(arr: &mut [T], mut i: usize, key: T) {
    if key < arr[i] {
        panic!("new key is smaller than current key");
    }

    // CLRS: while i > 1 and A[PARENT(i)] < key
    while i > 0 && arr[parent(i)] < key {
        // CLRS: A[i] = A[PARENT(i)]
        arr[i] = arr[parent(i)].clone();
        // CLRS: i = PARENT(i)
        i = parent(i);
    }

    // CLRS: A[i] = key
    arr[i] = key;
}

/// Inserts a new element into the heap
///
/// This corresponds to MAX-HEAP-INSERT from CLRS Section 6.5.
///
/// # Arguments
/// * `arr` - The heap array (must have capacity for one more element)
/// * `heap_size` - The current size of the heap (mutable reference, will be incremented)
/// * `key` - The key value to insert
///
/// # Note
/// This function assumes the array has enough capacity. In practice,
/// you might want to use a Vec and push/extend as needed.
///
/// # Complexity
/// - Time: O(lg n)
/// - Space: O(1)
///
/// # Example
/// ```
/// use clrs::chapter_06::{build_max_heap, max_heap_insert};
/// let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1, 0];
/// let mut heap_size = 10;
/// max_heap_insert(&mut arr, &mut heap_size, 15);
/// assert_eq!(heap_size, 11);
/// assert_eq!(arr[0], 16); // Max should still be at root
/// ```
pub fn max_heap_insert<T: Ord + Clone>(arr: &mut [T], heap_size: &mut usize, key: T) {
    // CLRS: A.heap-size = A.heap-size + 1
    *heap_size += 1;

    // CLRS: A[A.heap-size] = -∞
    // In practice, we set it to a value that will be replaced
    // For now, we'll use the key itself (will be overwritten)
    if *heap_size > arr.len() {
        panic!("heap overflow: array doesn't have capacity");
    }

    // CLRS: HEAP-INCREASE-KEY(A, A.heap-size, key)
    heap_increase_key_optimized(arr, *heap_size - 1, key);
}

/// Deletes the element at index i from the heap
///
/// This corresponds to HEAP-DELETE from CLRS Exercise 6.5-8.
///
/// # Arguments
/// * `arr` - The heap array
/// * `heap_size` - The current size of the heap (mutable reference)
/// * `i` - The index of the element to delete
///
/// # Complexity
/// - Time: O(lg n)
/// - Space: O(lg n) for recursive MAX-HEAPIFY
///
/// # Example
/// ```
/// use clrs::chapter_06::{build_max_heap, heap_delete};
/// let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
/// let mut heap_size = arr.len();
/// heap_delete(&mut arr, &mut heap_size, 2);
/// assert_eq!(heap_size, 9);
/// ```
pub fn heap_delete<T: Ord + Clone>(arr: &mut [T], heap_size: &mut usize, i: usize) {
    if i >= *heap_size {
        panic!("index out of bounds");
    }

    let last_idx = *heap_size - 1;
    let last_value = arr[last_idx].clone();

    // CLRS: if A[i] > A[A.heap-size]
    if arr[i] > last_value {
        // CLRS: A[i] = A[A.heap-size]
        arr[i] = last_value;
        // CLRS: MAX-HEAPIFY(A, i)
        max_heapify(arr, *heap_size - 1, i);
    } else {
        // CLRS: HEAP-INCREASE-KEY(A, i, A[A.heap-size])
        heap_increase_key_optimized(arr, i, last_value);
    }

    // CLRS: A.heap-size = A.heap-size - 1
    *heap_size -= 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapter_06::build_max_heap;

    #[test]
    fn test_heap_maximum() {
        let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_max_heap(&mut arr);
        assert_eq!(heap_maximum(&arr), &16);
    }

    #[test]
    fn test_heap_extract_max() {
        let mut arr = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_max_heap(&mut arr);
        let mut heap_size = arr.len();
        let max = heap_extract_max(&mut arr, &mut heap_size);
        assert_eq!(max, 16);
        assert_eq!(heap_size, 9);
        // Verify heap property is maintained
        assert_eq!(heap_maximum(&arr[..heap_size]), &14);
    }

    #[test]
    fn test_heap_increase_key() {
        let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        heap_increase_key(&mut arr, 8, 15);
        // After increase, 15 should bubble up
        assert_eq!(arr[0], 16);
        assert_eq!(arr[1], 15); // Should be at index 1 now
    }

    #[test]
    fn test_max_heap_insert() {
        let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1, 0];
        let mut heap_size = 10;
        max_heap_insert(&mut arr, &mut heap_size, 15);
        assert_eq!(heap_size, 11);
        // Verify heap property
        assert_eq!(arr[0], 16);
        assert_eq!(arr[1], 15);
    }

    #[test]
    fn test_heap_delete() {
        let mut arr = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let mut heap_size = arr.len();
        heap_delete(&mut arr, &mut heap_size, 2);
        assert_eq!(heap_size, 9);
        // Verify heap property is maintained
        for i in 0..heap_size {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < heap_size {
                assert!(arr[i] >= arr[left]);
            }
            if right < heap_size {
                assert!(arr[i] >= arr[right]);
            }
        }
    }
}

