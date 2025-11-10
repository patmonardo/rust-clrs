use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use super::{ShortestPathResult, WeightedDigraph};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DijkstraError {
    NegativeEdgeWeight,
}

/// Computes shortest paths from `source` using Dijkstra's algorithm.
///
/// Returns an error if a negative-weight edge is present in the graph.
pub fn dijkstra<W>(
    graph: &WeightedDigraph<W>,
    source: usize,
) -> Result<ShortestPathResult<W>, DijkstraError>
where
    W: Copy + Ord + Add<Output = W> + Default,
{
    let vertex_count = graph.vertex_count();
    assert!(source < vertex_count, "source vertex out of bounds");

    for (_, _, weight) in graph.edges() {
        if weight < W::default() {
            return Err(DijkstraError::NegativeEdgeWeight);
        }
    }

    let mut distances = vec![None; vertex_count];
    let mut predecessors = vec![None; vertex_count];
    let mut visited = vec![false; vertex_count];
    let mut heap: BinaryHeap<(Reverse<W>, usize)> = BinaryHeap::new();

    distances[source] = Some(W::default());
    heap.push((Reverse(W::default()), source));

    while let Some((Reverse(distance_u), u)) = heap.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;

        for (v, weight) in graph.neighbors(u) {
            let candidate = distance_u + weight;
            match distances[v] {
                None => {
                    distances[v] = Some(candidate);
                    predecessors[v] = Some(u);
                    heap.push((Reverse(candidate), v));
                }
                Some(current) if candidate < current => {
                    distances[v] = Some(candidate);
                    predecessors[v] = Some(u);
                    heap.push((Reverse(candidate), v));
                }
                _ => {}
            }
        }
    }

    Ok(ShortestPathResult {
        source,
        distances,
        predecessors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_example() {
        // CLRS Figure 24.6
        let mut graph = WeightedDigraph::new(5);
        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 3, 5);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 4, 4);
        graph.add_edge(3, 1, 3);
        graph.add_edge(3, 2, 9);
        graph.add_edge(3, 4, 2);
        graph.add_edge(4, 0, 7);
        graph.add_edge(4, 2, 6);

        let result = dijkstra(&graph, 0).expect("graph has no negative edges");
        assert_eq!(
            result.distances,
            vec![Some(0), Some(8), Some(9), Some(5), Some(7)]
        );
        assert_eq!(result.path_to(2), Some(vec![0, 3, 1, 2]));
    }

    #[test]
    fn rejects_negative_edge() {
        let mut graph = WeightedDigraph::new(2);
        graph.add_edge(0, 1, -1);

        let result = dijkstra(&graph, 0);
        assert_eq!(result, Err(DijkstraError::NegativeEdgeWeight));
    }
}
