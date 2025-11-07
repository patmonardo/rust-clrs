//! Matrix-Chain Multiplication (Section 15.2)
//!
//! Given a sequence of matrices, determine the most efficient way to multiply
//! them together. The problem is not actually to perform the multiplications,
//! but merely to decide the sequence of the matrix multiplications involved.

/// Solves the matrix-chain multiplication problem
///
/// This corresponds to MATRIX-CHAIN-ORDER from CLRS Section 15.2.
///
/// # Arguments
/// * `p` - Dimensions array where matrix A_i has dimensions p[i-1] × p[i]
///
/// # Returns
/// A tuple (m, s) where:
/// - m[i][j] is the minimum number of scalar multiplications needed to compute A_i...A_j
/// - s[i][j] is the index k at which to split the product A_i...A_j
///
/// # Complexity
/// - Time: O(n³)
/// - Space: O(n²)
///
/// # Example
/// ```
/// use clrs::chapter_15::matrix_chain_order;
/// let dims = vec![1, 2, 3, 4, 5];
/// let (m, s) = matrix_chain_order(&dims);
/// assert_eq!(m[1][4], 38); // Minimum multiplications for A_1...A_4
/// ```
pub fn matrix_chain_order(p: &[usize]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = p.len() - 1;
    let mut m = vec![vec![0; n + 1]; n + 1];
    let mut s = vec![vec![0; n + 1]; n + 1];
    
    for l in 2..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            m[i][j] = usize::MAX;
            
            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + p[i - 1] * p[k] * p[j];
                if q < m[i][j] {
                    m[i][j] = q;
                    s[i][j] = k;
                }
            }
        }
    }
    
    (m, s)
}

/// Prints the optimal parenthesization
///
/// This corresponds to PRINT-OPTIMAL-PARENS from CLRS Section 15.2.
///
/// # Arguments
/// * `s` - The split table from matrix_chain_order
/// * `i` - Start index (1-indexed)
/// * `j` - End index (1-indexed)
///
/// # Returns
/// A string representation of the optimal parenthesization
pub fn print_optimal_parens(s: &[Vec<usize>], i: usize, j: usize) -> String {
    if i == j {
        format!("A{}", i)
    } else {
        let k = s[i][j];
        format!(
            "({}{})",
            print_optimal_parens(s, i, k),
            print_optimal_parens(s, k + 1, j)
        )
    }
}

/// Recursive solution to matrix-chain multiplication (inefficient)
///
/// This corresponds to RECURSIVE-MATRIX-CHAIN from CLRS Section 15.2.
///
/// # Arguments
/// * `p` - Dimensions array where matrix A_i has dimensions p[i-1] × p[i]
/// * `i` - Start index (1-indexed)
/// * `j` - End index (1-indexed)
///
/// # Returns
/// The minimum number of scalar multiplications needed
///
/// # Complexity
/// - Time: Ω(2^n) (exponential)
/// - Space: O(n)
pub fn recursive_matrix_chain(p: &[usize], i: usize, j: usize) -> usize {
    if i == j {
        return 0;
    }
    
    let mut min_cost = usize::MAX;
    for k in i..j {
        let cost = recursive_matrix_chain(p, i, k)
            + recursive_matrix_chain(p, k + 1, j)
            + p[i - 1] * p[k] * p[j];
        min_cost = min_cost.min(cost);
    }
    
    min_cost
}

/// Memoized version of recursive matrix-chain multiplication
///
/// # Arguments
/// * `p` - Dimensions array where matrix A_i has dimensions p[i-1] × p[i]
///
/// # Returns
/// The minimum number of scalar multiplications needed
///
/// # Complexity
/// - Time: O(n³)
/// - Space: O(n²)
pub fn memoized_matrix_chain(p: &[usize]) -> usize {
    let n = p.len() - 1;
    let mut m = vec![vec![None; n + 1]; n + 1];
    lookup_chain(p, &mut m, 1, n)
}

fn lookup_chain(
    p: &[usize],
    m: &mut [Vec<Option<usize>>],
    i: usize,
    j: usize,
) -> usize {
    if let Some(cost) = m[i][j] {
        return cost;
    }
    
    let cost = if i == j {
        0
    } else {
        let mut min_cost = usize::MAX;
        for k in i..j {
            let cost = lookup_chain(p, m, i, k)
                + lookup_chain(p, m, k + 1, j)
                + p[i - 1] * p[k] * p[j];
            min_cost = min_cost.min(cost);
        }
        min_cost
    };
    
    m[i][j] = Some(cost);
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_chain_order() {
        // Example from CLRS: A_1 (30×35), A_2 (35×15), A_3 (15×5), A_4 (5×10)
        let dims = vec![30, 35, 15, 5, 10];
        let (m, s) = matrix_chain_order(&dims);
        
        // Minimum cost for A_1...A_4 should be 9375
        // Optimal: (A_1 (A_2 A_3)) A_4, split at k=3
        assert_eq!(m[1][4], 9375);
        assert_eq!(s[1][4], 3); // Split at k=3: (A_1...A_3) A_4
    }

    #[test]
    fn test_print_optimal_parens() {
        let dims = vec![30, 35, 15, 5, 10];
        let (_, s) = matrix_chain_order(&dims);
        let parens = print_optimal_parens(&s, 1, 4);
        // Should be ((A1(A2A3))A4) or similar
        assert!(parens.contains("A1"));
        assert!(parens.contains("A4"));
    }

    #[test]
    fn test_recursive_matrix_chain() {
        let dims = vec![30, 35, 15, 5, 10];
        let cost = recursive_matrix_chain(&dims, 1, 4);
        assert_eq!(cost, 9375);
    }

    #[test]
    fn test_memoized_matrix_chain() {
        let dims = vec![30, 35, 15, 5, 10];
        let cost = memoized_matrix_chain(&dims);
        assert_eq!(cost, 9375);
    }
}

