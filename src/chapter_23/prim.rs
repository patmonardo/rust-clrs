use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use super::{MstEdge, MstResult, WeightedGraph};

/// Computes an MST using Prim's algorithm starting from `source`.
///
/// The algorithm returns the spanning tree for the connected component
/// containing `source`. For disconnected graphs, the result will cover only the
/// reachable vertices.
pub fn prim_mst<W>(graph: &WeightedGraph<W>, source: usize) -> MstResult<W>
where
    W: Copy + Ord + Add<Output = W> + Default,
{
    let vertex_count = graph.vertex_count();
    assert!(source < vertex_count, "source vertex out of bounds");

    if vertex_count == 0 {
        return MstResult {
            edges: Vec::new(),
            total_weight: W::default(),
        };
    }

    let mut visited = vec![false; vertex_count];
    let mut heap: BinaryHeap<(Reverse<W>, usize, usize)> = BinaryHeap::new();
    let mut mst_edges = Vec::new();
    let mut total_weight = W::default();

    visited[source] = true;
    for (v, weight) in graph.neighbors(source) {
        heap.push((Reverse(weight), source, v));
    }

    while let Some((Reverse(weight), u, v)) = heap.pop() {
        if visited[v] {
            continue;
        }

        visited[v] = true;
        mst_edges.push(MstEdge { u, v, weight });
        total_weight = total_weight + weight;

        for (next, next_weight) in graph.neighbors(v) {
            if !visited[next] {
                heap.push((Reverse(next_weight), v, next));
            }
        }
    }

    MstResult {
        edges: mst_edges,
        total_weight,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prim_example_graph() {
        let mut graph = WeightedGraph::new(9);
        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 7, 8);
        graph.add_edge(1, 7, 11);
        graph.add_edge(1, 2, 8);
        graph.add_edge(7, 8, 7);
        graph.add_edge(7, 6, 1);
        graph.add_edge(2, 8, 2);
        graph.add_edge(8, 6, 6);
        graph.add_edge(2, 5, 4);
        graph.add_edge(6, 5, 2);
        graph.add_edge(2, 3, 7);
        graph.add_edge(3, 5, 14);
        graph.add_edge(3, 4, 9);
        graph.add_edge(5, 4, 10);

        let mst = prim_mst(&graph, 0);
        assert_eq!(mst.edges.len(), 8);
        assert_eq!(mst.total_weight, 37);
    }

    #[test]
    fn prim_handles_disconnected_component() {
        let mut graph = WeightedGraph::new(5);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 2);
        graph.add_edge(3, 4, 3);

        let mst = prim_mst(&graph, 0);
        let mut edges = mst.edges.clone();
        edges.sort_unstable_by_key(|edge| (edge.u.min(edge.v), edge.u.max(edge.v)));

        assert_eq!(edges, vec![MstEdge { u: 0, v: 1, weight: 1 }, MstEdge { u: 1, v: 2, weight: 2 }]);
        assert_eq!(mst.total_weight, 3);
    }
}

