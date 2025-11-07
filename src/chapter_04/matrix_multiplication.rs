//! Matrix Multiplication (Section 4.2)
//!
//! This module implements matrix multiplication algorithms including
//! the standard divide-and-conquer approach and Strassen's algorithm.

/// Multiplies two square matrices using standard divide-and-conquer
///
/// This corresponds to SQUARE-MATRIX-MULTIPLY-RECURSIVE from CLRS Section 4.2.
/// Note: This is a naive recursive implementation that divides matrices into
/// four quadrants. For efficiency, Strassen's algorithm is preferred.
///
/// # Arguments
/// * `a` - First matrix (n×n)
/// * `b` - Second matrix (n×n)
/// * `row_a` - Starting row of submatrix in A (0-based)
/// * `col_a` - Starting column of submatrix in A (0-based)
/// * `row_b` - Starting row of submatrix in B (0-based)
/// * `col_b` - Starting column of submatrix in B (0-based)
/// * `size` - Size of the submatrices to multiply
///
/// # Returns
/// Resulting n×n matrix C = A × B
///
/// # Complexity
/// - Time: O(n³)
/// - Space: O(n²)
pub fn square_matrix_multiply_recursive(
    a: &[Vec<i64>],
    b: &[Vec<i64>],
    row_a: usize,
    col_a: usize,
    row_b: usize,
    col_b: usize,
    size: usize,
) -> Vec<Vec<i64>> {
    let mut c = vec![vec![0; size]; size];

    // Base case: 1×1 matrix
    if size == 1 {
        c[0][0] = a[row_a][col_a] * b[row_b][col_b];
        return c;
    }

    let half = size / 2;

    // C11 = A11 * B11 + A12 * B21
    let c11_part1 = square_matrix_multiply_recursive(a, b, row_a, col_a, row_b, col_b, half);
    let c11_part2 =
        square_matrix_multiply_recursive(a, b, row_a, col_a + half, row_b + half, col_b, half);
    add_matrices(&c11_part1, &c11_part2, &mut c, 0, 0, half);

    // C12 = A11 * B12 + A12 * B22
    let c12_part1 = square_matrix_multiply_recursive(a, b, row_a, col_a, row_b, col_b + half, half);
    let c12_part2 = square_matrix_multiply_recursive(
        a,
        b,
        row_a,
        col_a + half,
        row_b + half,
        col_b + half,
        half,
    );
    add_matrices(&c12_part1, &c12_part2, &mut c, 0, half, half);

    // C21 = A21 * B11 + A22 * B21
    let c21_part1 = square_matrix_multiply_recursive(a, b, row_a + half, col_a, row_b, col_b, half);
    let c21_part2 = square_matrix_multiply_recursive(
        a,
        b,
        row_a + half,
        col_a + half,
        row_b + half,
        col_b,
        half,
    );
    add_matrices(&c21_part1, &c21_part2, &mut c, half, 0, half);

    // C22 = A21 * B12 + A22 * B22
    let c22_part1 =
        square_matrix_multiply_recursive(a, b, row_a + half, col_a, row_b, col_b + half, half);
    let c22_part2 = square_matrix_multiply_recursive(
        a,
        b,
        row_a + half,
        col_a + half,
        row_b + half,
        col_b + half,
        half,
    );
    add_matrices(&c22_part1, &c22_part2, &mut c, half, half, half);

    c
}

/// Helper function to add two matrices and store result in a submatrix of C
fn add_matrices(
    a: &[Vec<i64>],
    b: &[Vec<i64>],
    c: &mut [Vec<i64>],
    start_row: usize,
    start_col: usize,
    size: usize,
) {
    for i in 0..size {
        for j in 0..size {
            c[start_row + i][start_col + j] = a[i][j] + b[i][j];
        }
    }
}

