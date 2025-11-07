//! Optimal Binary Search Trees (Section 15.5)
//!
//! Given a set of keys and their access probabilities, construct a binary
//! search tree that minimizes the expected search cost.

/// Computes the cost and structure of an optimal binary search tree
///
/// This corresponds to OPTIMAL-BST from CLRS Section 15.5.
///
/// # Arguments
/// * `p` - Probability array where p[i] is the probability of searching for key k_i (1-indexed)
/// * `q` - Probability array where q[i] is the probability of searching for dummy key d_i (0-indexed)
/// * `n` - Number of keys
///
/// # Returns
/// A tuple (e, root) where:
/// - e[i][j] is the expected cost of searching in an optimal BST containing keys k_i...k_j
/// - root[i][j] is the index of the root of an optimal BST containing keys k_i...k_j
///
/// # Complexity
/// - Time: O(n³)
/// - Space: O(n²)
///
/// # Example
/// ```
/// use clrs::chapter_15::optimal_bst;
/// let p = vec![0.0, 0.15, 0.10, 0.05, 0.10, 0.20];
/// let q = vec![0.05, 0.10, 0.05, 0.05, 0.05, 0.10];
/// let (e, root) = optimal_bst(&p, &q, 5);
/// assert!((e[1][5] - 2.75).abs() < 0.01);
/// ```
pub fn optimal_bst(p: &[f64], q: &[f64], n: usize) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    let mut e = vec![vec![0.0; n + 2]; n + 2];
    let mut w = vec![vec![0.0; n + 2]; n + 2];
    let mut root = vec![vec![0; n + 1]; n + 1];
    
    // Initialize base cases
    for i in 1..=n + 1 {
        e[i][i - 1] = q[i - 1];
        w[i][i - 1] = q[i - 1];
    }
    
    // Compute e[i][j] and root[i][j] for all i <= j
    for l in 1..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            e[i][j] = f64::INFINITY;
            w[i][j] = w[i][j - 1] + p[j] + q[j];
            
            for r in i..=j {
                let t = e[i][r - 1] + e[r + 1][j] + w[i][j];
                if t < e[i][j] {
                    e[i][j] = t;
                    root[i][j] = r;
                }
            }
        }
    }
    
    (e, root)
}

/// Constructs the structure of an optimal BST
///
/// This corresponds to CONSTRUCT-OPTIMAL-BST from CLRS Exercise 15.5-1.
///
/// # Arguments
/// * `root` - The root table from optimal_bst
/// * `i` - Start index (1-indexed)
/// * `j` - End index (1-indexed)
/// * `parent` - Parent key index (0 means root)
///
/// # Returns
/// A vector of strings describing the tree structure
pub fn construct_optimal_bst(
    root: &[Vec<usize>],
    i: usize,
    j: usize,
    parent: usize,
) -> Vec<String> {
    if i > j {
        return vec![format!("d_{} is a child of k_{}", j, parent)];
    }
    
    let r = root[i][j];
    let mut result = Vec::new();
    
    if parent == 0 {
        result.push(format!("k_{} is the root", r));
    } else if j < parent {
        result.push(format!("k_{} is the left child of k_{}", r, parent));
    } else {
        result.push(format!("k_{} is the right child of k_{}", r, parent));
    }
    
    result.extend(construct_optimal_bst(root, i, r - 1, r));
    result.extend(construct_optimal_bst(root, r + 1, j, r));
    
    result
}

/// Optimal BST with improved time complexity using Knuth's optimization
///
/// This corresponds to the optimized version from CLRS Exercise 15.5-4.
///
/// # Arguments
/// * `p` - Probability array where p[i] is the probability of searching for key k_i (1-indexed)
/// * `q` - Probability array where q[i] is the probability of searching for dummy key d_i (0-indexed)
/// * `n` - Number of keys
///
/// # Returns
/// A tuple (e, root) where:
/// - e[i][j] is the expected cost of searching in an optimal BST containing keys k_i...k_j
/// - root[i][j] is the index of the root of an optimal BST containing keys k_i...k_j
///
/// # Complexity
/// - Time: O(n²) (improved from O(n³))
/// - Space: O(n²)
pub fn optimal_bst_knuth(p: &[f64], q: &[f64], n: usize) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    let mut e = vec![vec![0.0; n + 2]; n + 2];
    let mut w = vec![vec![0.0; n + 2]; n + 2];
    let mut root = vec![vec![0; n + 1]; n + 1];
    
    // Initialize base cases
    for i in 1..=n + 1 {
        e[i][i - 1] = q[i - 1];
        w[i][i - 1] = q[i - 1];
    }
    
    // Compute e[i][j] and root[i][j] using Knuth's optimization
    for l in 1..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            e[i][j] = f64::INFINITY;
            w[i][j] = w[i][j - 1] + p[j] + q[j];
            
            // Use Knuth's optimization: root[i][j-1] <= root[i][j] <= root[i+1][j]
            let lower = if i < j { root[i][j - 1] } else { i };
            let upper = if i < j { root[i + 1][j] } else { j };
            
            for r in lower..=upper {
                let t = e[i][r - 1] + e[r + 1][j] + w[i][j];
                if t < e[i][j] {
                    e[i][j] = t;
                    root[i][j] = r;
                }
            }
        }
    }
    
    (e, root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_bst() {
        // Example from CLRS Figure 15.9
        let p = vec![0.0, 0.15, 0.10, 0.05, 0.10, 0.20];
        let q = vec![0.05, 0.10, 0.05, 0.05, 0.05, 0.10];
        let (e, root) = optimal_bst(&p, &q, 5);
        
        // Expected cost should be approximately 2.75
        assert!((e[1][5] - 2.75).abs() < 0.1);
        assert!(root[1][5] > 0);
    }

    #[test]
    fn test_construct_optimal_bst() {
        let p = vec![0.0, 0.15, 0.10, 0.05, 0.10, 0.20];
        let q = vec![0.05, 0.10, 0.05, 0.05, 0.05, 0.10];
        let (_, root) = optimal_bst(&p, &q, 5);
        let structure = construct_optimal_bst(&root, 1, 5, 0);
        
        // Should have a root
        assert!(structure.iter().any(|s| s.contains("root")));
    }

    #[test]
    fn test_optimal_bst_knuth() {
        let p = vec![0.0, 0.15, 0.10, 0.05, 0.10, 0.20];
        let q = vec![0.05, 0.10, 0.05, 0.05, 0.05, 0.10];
        let (e1, root1) = optimal_bst(&p, &q, 5);
        let (e2, root2) = optimal_bst_knuth(&p, &q, 5);
        
        // Both should give the same result
        assert!((e1[1][5] - e2[1][5]).abs() < 0.0001);
        assert_eq!(root1[1][5], root2[1][5]);
    }
}

