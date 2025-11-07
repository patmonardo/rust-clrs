use super::MatrixGraph;

/// Computes the transitive closure of a directed graph using dynamic programming.
pub fn transitive_closure(graph: &MatrixGraph<bool>) -> Vec<Vec<bool>> {
    let n = graph.vertex_count();
    let mut closure = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            closure[i][j] = graph.weights()[i][j].unwrap_or(false);
        }
        closure[i][i] = true;
    }

    for k in 0..n {
        for i in 0..n {
            if closure[i][k] {
                for j in 0..n {
                    closure[i][j] = closure[i][j] || closure[k][j];
                }
            }
        }
    }

    closure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transitive_closure_example() {
        let mut graph: MatrixGraph<bool> = MatrixGraph::new(4);
        graph.set_edge(0, 1, true);
        graph.set_edge(1, 2, true);
        graph.set_edge(2, 3, true);

        let closure = transitive_closure(&graph);
        assert_eq!(
            closure,
            vec![
                vec![true, true, true, true],
                vec![false, true, true, true],
                vec![false, false, true, true],
                vec![false, false, false, true],
            ]
        );
    }
}

