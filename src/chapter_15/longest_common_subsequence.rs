//! Longest Common Subsequence (Section 15.4)
//!
//! Given two sequences, find the longest subsequence present in both of them.
//! A subsequence is a sequence that appears in the same relative order, but
//! not necessarily contiguous.

/// Computes the length of the longest common subsequence
///
/// This corresponds to LCS-LENGTH from CLRS Section 15.4.
///
/// # Arguments
/// * `x` - First sequence
/// * `y` - Second sequence
///
/// # Returns
/// A tuple (c, b) where:
/// - c[i][j] is the length of LCS of X[1..i] and Y[1..j]
/// - b[i][j] indicates the direction: '↖' (match), '↑' (from x), '←' (from y)
///
/// # Complexity
/// - Time: O(mn) where m = |x|, n = |y|
/// - Space: O(mn)
///
/// # Example
/// ```
/// use clrs::chapter_15::lcs_length;
/// let x = b"ABCBDAB";
/// let y = b"BDCABA";
/// let (c, _) = lcs_length(x, y);
/// assert_eq!(c[x.len()][y.len()], 4); // LCS length is 4
/// ```
pub fn lcs_length<T: Eq>(x: &[T], y: &[T]) -> (Vec<Vec<usize>>, Vec<Vec<char>>) {
    let m = x.len();
    let n = y.len();
    let mut c = vec![vec![0; n + 1]; m + 1];
    let mut b = vec![vec![' '; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if x[i - 1] == y[j - 1] {
                c[i][j] = c[i - 1][j - 1] + 1;
                b[i][j] = '↖';
            } else if c[i - 1][j] >= c[i][j - 1] {
                c[i][j] = c[i - 1][j];
                b[i][j] = '↑';
            } else {
                c[i][j] = c[i][j - 1];
                b[i][j] = '←';
            }
        }
    }

    (c, b)
}

/// Prints the longest common subsequence
///
/// This corresponds to PRINT-LCS from CLRS Section 15.4.
///
/// # Arguments
/// * `b` - The direction table from lcs_length
/// * `x` - First sequence
/// * `i` - Current index in x (1-indexed)
/// * `j` - Current index in y (1-indexed)
///
/// # Returns
/// The LCS as a vector
pub fn print_lcs<T: Clone>(b: &[Vec<char>], x: &[T], i: usize, j: usize) -> Vec<T> {
    if i == 0 || j == 0 {
        return Vec::new();
    }

    match b[i][j] {
        '↖' => {
            let mut lcs = print_lcs(b, x, i - 1, j - 1);
            lcs.push(x[i - 1].clone());
            lcs
        }
        '↑' => print_lcs(b, x, i - 1, j),
        '←' => print_lcs(b, x, i, j - 1),
        _ => Vec::new(),
    }
}

/// Prints the LCS without using the b table
///
/// This corresponds to PRINT-LCS from CLRS Exercise 15.4-2.
///
/// # Arguments
/// * `c` - The length table from lcs_length
/// * `x` - First sequence
/// * `y` - Second sequence
/// * `i` - Current index in x (1-indexed)
/// * `j` - Current index in y (1-indexed)
///
/// # Returns
/// The LCS as a vector
pub fn print_lcs_without_b<T: Clone + Eq>(
    c: &[Vec<usize>],
    x: &[T],
    y: &[T],
    i: usize,
    j: usize,
) -> Vec<T> {
    if c[i][j] == 0 {
        return Vec::new();
    }

    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        let mut lcs = print_lcs_without_b(c, x, y, i - 1, j - 1);
        lcs.push(x[i - 1].clone());
        lcs
    } else if i > 0 && (j == 0 || c[i - 1][j] >= c[i][j - 1]) {
        print_lcs_without_b(c, x, y, i - 1, j)
    } else {
        print_lcs_without_b(c, x, y, i, j - 1)
    }
}

/// Memoized version of LCS-LENGTH
///
/// This corresponds to MEMOIZED-LCS-LENGTH from CLRS Exercise 15.4-3.
///
/// # Arguments
/// * `x` - First sequence
/// * `y` - Second sequence
///
/// # Returns
/// The length table c where c[i][j] is the length of LCS of X[1..i] and Y[1..j]
///
/// # Complexity
/// - Time: O(mn)
/// - Space: O(mn)
pub fn memoized_lcs_length<T: Eq>(x: &[T], y: &[T]) -> Vec<Vec<usize>> {
    let m = x.len();
    let n = y.len();
    let mut c = vec![vec![usize::MAX; n + 1]; m + 1];
    memoized_lcs_length_aux(x, y, m, n, &mut c);
    c
}

fn memoized_lcs_length_aux<T: Eq>(
    x: &[T],
    y: &[T],
    i: usize,
    j: usize,
    c: &mut [Vec<usize>],
) -> usize {
    if c[i][j] != usize::MAX {
        return c[i][j];
    }

    let result = if i == 0 || j == 0 {
        0
    } else if x[i - 1] == y[j - 1] {
        memoized_lcs_length_aux(x, y, i - 1, j - 1, c) + 1
    } else {
        memoized_lcs_length_aux(x, y, i - 1, j, c).max(memoized_lcs_length_aux(x, y, i, j - 1, c))
    };

    c[i][j] = result;
    result
}

/// Computes LCS length using only O(min(m, n)) space
///
/// This corresponds to the space-optimized version from CLRS Exercise 15.4-4.
///
/// # Arguments
/// * `x` - First sequence
/// * `y` - Second sequence
///
/// # Returns
/// The length of the LCS
///
/// # Complexity
/// - Time: O(mn)
/// - Space: O(min(m, n))
pub fn lcs_length_space_optimized<T: Eq>(x: &[T], y: &[T]) -> usize {
    let (shorter, longer) = if x.len() <= y.len() { (x, y) } else { (y, x) };

    let mut prev = vec![0; shorter.len() + 1];
    let mut curr = vec![0; shorter.len() + 1];

    for item in longer.iter() {
        std::mem::swap(&mut prev, &mut curr);
        curr[0] = 0;

        for (j, shorter_item) in shorter.iter().enumerate() {
            if item == shorter_item {
                curr[j + 1] = prev[j] + 1;
            } else {
                curr[j + 1] = prev[j + 1].max(curr[j]);
            }
        }
    }

    curr[shorter.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_length() {
        let x = b"ABCBDAB";
        let y = b"BDCABA";
        let (c, _) = lcs_length(x, y);
        assert_eq!(c[x.len()][y.len()], 4);
    }

    #[test]
    fn test_print_lcs() {
        let x = b"ABCBDAB";
        let y = b"BDCABA";
        let (_c, b) = lcs_length(x, y);
        let lcs = print_lcs(&b, x, x.len(), y.len());
        assert_eq!(lcs.len(), 4);
        // LCS could be "BCBA" or "BCAB" or "BDAB"
    }

    #[test]
    fn test_print_lcs_without_b() {
        let x = b"ABCBDAB";
        let y = b"BDCABA";
        let (c, _) = lcs_length(x, y);
        let lcs = print_lcs_without_b(&c, x, y, x.len(), y.len());
        assert_eq!(lcs.len(), 4);
    }

    #[test]
    fn test_memoized_lcs_length() {
        let x = b"ABCBDAB";
        let y = b"BDCABA";
        let c = memoized_lcs_length(x, y);
        assert_eq!(c[x.len()][y.len()], 4);
    }

    #[test]
    fn test_lcs_length_space_optimized() {
        let x = b"ABCBDAB";
        let y = b"BDCABA";
        let length = lcs_length_space_optimized(x, y);
        assert_eq!(length, 4);
    }
}
