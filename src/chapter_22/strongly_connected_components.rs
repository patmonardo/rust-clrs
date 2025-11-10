use super::{depth_first_search, Graph};

/// Computes the strongly connected components of a directed graph using the
/// Kosaraju-Sharir algorithm.
///
/// Returns a vector of components, where each component is represented as a
/// list of vertex indices belonging to the same strongly connected component.
///
/// # Panics
///
/// Panics if the graph is undirected.
pub fn strongly_connected_components(graph: &Graph) -> Vec<Vec<usize>> {
    assert!(
        graph.is_directed(),
        "strongly connected components require a directed graph"
    );

    let finish_order = depth_first_search(graph).finish_order;
    let mut order_desc = finish_order;
    order_desc.reverse();

    let transpose = graph.transpose();
    let mut visited = vec![false; graph.vertex_count()];
    let mut components = Vec::new();

    for u in order_desc {
        if !visited[u] {
            let mut component = Vec::new();
            collect_component(&transpose, u, &mut visited, &mut component);
            components.push(component);
        }
    }

    components
}

fn collect_component(graph: &Graph, u: usize, visited: &mut [bool], component: &mut Vec<usize>) {
    visited[u] = true;
    component.push(u);

    for v in graph.neighbors_iter(u) {
        if !visited[v] {
            collect_component(graph, v, visited, component);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scc_example() {
        let mut graph = Graph::new(8, true);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(1, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 3);
        graph.add_edge(5, 6);
        graph.add_edge(2, 6);
        graph.add_edge(6, 7);
        graph.add_edge(7, 6);

        let mut components = strongly_connected_components(&graph);
        for component in &mut components {
            component.sort_unstable();
        }
        components.sort_unstable_by_key(|component| component[0]);

        assert_eq!(components, vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]]);
    }

    #[test]
    fn scc_isolated_vertices() {
        let graph = Graph::new(3, true);
        let mut components = strongly_connected_components(&graph);
        for component in &mut components {
            component.sort_unstable();
        }
        components.sort_unstable_by_key(|component| component[0]);

        assert_eq!(components, vec![vec![0], vec![1], vec![2]]);
    }
}
