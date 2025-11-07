use std::fmt;

/// Weighted adjacency-matrix representation for all-pairs algorithms.
#[derive(Clone, PartialEq, Eq)]
pub struct MatrixGraph<W> {
    weights: Vec<Vec<Option<W>>>,
}

impl<W> MatrixGraph<W>
where
    W: Copy + Default,
{
    /// Creates a new graph with `vertex_count` vertices and zero-weight self loops.
    pub fn new(vertex_count: usize) -> Self {
        let mut weights = vec![vec![None; vertex_count]; vertex_count];
        for i in 0..vertex_count {
            weights[i][i] = Some(W::default());
        }
        Self { weights }
    }

    /// Returns the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.weights.len()
    }

    /// Sets the directed edge `(u, v)` to the provided weight.
    ///
    /// # Panics
    ///
    /// Panics if `u` or `v` are out of bounds.
    pub fn set_edge(&mut self, u: usize, v: usize, weight: W) {
        assert!(u < self.vertex_count(), "vertex {} out of bounds", u);
        assert!(v < self.vertex_count(), "vertex {} out of bounds", v);
        self.weights[u][v] = Some(weight);
    }

    /// Returns a reference to the weight matrix.
    pub fn weights(&self) -> &[Vec<Option<W>>] {
        &self.weights
    }
}

impl<W> fmt::Debug for MatrixGraph<W>
where
    W: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MatrixGraph")
            .field("weights", &self.weights)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_edge_updates_matrix() {
        let mut graph: MatrixGraph<i32> = MatrixGraph::new(3);
        graph.set_edge(0, 1, 5);
        graph.set_edge(1, 2, -2);

        assert_eq!(
            graph.weights(),
            &[
                vec![Some(0), Some(5), None],
                vec![None, Some(0), Some(-2)],
                vec![None, None, Some(0)],
            ]
        );
    }
}

