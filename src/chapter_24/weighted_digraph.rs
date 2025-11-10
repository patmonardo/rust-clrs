use std::fmt;
use std::ops::{Add, Sub};

/// A weighted directed graph represented by adjacency lists.
///
/// Each vertex is identified by a `usize` index, and edges are stored as
/// `(target, weight)` pairs.
#[derive(Clone, PartialEq, Eq)]
pub struct WeightedDigraph<W> {
    adjacency_list: Vec<Vec<(usize, W)>>,
}

impl<W> WeightedDigraph<W>
where
    W: Copy,
{
    /// Creates a new directed graph with `vertex_count` vertices.
    pub fn new(vertex_count: usize) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); vertex_count],
        }
    }

    /// Returns the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Adds a directed edge `(u, v)` with the given `weight`.
    ///
    /// # Panics
    ///
    /// Panics if `u` or `v` are out of bounds.
    pub fn add_edge(&mut self, u: usize, v: usize, weight: W) {
        assert!(u < self.vertex_count(), "vertex {} out of bounds", u);
        assert!(v < self.vertex_count(), "vertex {} out of bounds", v);
        self.adjacency_list[u].push((v, weight));
    }

    /// Returns an iterator over the outgoing edges from `u`.
    pub fn neighbors(&self, u: usize) -> impl Iterator<Item = (usize, W)> + '_ {
        self.adjacency_list[u].iter().copied()
    }

    /// Collects all edges `(u, v, weight)` in the graph.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        let mut edges = Vec::new();
        for (u, neighbors) in self.adjacency_list.iter().enumerate() {
            for &(v, weight) in neighbors {
                edges.push((u, v, weight));
            }
        }
        edges
    }
}

impl<W> WeightedDigraph<W>
where
    W: Copy + Add<Output = W> + Sub<Output = W>,
{
    /// Applies Johnson-style reweighting given vertex potentials `h`.
    ///
    /// The returned graph has edge weights `w'(u, v) = w(u, v) + h[u] - h[v]`.
    pub fn reweight(&self, potentials: &[W]) -> Self {
        assert_eq!(
            potentials.len(),
            self.vertex_count(),
            "potentials must match vertex count"
        );

        let mut reweighted = WeightedDigraph::new(self.vertex_count());
        for (u, neighbors) in self.adjacency_list.iter().enumerate() {
            for &(v, weight) in neighbors {
                let new_weight = weight + potentials[u] - potentials[v];
                reweighted.add_edge(u, v, new_weight);
            }
        }
        reweighted
    }
}

impl<W> fmt::Debug for WeightedDigraph<W>
where
    W: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeightedDigraph")
            .field("adjacency_list", &self.adjacency_list)
            .finish()
    }
}

/// Result of running a shortest-path algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortestPathResult<W> {
    pub source: usize,
    pub distances: Vec<Option<W>>,
    pub predecessors: Vec<Option<usize>>,
}

impl<W> ShortestPathResult<W>
where
    W: Copy,
{
    /// Reconstructs the shortest path from the source to `target`, if it exists.
    pub fn path_to(&self, target: usize) -> Option<Vec<usize>> {
        if self.distances.get(target)?.is_none() {
            return None;
        }
        let mut path = Vec::new();
        let mut current = Some(target);
        while let Some(node) = current {
            path.push(node);
            if node == self.source {
                path.reverse();
                return Some(path);
            }
            current = self.predecessors[node];
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge_and_neighbors() {
        let mut graph = WeightedDigraph::new(3);
        graph.add_edge(0, 1, 5);
        graph.add_edge(0, 2, 2);
        graph.add_edge(1, 2, 1);

        let outgoing: Vec<_> = graph.neighbors(0).collect();
        assert_eq!(outgoing, vec![(1, 5), (2, 2)]);

        let edges = graph.edges();
        assert_eq!(edges, vec![(0, 1, 5), (0, 2, 2), (1, 2, 1)]);
    }

    #[test]
    fn reweight_applies_potentials() {
        let mut graph = WeightedDigraph::new(2);
        graph.add_edge(0, 1, 3);
        graph.add_edge(1, 0, 1);

        let reweighted = graph.reweight(&[1, 4]);
        assert_eq!(reweighted.edges(), vec![(0, 1, 0), (1, 0, 4)]);
    }
}