/// Multiplies two square matrices using Strassen's algorithm
///
/// This corresponds to STRASSEN from CLRS Section 4.2.
/// Strassen's algorithm reduces the number of multiplications from 8 to 7,
/// resulting in O(n^lg 7) ≈ O(n^2.81) time complexity.
///
/// # Arguments
/// * `a` - First matrix (must be n×n where n is a power of 2)
/// * `b` - Second matrix (must be n×n where n is a power of 2)
///
/// # Returns
/// Resulting n×n matrix C = A × B
///
/// # Panics
/// Panics if matrices are not square, have different sizes, or size is not a power of 2.
///
/// # Example
/// ```
/// use clrs::chapter_04::strassen_matrix_multiply;
/// let a = vec![vec![1, 3], vec![7, 5]];
/// let b = vec![vec![6, 8], vec![4, 2]];
/// let c = strassen_matrix_multiply(&a, &b);
/// // Result: [[18, 14], [62, 66]]
/// ```
///
/// # Complexity
/// - Time: O(n^lg 7) ≈ O(n^2.81)
/// - Space: O(n²)
pub fn strassen_matrix_multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();

    // Validate input
    assert_eq!(n, a[0].len(), "Matrix A must be square");
    assert_eq!(n, b.len(), "Matrices must have same size");
    assert_eq!(n, b[0].len(), "Matrix B must be square");
    assert!(n > 0, "Matrices cannot be empty");
    assert!(n.is_power_of_two(), "Matrix size must be a power of 2");

    // Base case: 1×1 matrix
    if n == 1 {
        return vec![vec![a[0][0] * b[0][0]]];
    }

    let half = n / 2;

    // Divide matrices into submatrices
    // A11 = A[0..half][0..half]
    // A12 = A[0..half][half..n]
    // A21 = A[half..n][0..half]
    // A22 = A[half..n][half..n]
    // Same for B

    let a11 = extract_submatrix(a, 0, 0, half);
    let a12 = extract_submatrix(a, 0, half, half);
    let a21 = extract_submatrix(a, half, 0, half);
    let a22 = extract_submatrix(a, half, half, half);

    let b11 = extract_submatrix(b, 0, 0, half);
    let b12 = extract_submatrix(b, 0, half, half);
    let b21 = extract_submatrix(b, half, 0, half);
    let b22 = extract_submatrix(b, half, half, half);

    // Compute the 10 matrices S1 through S10
    // CLRS: S1 = B12 - B22
    let s1 = subtract_matrices(&b12, &b22);
    // CLRS: S2 = A11 + A12
    let s2 = add_matrices_full(&a11, &a12);
    // CLRS: S3 = A21 + A22
    let s3 = add_matrices_full(&a21, &a22);
    // CLRS: S4 = B21 - B11
    let s4 = subtract_matrices(&b21, &b11);
    // CLRS: S5 = A11 + A22
    let s5 = add_matrices_full(&a11, &a22);
    // CLRS: S6 = B11 + B22
    let s6 = add_matrices_full(&b11, &b22);
    // CLRS: S7 = A12 - A22
    let s7 = subtract_matrices(&a12, &a22);
    // CLRS: S8 = B21 + B22
    let s8 = add_matrices_full(&b21, &b22);
    // CLRS: S9 = A11 - A21
    let s9 = subtract_matrices(&a11, &a21);
    // CLRS: S10 = B11 + B12
    let s10 = add_matrices_full(&b11, &b12);

    // Compute the 7 products P1 through P7
    // CLRS: P1 = STRASSEN(A11, S1)
    let p1 = strassen_matrix_multiply(&a11, &s1);
    // CLRS: P2 = STRASSEN(S2, B22)
    let p2 = strassen_matrix_multiply(&s2, &b22);
    // CLRS: P3 = STRASSEN(S3, B11)
    let p3 = strassen_matrix_multiply(&s3, &b11);
    // CLRS: P4 = STRASSEN(A22, S4)
    let p4 = strassen_matrix_multiply(&a22, &s4);
    // CLRS: P5 = STRASSEN(S5, S6)
    let p5 = strassen_matrix_multiply(&s5, &s6);
    // CLRS: P6 = STRASSEN(S7, S8)
    let p6 = strassen_matrix_multiply(&s7, &s8);
    // CLRS: P7 = STRASSEN(S9, S10)
    let p7 = strassen_matrix_multiply(&s9, &s10);

    // Compute the four quadrants of C
    // CLRS: C11 = P5 + P4 - P2 + P6
    let c11 = add_matrices_full(&add_matrices_full(&p5, &p4), &subtract_matrices(&p6, &p2));

    // CLRS: C12 = P1 + P2
    let c12 = add_matrices_full(&p1, &p2);

    // CLRS: C21 = P3 + P4
    let c21 = add_matrices_full(&p3, &p4);

    // CLRS: C22 = P5 + P1 - P3 - P7
    let c22 = subtract_matrices(&subtract_matrices(&add_matrices_full(&p5, &p1), &p3), &p7);

    // Combine the four quadrants
    combine_matrices(&c11, &c12, &c21, &c22, n)
}

