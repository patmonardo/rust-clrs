use std::fmt;

/// An adjacency-list representation of a graph.
///
/// The vertices are identified by indices in `0..vertex_count`. Edges are
/// stored as adjacency lists, and the graph can be either directed or
/// undirected.
#[derive(Clone, PartialEq, Eq)]
pub struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    directed: bool,
}

impl Graph {
    /// Creates a new graph with the given number of vertices.
    ///
    /// When `directed` is `false`, edges added via [`Graph::add_edge`] will be
    /// mirrored to maintain an undirected graph.
    pub fn new(vertex_count: usize, directed: bool) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); vertex_count],
            directed,
        }
    }

    /// Constructs a graph from an adjacency list. The graph is assumed to be
    /// directed when `directed` is `true`; otherwise, it is treated as
    /// undirected.
    pub fn from_adjacency_list(adjacency_list: Vec<Vec<usize>>, directed: bool) -> Self {
        Self {
            adjacency_list,
            directed,
        }
    }

    /// Returns the number of vertices in the graph.
    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Returns whether this graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Adds an edge `(u, v)` to the graph. When the graph is undirected, the
    /// reciprocal edge `(v, u)` is also inserted.
    ///
    /// # Panics
    ///
    /// Panics if either `u` or `v` is not a valid vertex index.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.vertex_count(), "vertex {} out of bounds", u);
        assert!(v < self.vertex_count(), "vertex {} out of bounds", v);
        self.adjacency_list[u].push(v);
        if !self.directed && u != v {
            self.adjacency_list[v].push(u);
        }
    }

    /// Returns the neighbors of vertex `u`.
    ///
    /// # Panics
    ///
    /// Panics if `u` is not a valid vertex index.
    pub fn neighbors(&self, u: usize) -> &[usize] {
        &self.adjacency_list[u]
    }

    /// Returns an iterator over the neighbors of `u`.
    pub fn neighbors_iter(&self, u: usize) -> impl Iterator<Item = usize> + '_ {
        self.adjacency_list[u].iter().copied()
    }

    /// Produces the transpose of a directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the graph is undirected. The transpose is only defined for
    /// directed graphs.
    pub fn transpose(&self) -> Self {
        assert!(
            self.directed,
            "transpose is only defined for directed graphs"
        );
        let mut transposed = vec![Vec::new(); self.vertex_count()];
        for (u, neighbors) in self.adjacency_list.iter().enumerate() {
            for &v in neighbors {
                transposed[v].push(u);
            }
        }
        Self {
            adjacency_list: transposed,
            directed: true,
        }
    }

    /// Consumes the graph and returns the underlying adjacency list.
    pub fn into_adjacency_list(self) -> Vec<Vec<usize>> {
        self.adjacency_list
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Graph")
            .field("directed", &self.directed)
            .field("adjacency_list", &self.adjacency_list)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge_directed() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(2, 1);

        assert_eq!(g.into_adjacency_list(), vec![vec![1, 2], vec![], vec![1]]);
    }

    #[test]
    fn add_edge_undirected() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        assert_eq!(g.into_adjacency_list(), vec![vec![1], vec![0, 2], vec![1]]);
    }

    #[test]
    #[should_panic]
    fn transpose_panics_for_undirected() {
        let g = Graph::new(2, false);
        let _ = g.transpose();
    }
}
