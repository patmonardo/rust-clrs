//! Fractional Knapsack Problem (Section 16.2)
//!
//! Given items with weights and values, fill a knapsack of capacity W to
//! maximize total value. Items can be taken fractionally.

/// Represents an item with weight and value
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: f64,
    pub value: f64,
}

impl Item {
    /// Creates a new item
    ///
    /// # Arguments
    /// * `weight` - Weight of the item
    /// * `value` - Value of the item
    pub fn new(weight: f64, value: f64) -> Self {
        Item { weight, value }
    }

    /// Returns the value per unit weight (density)
    pub fn density(&self) -> f64 {
        if self.weight == 0.0 {
            f64::INFINITY
        } else {
            self.value / self.weight
        }
    }
}

/// Solves the fractional knapsack problem using a greedy algorithm
///
/// This corresponds to the greedy algorithm from CLRS Section 16.2.
/// Items are sorted by value/weight ratio in decreasing order.
///
/// # Arguments
/// * `items` - Vector of items
/// * `capacity` - Maximum weight capacity
///
/// # Returns
/// A tuple (total_value, fractions) where:
/// - total_value is the maximum value achievable
/// - fractions[i] is the fraction of item i taken (0.0 to 1.0)
///
/// # Complexity
/// - Time: O(n log n) where n is the number of items (due to sorting)
/// - Space: O(n)
///
/// # Example
/// ```
/// use clrs::chapter_16::{Item, fractional_knapsack};
/// let items = vec![
///     Item::new(10.0, 60.0),
///     Item::new(20.0, 100.0),
///     Item::new(30.0, 120.0),
/// ];
/// let (value, fractions) = fractional_knapsack(&items, 50.0);
/// assert!(value > 0.0);
/// ```
pub fn fractional_knapsack(items: &[Item], capacity: f64) -> (f64, Vec<f64>) {
    let n = items.len();
    let mut indices: Vec<usize> = (0..n).collect();

    // Sort by value/weight ratio in decreasing order
    indices.sort_by(|&i, &j| items[j].density().partial_cmp(&items[i].density()).unwrap());

    let mut remaining = capacity;
    let mut total_value = 0.0;
    let mut fractions = vec![0.0; n];

    for &idx in &indices {
        if remaining <= 0.0 {
            break;
        }

        let item = items[idx];
        let fraction = (remaining / item.weight).min(1.0);
        fractions[idx] = fraction;
        total_value += item.value * fraction;
        remaining -= item.weight * fraction;
    }

    (total_value, fractions)
}

/// Solves the 0-1 knapsack problem using dynamic programming
///
/// This corresponds to 0-1-KNAPSACK from CLRS Exercise 16.2-2.
///
/// # Arguments
/// * `items` - Vector of items (each item can be taken at most once)
/// * `capacity` - Maximum weight capacity (must be non-negative integer)
///
/// # Returns
/// A tuple (max_value, selected) where:
/// - max_value is the maximum value achievable
/// - selected[i] is true if item i is selected
///
/// # Complexity
/// - Time: O(nW) where n is the number of items and W is capacity
/// - Space: O(nW)
pub fn knapsack_01(items: &[Item], capacity: usize) -> (f64, Vec<bool>) {
    let n = items.len();
    let w = capacity;

    // K[i][j] = maximum value using first i items with capacity j
    let mut k = vec![vec![0.0; w + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=w {
            let weight = items[i - 1].weight as usize;
            if weight > j {
                k[i][j] = k[i - 1][j];
            } else {
                let without = k[i - 1][j];
                let with = k[i - 1][j - weight] + items[i - 1].value;
                k[i][j] = if without > with { without } else { with };
            }
        }
    }

    // Reconstruct the solution
    let mut selected = vec![false; n];
    let mut j = w;
    for i in (1..=n).rev() {
        let weight = items[i - 1].weight as usize;
        if j >= weight && k[i][j] == k[i - 1][j - weight] + items[i - 1].value {
            selected[i - 1] = true;
            j -= weight;
        }
    }

    (k[n][w], selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractional_knapsack() {
        let items = vec![
            Item::new(10.0, 60.0),
            Item::new(20.0, 100.0),
            Item::new(30.0, 120.0),
        ];
        let (value, fractions) = fractional_knapsack(&items, 50.0);

        // Should take all of item 0 (10), all of item 1 (20), and 2/3 of item 2 (20)
        // Total value: 60 + 100 + 120 * (2/3) = 60 + 100 + 80 = 240
        assert!((value - 240.0).abs() < 0.01);
        assert_eq!(fractions[0], 1.0);
        assert_eq!(fractions[1], 1.0);
        assert!((fractions[2] - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_fractional_knapsack_empty() {
        let items = vec![];
        let (value, fractions) = fractional_knapsack(&items, 50.0);
        assert_eq!(value, 0.0);
        assert!(fractions.is_empty());
    }

    #[test]
    fn test_knapsack_01() {
        let items = vec![
            Item::new(10.0, 60.0),
            Item::new(20.0, 100.0),
            Item::new(30.0, 120.0),
        ];
        let (value, _selected) = knapsack_01(&items, 50);

        // Should select items 0 and 1 (total weight 30, value 160)
        // or items 1 and 2 (total weight 50, value 220)
        assert!(value >= 160.0);
        assert!(value <= 220.0);
    }
}
