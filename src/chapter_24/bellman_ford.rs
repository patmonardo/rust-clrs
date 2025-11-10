use std::ops::Add;

use super::{ShortestPathResult, WeightedDigraph};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellmanFordError {
    NegativeCycle,
}

/// Runs the Bellman-Ford algorithm from `source`.
///
/// Returns the shortest-path estimates when no negative cycles are reachable
/// from the source, otherwise reports `BellmanFordError::NegativeCycle`.
pub fn bellman_ford<W>(
    graph: &WeightedDigraph<W>,
    source: usize,
) -> Result<ShortestPathResult<W>, BellmanFordError>
where
    W: Copy + PartialOrd + Add<Output = W> + Default,
{
    let vertex_count = graph.vertex_count();
    assert!(source < vertex_count, "source vertex out of bounds");

    let mut distances: Vec<Option<W>> = vec![None; vertex_count];
    let mut predecessors = vec![None; vertex_count];
    distances[source] = Some(W::default());

    for _ in 0..vertex_count.saturating_sub(1) {
        let mut updated = false;
        for (u, v, weight) in graph.edges() {
            if relax(u, v, weight, &mut distances, &mut predecessors) {
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }

    for (u, v, weight) in graph.edges() {
        if let Some(new_distance) = candidate_distance(u, weight, &distances) {
            if let Some(current) = distances[v] {
                if new_distance < current {
                    return Err(BellmanFordError::NegativeCycle);
                }
            }
        }
    }

    Ok(ShortestPathResult {
        source,
        distances,
        predecessors,
    })
}

fn candidate_distance<W>(u: usize, weight: W, distances: &[Option<W>]) -> Option<W>
where
    W: Copy + Add<Output = W>,
{
    distances[u].map(|distance_u| distance_u + weight)
}

fn relax<W>(
    u: usize,
    v: usize,
    weight: W,
    distances: &mut [Option<W>],
    predecessors: &mut [Option<usize>],
) -> bool
where
    W: Copy + PartialOrd + Add<Output = W>,
{
    let Some(new_distance) = candidate_distance(u, weight, distances) else {
        return false;
    };

    match distances[v] {
        None => {
            distances[v] = Some(new_distance);
            predecessors[v] = Some(u);
            true
        }
        Some(current) if new_distance < current => {
            distances[v] = Some(new_distance);
            predecessors[v] = Some(u);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bellman_ford_example() {
        // CLRS Figure 24.4
        let mut graph = WeightedDigraph::new(5);
        graph.add_edge(0, 1, 6);
        graph.add_edge(0, 3, 7);
        graph.add_edge(1, 2, 5);
        graph.add_edge(1, 3, 8);
        graph.add_edge(1, 4, -4);
        graph.add_edge(2, 1, -2);
        graph.add_edge(3, 2, -3);
        graph.add_edge(3, 4, 9);
        graph.add_edge(4, 0, 2);
        graph.add_edge(4, 2, 7);

        let result = bellman_ford(&graph, 0).expect("no negative cycle");
        assert_eq!(
            result.distances,
            vec![Some(0), Some(2), Some(4), Some(7), Some(-2)]
        );
        assert_eq!(
            result.predecessors,
            vec![None, Some(2), Some(3), Some(0), Some(1)]
        );
        assert_eq!(result.path_to(4), Some(vec![0, 3, 2, 1, 4]));
    }

    #[test]
    fn detects_negative_cycle() {
        let mut graph = WeightedDigraph::new(3);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, -1);
        graph.add_edge(2, 0, -1);

        let result = bellman_ford(&graph, 0);
        assert_eq!(result, Err(BellmanFordError::NegativeCycle));
    }
}
