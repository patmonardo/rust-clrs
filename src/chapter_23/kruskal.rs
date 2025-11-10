use std::ops::Add;

use super::{MstEdge, MstResult, WeightedGraph};
use crate::chapter_21::DisjointSet;

/// Computes a minimum spanning forest using Kruskal's algorithm.
///
/// Returns the collection of selected edges and their total weight. The result
/// contains one tree per connected component of the input graph.
pub fn kruskal_mst<W>(graph: &WeightedGraph<W>) -> MstResult<W>
where
    W: Copy + Ord + Add<Output = W> + Default,
{
    let mut disjoint_set = DisjointSet::new();
    for vertex in 0..graph.vertex_count() {
        disjoint_set.make_set(vertex);
    }

    let mut edges = graph.edges();
    edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut mst_edges = Vec::new();
    let mut total_weight = W::default();

    for (u, v, weight) in edges {
        if disjoint_set.union(&u, &v) {
            mst_edges.push(MstEdge { u, v, weight });
            total_weight = total_weight + weight;
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
    fn kruskal_example_graph() {
        // Graph inspired by CLRS Figure 23.1
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

        let mst = kruskal_mst(&graph);

        let mut edges = mst.edges.clone();
        edges.sort_unstable_by_key(|edge| (edge.u.min(edge.v), edge.u.max(edge.v)));
        assert_eq!(edges.len(), 8);
        assert_eq!(mst.total_weight, 37);
    }

    #[test]
    fn kruskal_forest() {
        let mut graph = WeightedGraph::new(4);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 2);
        // Two disconnected components.

        let mst = kruskal_mst(&graph);
        assert_eq!(mst.edges.len(), 2);
        assert_eq!(mst.total_weight, 3);
    }
}
