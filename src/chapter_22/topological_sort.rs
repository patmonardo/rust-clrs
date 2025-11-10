use super::Graph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologicalSortError {
    NotDag,
}

/// Returns a topological ordering of a directed acyclic graph.
///
/// # Panics
///
/// Panics if the input graph is undirected.
pub fn topological_sort(graph: &Graph) -> Result<Vec<usize>, TopologicalSortError> {
    assert!(
        graph.is_directed(),
        "topological sort requires a directed graph"
    );

    let vertex_count = graph.vertex_count();
    let mut color = vec![Color::White; vertex_count];
    let mut order = Vec::with_capacity(vertex_count);

    for u in 0..vertex_count {
        if color[u] == Color::White {
            dfs_visit(graph, u, &mut color, &mut order)?;
        }
    }

    order.reverse();
    Ok(order)
}

fn dfs_visit(
    graph: &Graph,
    u: usize,
    color: &mut [Color],
    order: &mut Vec<usize>,
) -> Result<(), TopologicalSortError> {
    color[u] = Color::Gray;
    for v in graph.neighbors_iter(u) {
        match color[v] {
            Color::White => dfs_visit(graph, v, color, order)?,
            Color::Gray => return Err(TopologicalSortError::NotDag),
            Color::Black => {}
        }
    }
    color[u] = Color::Black;
    order.push(u);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topo_sort_linear_graph() {
        let mut graph = Graph::new(6, true);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let order = topological_sort(&graph).expect("should be a DAG");
        assert_eq!(order, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn topo_sort_clrs_example() {
        // Example similar to CLRS Figure 22.8.
        let mut graph = Graph::new(8, true);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 5);
        graph.add_edge(4, 5);
        graph.add_edge(4, 6);
        graph.add_edge(6, 7);

        let order = topological_sort(&graph).expect("should be a DAG");

        // Verify that every edge points forward in the ordering.
        let position: Vec<_> = {
            let mut pos = vec![0usize; order.len()];
            for (idx, &vertex) in order.iter().enumerate() {
                pos[vertex] = idx;
            }
            pos
        };

        for u in 0..graph.vertex_count() {
            for v in graph.neighbors_iter(u) {
                assert!(
                    position[u] < position[v],
                    "edge {u}->{v} violates topological order"
                );
            }
        }
    }

    #[test]
    fn topo_sort_detects_cycle() {
        let mut graph = Graph::new(3, true);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let result = topological_sort(&graph);
        assert_eq!(result, Err(TopologicalSortError::NotDag));
    }
}
