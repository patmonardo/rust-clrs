use std::collections::VecDeque;
use std::ops::Add;

use super::{ShortestPathResult, WeightedDigraph};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DagShortestPathsError {
    NotDag,
}

/// Computes shortest paths in a DAG using relaxation along a topological order.
pub fn dag_shortest_paths<W>(
    graph: &WeightedDigraph<W>,
    source: usize,
) -> Result<ShortestPathResult<W>, DagShortestPathsError>
where
    W: Copy + Add<Output = W> + PartialOrd + Default,
{
    let vertex_count = graph.vertex_count();
    assert!(source < vertex_count, "source vertex out of bounds");

    let order = topological_order(graph).ok_or(DagShortestPathsError::NotDag)?;

    let mut distances = vec![None; vertex_count];
    let mut predecessors = vec![None; vertex_count];
    distances[source] = Some(W::default());

    for &u in &order {
        if distances[u].is_none() {
            continue;
        }
        for (v, weight) in graph.neighbors(u) {
            relax(u, v, weight, &mut distances, &mut predecessors);
        }
    }

    Ok(ShortestPathResult {
        source,
        distances,
        predecessors,
    })
}

fn topological_order<W>(graph: &WeightedDigraph<W>) -> Option<Vec<usize>>
where
    W: Copy,
{
    let vertex_count = graph.vertex_count();
    let mut incoming = vec![0usize; vertex_count];
    for (_, v, _) in graph.edges() {
        incoming[v] += 1;
    }

    let mut queue: VecDeque<usize> = incoming
        .iter()
        .enumerate()
        .filter_map(|(v, &deg)| (deg == 0).then_some(v))
        .collect();

    let mut order = Vec::with_capacity(vertex_count);
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for (v, _) in graph.neighbors(u) {
            incoming[v] -= 1;
            if incoming[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if order.len() == vertex_count {
        Some(order)
    } else {
        None
    }
}

fn relax<W>(
    u: usize,
    v: usize,
    weight: W,
    distances: &mut [Option<W>],
    predecessors: &mut [Option<usize>],
) where
    W: Copy + Add<Output = W> + PartialOrd,
{
    let Some(distance_u) = distances[u] else {
        return;
    };
    let candidate = distance_u + weight;

    match distances[v] {
        None => {
            distances[v] = Some(candidate);
            predecessors[v] = Some(u);
        }
        Some(current) if candidate < current => {
            distances[v] = Some(candidate);
            predecessors[v] = Some(u);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_shortest_paths_example() {
        // Example from CLRS Figure 24.5
        let mut graph = WeightedDigraph::new(6);
        graph.add_edge(0, 1, 5);
        graph.add_edge(0, 2, 3);
        graph.add_edge(1, 2, 2);
        graph.add_edge(1, 3, 6);
        graph.add_edge(2, 3, 7);
        graph.add_edge(2, 4, 4);
        graph.add_edge(2, 5, 2);
        graph.add_edge(3, 4, -1);
        graph.add_edge(3, 5, 1);
        graph.add_edge(4, 5, -2);

        let result = dag_shortest_paths(&graph, 1).expect("should be a DAG");
        assert_eq!(
            result.distances,
            vec![None, Some(0), Some(2), Some(6), Some(5), Some(3)]
        );
        assert_eq!(result.path_to(5), Some(vec![1, 3, 4, 5]));
    }

    #[test]
    fn detects_cycle() {
        let mut graph = WeightedDigraph::new(3);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 0, 1);

        let result = dag_shortest_paths(&graph, 0);
        assert_eq!(result, Err(DagShortestPathsError::NotDag));
    }
}
