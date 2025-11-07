//! Rod Cutting Problem (Section 15.1)
//!
//! Given a rod of length n and a table of prices p_i for rods of length i,
//! determine the maximum revenue obtainable by cutting up the rod and selling
//! the pieces.

/// Solves the rod-cutting problem using a bottom-up approach
///
/// This corresponds to BOTTOM-UP-CUT-ROD from CLRS Section 15.1.
///
/// # Arguments
/// * `p` - Price table where p[i] is the price for a rod of length i (1-indexed)
/// * `n` - Length of the rod
///
/// # Returns
/// The maximum revenue obtainable
///
/// # Complexity
/// - Time: O(n²)
/// - Space: O(n)
///
/// # Example
/// ```
/// use clrs::chapter_15::bottom_up_cut_rod;
/// let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
/// assert_eq!(bottom_up_cut_rod(&prices, 4), 10);
/// ```
pub fn bottom_up_cut_rod(p: &[i32], n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut r = vec![0; n + 1];

    for j in 1..=n {
        let mut q = i32::MIN;
        for i in 1..=j {
            if i < p.len() {
                q = q.max(p[i] + r[j - i]);
            }
        }
        r[j] = q;
    }

    r[n]
}

/// Solves the rod-cutting problem using memoization (top-down)
///
/// This corresponds to MEMOIZED-CUT-ROD from CLRS Section 15.1.
///
/// # Arguments
/// * `p` - Price table where p[i] is the price for a rod of length i (1-indexed)
/// * `n` - Length of the rod
///
/// # Returns
/// The maximum revenue obtainable
///
/// # Complexity
/// - Time: O(n²)
/// - Space: O(n)
pub fn memoized_cut_rod(p: &[i32], n: usize) -> i32 {
    let mut r = vec![i32::MIN; n + 1];
    memoized_cut_rod_aux(p, n, &mut r)
}

fn memoized_cut_rod_aux(p: &[i32], n: usize, r: &mut [i32]) -> i32 {
    if r[n] >= 0 {
        return r[n];
    }

    let q = if n == 0 {
        0
    } else {
        let mut max_revenue = i32::MIN;
        for i in 1..=n {
            if i < p.len() {
                max_revenue = max_revenue.max(p[i] + memoized_cut_rod_aux(p, n - i, r));
            }
        }
        max_revenue
    };

    r[n] = q;
    q
}

/// Solves the rod-cutting problem and returns the optimal solution
///
/// This corresponds to EXTENDED-BOTTOM-UP-CUT-ROD from CLRS Section 15.1.
///
/// # Arguments
/// * `p` - Price table where p[i] is the price for a rod of length i (1-indexed)
/// * `n` - Length of the rod
///
/// # Returns
/// A tuple (max_revenue, solution) where solution[i] is the optimal first cut
/// for a rod of length i
///
/// # Complexity
/// - Time: O(n²)
/// - Space: O(n)
pub fn extended_bottom_up_cut_rod(p: &[i32], n: usize) -> (i32, Vec<usize>) {
    let mut r = vec![0; n + 1];
    let mut s = vec![0; n + 1];

    for j in 1..=n {
        let mut q = i32::MIN;
        for i in 1..=j {
            if i < p.len() {
                let revenue = p[i] + r[j - i];
                if q < revenue {
                    q = revenue;
                    s[j] = i;
                }
            }
        }
        r[j] = q;
    }

    (r[n], s)
}

/// Prints the optimal solution to the rod-cutting problem
///
/// This corresponds to PRINT-CUT-ROD-SOLUTION from CLRS Section 15.1.
///
/// # Arguments
/// * `p` - Price table where p[i] is the price for a rod of length i (1-indexed)
/// * `n` - Length of the rod
///
/// # Returns
/// A vector of cut lengths that form the optimal solution
pub fn print_cut_rod_solution(p: &[i32], n: usize) -> Vec<usize> {
    let (_, s) = extended_bottom_up_cut_rod(p, n);
    let mut solution = Vec::new();
    let mut j = n;

    while j > 0 {
        solution.push(s[j]);
        j -= s[j];
    }

    solution
}

/// Solves the rod-cutting problem with a fixed cost per cut
///
/// This corresponds to MODIFIED-CUT-ROD from CLRS Exercise 15.1-3.
///
/// # Arguments
/// * `p` - Price table where p[i] is the price for a rod of length i (1-indexed)
/// * `n` - Length of the rod
/// * `c` - Fixed cost per cut
///
/// # Returns
/// The maximum revenue obtainable (revenue minus cut costs)
///
/// # Complexity
/// - Time: O(n²)
/// - Space: O(n)
pub fn modified_cut_rod(p: &[i32], n: usize, c: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut r = vec![0; n + 1];

    for j in 1..=n {
        let mut q = if j < p.len() { p[j] } else { i32::MIN };
        for i in 1..j {
            if i < p.len() {
                q = q.max(p[i] + r[j - i] - c);
            }
        }
        r[j] = q;
    }

    r[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_up_cut_rod() {
        let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        assert_eq!(bottom_up_cut_rod(&prices, 0), 0);
        assert_eq!(bottom_up_cut_rod(&prices, 1), 1);
        assert_eq!(bottom_up_cut_rod(&prices, 4), 10);
        assert_eq!(bottom_up_cut_rod(&prices, 7), 18);
    }

    #[test]
    fn test_memoized_cut_rod() {
        let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        assert_eq!(memoized_cut_rod(&prices, 0), 0);
        assert_eq!(memoized_cut_rod(&prices, 1), 1);
        assert_eq!(memoized_cut_rod(&prices, 4), 10);
        assert_eq!(memoized_cut_rod(&prices, 7), 18);
    }

    #[test]
    fn test_extended_bottom_up_cut_rod() {
        let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let (revenue, s) = extended_bottom_up_cut_rod(&prices, 7);
        assert_eq!(revenue, 18);
        assert_eq!(s[7], 1); // Optimal first cut for length 7
    }

    #[test]
    fn test_print_cut_rod_solution() {
        let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let solution = print_cut_rod_solution(&prices, 7);
        // Solution should sum to 7
        assert_eq!(solution.iter().sum::<usize>(), 7);
    }

    #[test]
    fn test_modified_cut_rod() {
        let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let revenue = modified_cut_rod(&prices, 4, 1);
        // With cost=1, cutting might not always be optimal
        assert!(revenue >= 0);
    }
}
