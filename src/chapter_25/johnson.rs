use std::ops::{Add, Sub};

use super::MatrixGraph;
use crate::chapter_24::{
    bellman_ford,
    bellman_ford::BellmanFordError,
    dijkstra,
    dijkstra::DijkstraError,
    weighted_digraph::{ShortestPathResult, WeightedDigraph},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JohnsonError {
    NegativeCycle,
}

/// Runs Johnson's algorithm to compute all-pairs shortest paths on a sparse graph.
pub fn johnson<W>(graph: &WeightedDigraph<W>) -> Result<Vec<Vec<Option<W>>>, JohnsonError>
where
    W: Copy + Ord + PartialOrd + Add<Output = W> + Sub<Output = W> + Default,
{
    let n = graph.vertex_count();
    let mut extended = WeightedDigraph::new(n + 1);

    for u in 0..n {
        for (v, weight) in graph.neighbors(u) {
            extended.add_edge(u, v, weight);
        }
    }
    let super_source = n;
    for v in 0..n {
        extended.add_edge(super_source, v, W::default());
    }

    let potentials = bellman_ford(&extended, super_source).map_err(|err| match err {
        BellmanFordError::NegativeCycle => JohnsonError::NegativeCycle,
    })?;

    let mut h = vec![W::default(); n];
    for v in 0..n {
        h[v] = potentials.distances[v].ok_or(JohnsonError::NegativeCycle)?;
    }

    let reweighted = graph.reweight(&h);

    let mut distances = vec![vec![None; n]; n];
    for u in 0..n {
        let result = dijkstra(&reweighted, u).map_err(|err| match err {
            DijkstraError::NegativeEdgeWeight => {
                unreachable!("reweighting guarantees non-negative edges")
            }
        })?;
        convert_distances(u, &h, &result, &mut distances);
    }

    Ok(distances)
}

fn convert_distances<W>(
    source: usize,
    potentials: &[W],
    result: &ShortestPathResult<W>,
    all_pairs: &mut [Vec<Option<W>>],
) where
    W: Copy + Add<Output = W> + Sub<Output = W>,
{
    for (v, maybe_dist) in result.distances.iter().enumerate() {
        if let Some(dist_prime) = maybe_dist {
            let w = *dist_prime + potentials[v] - potentials[source];
            all_pairs[source][v] = Some(w);
        }
    }
}

/// Builds a `MatrixGraph` from the Johnson output distances.
pub fn johnson_distance_matrix<W>(
    graph: &WeightedDigraph<W>,
) -> Result<MatrixGraph<W>, JohnsonError>
where
    W: Copy + Ord + PartialOrd + Add<Output = W> + Sub<Output = W> + Default,
{
    let distances = johnson(graph)?;
    let n = distances.len();
    let mut matrix = MatrixGraph::new(n);

    for i in 0..n {
        for j in 0..n {
            if let Some(weight) = distances[i][j] {
                matrix.set_edge(i, j, weight);
            }
        }
    }

    Ok(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn johnson_example() {
        // CLRS Figure 25.4
        let mut graph = WeightedDigraph::new(5);
        graph.add_edge(0, 1, 3);
        graph.add_edge(0, 2, 8);
        graph.add_edge(0, 3, -4);
        graph.add_edge(1, 3, 7);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 1, 4);
        graph.add_edge(3, 2, -5);
        graph.add_edge(3, 4, 2);
        graph.add_edge(4, 1, 6);

        let matrix = johnson_distance_matrix(&graph).expect("no negative cycles");

        assert_eq!(
            matrix.weights()[0],
            vec![Some(0), Some(-5), Some(-9), Some(-4), Some(-2)]
        );
        assert_eq!(
            matrix.weights()[1],
            vec![None, Some(0), Some(1), Some(7), Some(9)]
        );
        assert_eq!(
            matrix.weights()[2],
            vec![None, Some(4), Some(0), Some(11), Some(13)]
        );
        assert_eq!(
            matrix.weights()[3],
            vec![None, Some(-1), Some(-5), Some(0), Some(2)]
        );
        assert_eq!(
            matrix.weights()[4],
            vec![None, Some(6), Some(7), Some(13), Some(0)]
        );
    }

    #[test]
    fn johnson_detects_negative_cycle() {
        let mut graph = WeightedDigraph::new(3);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, -1);
        graph.add_edge(2, 0, -1);

        let result = johnson(&graph);
        assert_eq!(result, Err(JohnsonError::NegativeCycle));
    }
}
