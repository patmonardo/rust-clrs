use std::fmt;

/// Edge included in an MST solution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MstEdge<W> {
    pub u: usize,
    pub v: usize,
    pub weight: W,
}

/// Result of running an MST algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MstResult<W> {
    pub edges: Vec<MstEdge<W>>,
    pub total_weight: W,
}

/// An undirected, weighted graph represented by adjacency lists.
///
/// The graph stores symmetric edges; each call to [`WeightedGraph::add_edge`]
/// inserts both `(u, v)` and `(v, u)` entries.
#[derive(Clone, PartialEq, Eq)]
pub struct WeightedGraph<W> {
    adjacency_list: Vec<Vec<(usize, W)>>,
}

impl<W> WeightedGraph<W>
where
    W: Copy,
{
    /// Creates a new weighted graph with the given number of vertices.
    pub fn new(vertex_count: usize) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); vertex_count],
        }
    }

    /// Adds an undirected edge between `u` and `v` with the specified weight.
    ///
    /// # Panics
    ///
    /// Panics if `u` or `v` are out of bounds.
    pub fn add_edge(&mut self, u: usize, v: usize, weight: W) {
        assert!(u < self.vertex_count(), "vertex {} out of bounds", u);
        assert!(v < self.vertex_count(), "vertex {} out of bounds", v);
        self.adjacency_list[u].push((v, weight));
        if u != v {
            self.adjacency_list[v].push((u, weight));
        }
    }

    /// Returns the number of vertices in the graph.
    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Provides an iterator over the neighbors of `u`.
    pub fn neighbors(&self, u: usize) -> impl Iterator<Item = (usize, W)> + '_ {
        self.adjacency_list[u].iter().copied()
    }

    /// Collects all edges `(u, v, weight)` with `u < v` for algorithms that
    /// operate over edge sets (e.g., Kruskal).
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        let mut result = Vec::new();
        for (u, neighbors) in self.adjacency_list.iter().enumerate() {
            for &(v, weight) in neighbors {
                if u < v {
                    result.push((u, v, weight));
                }
            }
        }
        result
    }
}

impl<W> fmt::Debug for WeightedGraph<W>
where
    W: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeightedGraph")
            .field("adjacency_list", &self.adjacency_list)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge_symmetry() {
        let mut graph = WeightedGraph::new(3);
        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 2, 7);

        let mut edges = graph.edges();
        edges.sort_unstable();
        assert_eq!(edges, vec![(0, 1, 4), (0, 2, 7)]);

        let neighbors_0: Vec<_> = graph.neighbors(0).collect();
        assert_eq!(neighbors_0, vec![(1, 4), (2, 7)]);

        let neighbors_1: Vec<_> = graph.neighbors(1).collect();
        assert_eq!(neighbors_1, vec![(0, 4)]);
    }
}
