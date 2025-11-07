//! Real-World Algorithm Examples (Section 1.1)
//!
//! This module demonstrates real-world applications of algorithms,
//! illustrating concepts from Chapter 1 Section 1.1.

/// Real-world example: Restaurant price sorting
///
/// Demonstrates sorting application from CLRS Exercise 1.1-1
///
/// # Arguments
/// * `restaurants` - Vector of (name, price) tuples
///
/// # Returns
/// Sorted restaurants by price (ascending)
///
/// # Example
/// ```
/// use clrs::chapter_01::sort_restaurants_by_price;
/// let restaurants = vec![
///     ("Restaurant A", 25.0),
///     ("Restaurant B", 15.0),
///     ("Restaurant C", 30.0),
/// ];
/// let sorted = sort_restaurants_by_price(restaurants);
/// assert_eq!(sorted[0].1, 15.0);
/// ```
pub fn sort_restaurants_by_price(mut restaurants: Vec<(&str, f64)>) -> Vec<(&str, f64)> {
    restaurants.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    restaurants
}

/// Real-world example: Finding GCD (exact solution required)
///
/// Demonstrates "best solution required" from CLRS Exercise 1.1-5
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// Greatest Common Divisor
///
/// # Example
/// ```
/// use clrs::chapter_01::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(17, 19), 1);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Data structure: Simple linked list implementation
///
/// Demonstrates strengths and limitations from CLRS Exercise 1.1-3
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    /// Insert at the beginning - O(1) operation (strength)
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Get element at index - O(n) operation (limitation)
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current.and_then(|n| n.next.as_ref());
        }
        current.map(|n| &n.data)
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Demonstrates shortest path vs traveling salesman problem
///
/// Both find paths, but TSP has additional constraints (visit all nodes exactly once)
#[derive(Debug, Clone)]
pub struct PathProblem {
    pub nodes: Vec<(f64, f64)>, // (x, y) coordinates
}

impl PathProblem {
    pub fn new(nodes: Vec<(f64, f64)>) -> Self {
        PathProblem { nodes }
    }

    /// Compute distance between two nodes
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        let (x1, y1) = self.nodes[i];
        let (x2, y2) = self.nodes[j];
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    /// Shortest path from start to end (simple problem)
    pub fn shortest_path(&self, start: usize, end: usize) -> f64 {
        // Simplified: just return direct distance
        // In practice, would use Dijkstra's algorithm
        self.distance(start, end)
    }

    /// Traveling salesman: visit all nodes exactly once and return to start
    /// This is much harder due to the "visit all exactly once" constraint
    pub fn tsp_brute_force(&self) -> Option<f64> {
        if self.nodes.len() > 10 {
            // Too many combinations for brute force
            return None;
        }

        // Brute force: try all permutations
        let mut min_distance = f64::INFINITY;
        let mut indices: Vec<usize> = (1..self.nodes.len()).collect();

        // Generate all permutations and find minimum
        self.permute_and_evaluate(&mut indices, 0, &mut min_distance);

        if min_distance.is_finite() {
            Some(min_distance)
        } else {
            None
        }
    }

    fn permute_and_evaluate(&self, arr: &mut [usize], depth: usize, min_distance: &mut f64) {
        if depth == arr.len() {
            // Calculate distance for this permutation
            let mut dist = self.distance(0, arr[0]); // Start to first
            for i in 0..arr.len() - 1 {
                dist += self.distance(arr[i], arr[i + 1]);
            }
            dist += self.distance(arr[arr.len() - 1], 0); // Last to start
            *min_distance = (*min_distance).min(dist);
            return;
        }

        for i in depth..arr.len() {
            arr.swap(depth, i);
            self.permute_and_evaluate(arr, depth + 1, min_distance);
            arr.swap(depth, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_restaurants() {
        let restaurants = vec![("Expensive", 50.0), ("Cheap", 10.0), ("Medium", 25.0)];
        let sorted = sort_restaurants_by_price(restaurants);
        assert_eq!(sorted[0].1, 10.0);
        assert_eq!(sorted[2].1, 50.0);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(100, 25), 25);
    }

    #[test]
    fn test_linked_list() {
        let mut list = SimpleLinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
    }

    #[test]
    fn test_path_problem() {
        let nodes = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
        let problem = PathProblem::new(nodes);

        let dist = problem.shortest_path(0, 1);
        assert!((dist - 1.0).abs() < 0.001);

        // TSP should find a tour
        let tsp_dist = problem.tsp_brute_force();
        assert!(tsp_dist.is_some());
        assert!(tsp_dist.unwrap() > 0.0);
    }
}
