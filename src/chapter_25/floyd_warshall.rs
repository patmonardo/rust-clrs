use std::ops::Add;

use super::MatrixGraph;

/// Runs the Floyd-Warshall algorithm to compute all-pairs shortest paths.
pub fn floyd_warshall<W>(graph: &MatrixGraph<W>) -> Vec<Vec<Option<W>>>
where
    W: Copy + PartialOrd + Add<Output = W> + Default,
{
    let n = graph.vertex_count();
    let mut dist = graph.weights().to_vec();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let Some(dik) = dist[i][k] else { continue };
                let Some(dkj) = dist[k][j] else { continue };
                let candidate = dik + dkj;
                match dist[i][j] {
                    None => dist[i][j] = Some(candidate),
                    Some(current) if candidate < current => dist[i][j] = Some(candidate),
                    _ => {}
                }
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floyd_warshall_example() {
        let mut graph: MatrixGraph<i32> = MatrixGraph::new(3);
        graph.set_edge(0, 1, 5);
        graph.set_edge(0, 2, 10);
        graph.set_edge(1, 2, 3);
        graph.set_edge(2, 0, 2);

        let result = floyd_warshall(&graph);

        assert_eq!(
            result,
            vec![
                vec![Some(0), Some(5), Some(8)],
                vec![Some(5), Some(0), Some(3)],
                vec![Some(2), Some(7), Some(0)],
            ]
        );
    }
}
