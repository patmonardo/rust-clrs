use std::collections::VecDeque;

use super::Graph;

/// The outcome of a breadth-first search.
#[derive(Debug, Clone)]
pub struct BfsResult {
    pub source: usize,
    pub distances: Vec<Option<usize>>,
    pub predecessors: Vec<Option<usize>>,
    pub order: Vec<usize>,
}

impl BfsResult {
    /// Reconstructs a path from the BFS source to `target`, if one exists.
    pub fn path_to(&self, target: usize) -> Option<Vec<usize>> {
        if self.distances.get(target)?.is_none() {
            return None;
        }
        let mut path = Vec::new();
        let mut current = Some(target);
        while let Some(vertex) = current {
            path.push(vertex);
            if vertex == self.source {
                path.reverse();
                return Some(path);
            }
            current = self.predecessors[vertex];
        }
        None
    }
}

/// Performs a breadth-first search from the given `source`.
///
/// The search computes the shortest-path tree for graphs with unit edge
/// weights, returning distance estimates and parent pointers for each vertex.
pub fn breadth_first_search(graph: &Graph, source: usize) -> BfsResult {
    let vertex_count = graph.vertex_count();
    assert!(source < vertex_count, "source vertex out of bounds");

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut color = vec![Color::White; vertex_count];
    let mut distances = vec![None; vertex_count];
    let mut predecessors = vec![None; vertex_count];
    let mut order = Vec::with_capacity(vertex_count);

    color[source] = Color::Gray;
    distances[source] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(u) = queue.pop_front() {
        order.push(u);
        for v in graph.neighbors_iter(u) {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                distances[v] = distances[u].map(|d| d + 1);
                predecessors[v] = Some(u);
                queue.push_back(v);
            }
        }
        color[u] = Color::Black;
    }

    BfsResult {
        source,
        distances,
        predecessors,
        order,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_directed_example() {
        // Example from CLRS Figure 22.3.
        let mut graph = Graph::new(5, true);
        graph.add_edge(0, 1); // s -> t
        graph.add_edge(0, 3); // s -> y
        graph.add_edge(1, 2); // t -> x
        graph.add_edge(1, 3); // t -> y
        graph.add_edge(2, 4); // x -> z
        graph.add_edge(3, 1); // y -> t
        graph.add_edge(3, 2); // y -> x
        graph.add_edge(3, 4); // y -> z
        graph.add_edge(4, 0); // z -> s
        graph.add_edge(4, 2); // z -> x

        let result = breadth_first_search(&graph, 0);

        assert_eq!(
            result.distances,
            vec![Some(0), Some(1), Some(2), Some(1), Some(2)]
        );
        assert_eq!(result.predecessors, vec![None, Some(0), Some(1), Some(0), Some(3)]);
        assert_eq!(result.path_to(4), Some(vec![0, 3, 4]));
        assert_eq!(result.path_to(2), Some(vec![0, 1, 2]));
    }

    #[test]
    fn bfs_unreachable_vertices() {
        let mut graph = Graph::new(4, true);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        // vertex 3 is isolated

        let result = breadth_first_search(&graph, 0);
        assert_eq!(
            result.distances,
            vec![Some(0), Some(1), Some(2), None]
        );
        assert_eq!(result.path_to(3), None);
    }
}

