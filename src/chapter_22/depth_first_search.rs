use super::Graph;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

/// The outcome of running depth-first search over a graph.
#[derive(Debug, Clone)]
pub struct DfsResult {
    pub discovery_times: Vec<Option<usize>>,
    pub finish_times: Vec<Option<usize>>,
    pub predecessors: Vec<Option<usize>>,
    pub forest: Vec<Vec<usize>>,
    pub finish_order: Vec<usize>,
}

/// Performs depth-first search over the entire graph.
///
/// The DFS runs over every connected component, yielding a depth-first forest of
/// trees, discovery/finish timestamps, and the order in which vertices finish.
pub fn depth_first_search(graph: &Graph) -> DfsResult {
    let vertex_count = graph.vertex_count();

    let mut color = vec![Color::White; vertex_count];
    let mut discovery_times = vec![None; vertex_count];
    let mut finish_times = vec![None; vertex_count];
    let mut predecessors = vec![None; vertex_count];
    let mut finish_order = Vec::with_capacity(vertex_count);
    let mut forest: Vec<Vec<usize>> = Vec::new();
    let mut time = 0usize;

    for u in 0..vertex_count {
        if color[u] == Color::White {
            forest.push(Vec::new());
            dfs_visit(
                graph,
                u,
                &mut color,
                &mut discovery_times,
                &mut finish_times,
                &mut predecessors,
                &mut finish_order,
                &mut time,
                forest.last_mut().expect("forest entry must exist"),
            );
        }
    }

    DfsResult {
        discovery_times,
        finish_times,
        predecessors,
        forest,
        finish_order,
    }
}

fn dfs_visit(
    graph: &Graph,
    u: usize,
    color: &mut [Color],
    discovery_times: &mut [Option<usize>],
    finish_times: &mut [Option<usize>],
    predecessors: &mut [Option<usize>],
    finish_order: &mut Vec<usize>,
    time: &mut usize,
    current_tree: &mut Vec<usize>,
) {
    *time += 1;
    discovery_times[u] = Some(*time);
    color[u] = Color::Gray;
    current_tree.push(u);

    for v in graph.neighbors_iter(u) {
        if color[v] == Color::White {
            predecessors[v] = Some(u);
            dfs_visit(
                graph,
                v,
                color,
                discovery_times,
                finish_times,
                predecessors,
                finish_order,
                time,
                current_tree,
            );
        }
    }

    color[u] = Color::Black;
    *time += 1;
    finish_times[u] = Some(*time);
    finish_order.push(u);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_simple_dag() {
        let mut graph = Graph::new(6, true);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let result = depth_first_search(&graph);

        assert_eq!(
            result.predecessors,
            vec![None, Some(0), Some(1), Some(2), Some(3), Some(4)]
        );
        assert_eq!(result.finish_order, vec![5, 4, 3, 2, 1, 0]);
        assert_eq!(result.forest.len(), 1);
        assert_eq!(result.forest[0], vec![0, 1, 2, 3, 4, 5]);

        // Verify discovery/finish timestamps strictly increase.
        let discovery: Vec<_> = result
            .discovery_times
            .iter()
            .map(|&t| t.expect("all vertices discovered"))
            .collect();
        let finish: Vec<_> = result
            .finish_times
            .iter()
            .map(|&t| t.expect("all vertices finished"))
            .collect();

        for i in 0..discovery.len() {
            assert!(discovery[i] < finish[i]);
        }
    }
}
