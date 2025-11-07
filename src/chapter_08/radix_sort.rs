//! Radix Sort (Section 8.3)
//!
//! Radix sort sorts on the least significant digit first, then the next,
//! and so on. It uses a stable sort (like counting sort) as a subroutine.

// Radix sort uses counting sort as a subroutine, but implements it inline

/// Sorts an array of integers using radix sort
///
/// This corresponds to RADIX-SORT from CLRS Section 8.3.
/// The algorithm sorts by processing digits from least significant to most.
///
/// # Arguments
/// * `arr` - The array to be sorted (must contain non-negative integers)
///
/// # Returns
/// A new sorted vector
///
/// # Complexity
/// - Time: Θ(d(n + k)) where d is the number of digits and k is the radix (10 for decimal)
/// - Space: Θ(n + k)
///
/// # Example
/// ```
/// use clrs::chapter_08::radix_sort;
/// let arr = vec![329, 457, 657, 839, 436, 720, 355];
/// let sorted = radix_sort(&arr);
/// assert_eq!(sorted, vec![329, 355, 436, 457, 657, 720, 839]);
/// ```
pub fn radix_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }
    
    // Find the maximum number to know number of digits
    let max = *arr.iter().max().unwrap();
    let mut result = arr.to_vec();
    
    // Do counting sort for every digit
    // Instead of passing digit number, pass exp (10^i where i is current digit number)
    let mut exp = 1;
    while max / exp > 0 {
        // Sort by current digit using counting sort
        result = radix_sort_counting_sort_by_digit(&result, exp);
        exp *= 10;
    }
    
    result
}

/// Helper function: counting sort by a specific digit
///
/// Sorts the array based on the digit at position `exp` (1, 10, 100, ...)
fn radix_sort_counting_sort_by_digit(arr: &[usize], exp: usize) -> Vec<usize> {
    let n = arr.len();
    let k = 9; // For decimal digits, range is 0-9
    
    // Count occurrences of each digit
    let mut c = vec![0; k + 1];
    
    for &value in arr {
        let digit = (value / exp) % 10;
        c[digit] += 1;
    }
    
    // Make cumulative
    for i in 1..=k {
        c[i] += c[i - 1];
    }
    
    // Build output array
    let mut b = vec![0; n];
    
    // Process in reverse to maintain stability
    for &value in arr.iter().rev() {
        let digit = (value / exp) % 10;
        b[c[digit] - 1] = value;
        c[digit] -= 1;
    }
    
    b
}

/// Sorts an array in-place using radix sort
///
/// # Arguments
/// * `arr` - The array to be sorted (modified in-place)
///
/// # Example
/// ```
/// use clrs::chapter_08::radix_sort_inplace;
/// let mut arr = vec![329, 457, 657, 839, 436, 720, 355];
/// radix_sort_inplace(&mut arr);
/// assert_eq!(arr, vec![329, 355, 436, 457, 657, 720, 839]);
/// ```
pub fn radix_sort_inplace(arr: &mut [usize]) {
    let sorted = radix_sort(arr);
    arr.copy_from_slice(&sorted);
}

/// Sorts integers in range [0, n³ - 1] in O(n) time (Exercise 8.3-4)
///
/// This converts numbers to base n, then uses radix sort with counting sort
/// as the stable sort subroutine.
///
/// # Arguments
/// * `arr` - The array to be sorted (must contain integers in range [0, n³ - 1])
///
/// # Returns
/// A new sorted vector
///
/// # Complexity
/// - Time: O(n) - 3 passes of counting sort, each O(n)
/// - Space: O(n)
///
/// # Example
/// ```
/// use clrs::chapter_08::radix_sort_base_n;
/// let arr = vec![100, 50, 200, 150, 75];
/// let sorted = radix_sort_base_n(&arr);
/// assert_eq!(sorted, vec![50, 75, 100, 150, 200]);
/// ```
pub fn radix_sort_base_n(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }
    
    let n = arr.len();
    let max = *arr.iter().max().unwrap();
    
    // Verify all elements are in range [0, n³ - 1]
    if max >= n * n * n {
        panic!("Element {} exceeds maximum value n³ - 1 = {}", max, n * n * n - 1);
    }
    
    // Convert to base n representation
    let mut base_n_numbers: Vec<Vec<usize>> = arr.iter()
        .map(|&x| {
            let mut digits = Vec::new();
            let mut num = x;
            // Convert to base n (at most 3 digits)
            for _ in 0..3 {
                digits.push(num % n);
                num /= n;
            }
            digits
        })
        .collect();
    
    // Radix sort on base n digits
    for digit_pos in 0..3 {
        base_n_numbers = radix_sort_base_n_by_digit(&base_n_numbers, digit_pos, n);
    }
    
    // Convert back from base n
    base_n_numbers.iter()
        .map(|digits| {
            digits.iter().enumerate()
                .map(|(i, &d)| d * n.pow(i as u32))
                .sum()
        })
        .collect()
}

/// Helper function: counting sort by a specific digit position in base n
fn radix_sort_base_n_by_digit(
    arr: &[Vec<usize>],
    digit_pos: usize,
    base: usize,
) -> Vec<Vec<usize>> {
    let n = arr.len();
    let k = base - 1; // Digits in base n are 0..(n-1)
    
    // Count occurrences
    let mut c = vec![0; k + 1];
    for digits in arr {
        let digit = if digit_pos < digits.len() { digits[digit_pos] } else { 0 };
        c[digit] += 1;
    }
    
    // Make cumulative
    for i in 1..=k {
        c[i] += c[i - 1];
    }
    
    // Build output
    let mut b = vec![vec![]; n];
    for digits in arr.iter().rev() {
        let digit = if digit_pos < digits.len() { digits[digit_pos] } else { 0 };
        b[c[digit] - 1] = digits.clone();
        c[digit] -= 1;
    }
    
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort_basic() {
        let arr = vec![329, 457, 657, 839, 436, 720, 355];
        let sorted = radix_sort(&arr);
        assert_eq!(sorted, vec![329, 355, 436, 457, 657, 720, 839]);
    }

    #[test]
    fn test_radix_sort_single_digit() {
        let arr = vec![5, 2, 8, 1, 9, 3];
        let sorted = radix_sort(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_radix_sort_varying_digits() {
        let arr = vec![5, 123, 45, 1, 9999, 789];
        let sorted = radix_sort(&arr);
        assert_eq!(sorted, vec![1, 5, 45, 123, 789, 9999]);
    }

    #[test]
    fn test_radix_sort_already_sorted() {
        let arr = vec![100, 200, 300, 400, 500];
        let sorted = radix_sort(&arr);
        assert_eq!(sorted, vec![100, 200, 300, 400, 500]);
    }

    #[test]
    fn test_radix_sort_inplace() {
        let mut arr = vec![329, 457, 657, 839, 436, 720, 355];
        radix_sort_inplace(&mut arr);
        assert_eq!(arr, vec![329, 355, 436, 457, 657, 720, 839]);
    }

    #[test]
    fn test_radix_sort_base_n() {
        // Example: n=5, so range is [0, 124]
        let arr = vec![100, 50, 75, 25, 0];
        let sorted = radix_sort_base_n(&arr);
        assert_eq!(sorted, vec![0, 25, 50, 75, 100]);
    }

    #[test]
    fn test_radix_sort_empty() {
        let arr: Vec<usize> = vec![];
        let sorted = radix_sort(&arr);
        assert!(sorted.is_empty());
    }
}