/// Extracts a submatrix from a matrix
fn extract_submatrix(
    matrix: &[Vec<i64>],
    start_row: usize,
    start_col: usize,
    size: usize,
) -> Vec<Vec<i64>> {
    let mut submatrix = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            submatrix[i][j] = matrix[start_row + i][start_col + j];
        }
    }
    submatrix
}

/// Adds two matrices of the same size
fn add_matrices_full(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[i][j] + b[i][j];
        }
    }
    result
}

/// Subtracts matrix b from matrix a (a - b)
fn subtract_matrices(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[i][j] - b[i][j];
        }
    }
    result
}

/// Combines four submatrices into a single matrix
fn combine_matrices(
    c11: &[Vec<i64>],
    c12: &[Vec<i64>],
    c21: &[Vec<i64>],
    c22: &[Vec<i64>],
    n: usize,
) -> Vec<Vec<i64>> {
    let half = n / 2;
    let mut c = vec![vec![0; n]; n];

    for i in 0..half {
        for j in 0..half {
            c[i][j] = c11[i][j];
            c[i][j + half] = c12[i][j];
            c[i + half][j] = c21[i][j];
            c[i + half][j + half] = c22[i][j];
        }
    }

    c
}

/// Standard iterative matrix multiplication
///
/// This is the naive O(n³) algorithm for comparison.
///
/// # Arguments
/// * `a` - First matrix (n×m)
/// * `b` - Second matrix (m×p)
///
/// # Returns
/// Resulting n×p matrix C = A × B
///
/// # Panics
/// Panics if the number of columns in A doesn't match the number of rows in B.
///
/// # Complexity
/// - Time: O(nmp) = O(n³) for square matrices
/// - Space: O(np)
pub fn standard_matrix_multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let m = a[0].len();
    let p = b[0].len();

    assert_eq!(
        m,
        b.len(),
        "Number of columns in A must equal number of rows in B"
    );

    let mut c = vec![vec![0; p]; n];

    for i in 0..n {
        for j in 0..p {
            for k in 0..m {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_matrix_multiply() {
        let a = vec![vec![1, 3], vec![7, 5]];
        let b = vec![vec![6, 8], vec![4, 2]];
        let c = standard_matrix_multiply(&a, &b);
        // 1*6 + 3*4 = 6 + 12 = 18
        // 1*8 + 3*2 = 8 + 6 = 14
        // 7*6 + 5*4 = 42 + 20 = 62
        // 7*8 + 5*2 = 56 + 10 = 66
        assert_eq!(c, vec![vec![18, 14], vec![62, 66]]);
    }

    #[test]
    fn test_strassen_example_from_clrs() {
        // Example from CLRS Section 4.2, Exercise 4.2-1
        let a = vec![vec![1, 3], vec![7, 5]];
        let b = vec![vec![6, 8], vec![4, 2]];
        let c = strassen_matrix_multiply(&a, &b);
        assert_eq!(c, vec![vec![18, 14], vec![62, 66]]);
    }

    #[test]
    fn test_strassen_4x4() {
        let a = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let b = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let c_strassen = strassen_matrix_multiply(&a, &b);
        let c_standard = standard_matrix_multiply(&a, &b);
        assert_eq!(c_strassen, c_standard);
    }

    #[test]
    fn test_strassen_vs_standard() {
        // Test that Strassen's algorithm produces the same result as standard multiplication
        let a = vec![
            vec![1, 3, 5, 7],
            vec![2, 4, 6, 8],
            vec![9, 11, 13, 15],
            vec![10, 12, 14, 16],
        ];
        let b = vec![
            vec![2, 4, 6, 8],
            vec![1, 3, 5, 7],
            vec![10, 12, 14, 16],
            vec![9, 11, 13, 15],
        ];
        let c_strassen = strassen_matrix_multiply(&a, &b);
        let c_standard = standard_matrix_multiply(&a, &b);
        assert_eq!(c_strassen, c_standard);
    }

    #[test]
    fn test_strassen_identity() {
        let identity = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let a = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let c = strassen_matrix_multiply(&a, &identity);
        assert_eq!(c, a);
    }

    #[test]
    fn test_strassen_single_element() {
        let a = vec![vec![5]];
        let b = vec![vec![7]];
        let c = strassen_matrix_multiply(&a, &b);
        assert_eq!(c, vec![vec![35]]);
    }
}
