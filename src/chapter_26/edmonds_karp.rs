use std::cmp::min;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::FlowNetwork;

/// Computes the maximum flow using the Edmonds-Karp variant of Ford-Fulkerson.
pub fn edmonds_karp<W>(network: &mut FlowNetwork<W>, source: usize, sink: usize) -> W
where
    W: Copy + Ord + Default + AddAssign + SubAssign + Add<Output = W> + Sub<Output = W>,
{
    assert!(source < network.vertex_count(), "source out of bounds");
    assert!(sink < network.vertex_count(), "sink out of bounds");
    assert!(source != sink, "source and sink must differ");

    let mut max_flow = W::default();

    while let Some(path) = bfs(network, source, sink) {
        let mut residual_capacity = W::default();
        let mut first = true;
        for edge_index in &path {
            let capacity = network.residual_capacity(*edge_index);
            residual_capacity = if first {
                first = false;
                capacity
            } else {
                min(residual_capacity, capacity)
            };
        }

        max_flow += residual_capacity;
        for edge_index in path {
            network.augment_edge(edge_index, residual_capacity);
        }
    }

    max_flow
}

fn bfs<W>(network: &FlowNetwork<W>, source: usize, sink: usize) -> Option<Vec<usize>>
where
    W: Copy + PartialOrd + Default + Sub<Output = W>,
{
    let mut parent = vec![None; network.vertex_count()];
    let mut queue = VecDeque::new();
    queue.push_back(source);
    parent[source] = Some((source, usize::MAX));

    while let Some(u) = queue.pop_front() {
        if u == sink {
            break;
        }
        for &edge_index in network.adjacency(u) {
            let edge = &network.edges()[edge_index];
            if parent[edge.to].is_none() && network.residual_capacity(edge_index) > W::default() {
                parent[edge.to] = Some((u, edge_index));
                queue.push_back(edge.to);
            }
        }
    }

    parent[sink]?;

    let mut path = Vec::new();
    let mut current = sink;
    while current != source {
        let (prev, edge_index) = parent[current].expect("path should exist");
        path.push(edge_index);
        current = prev;
    }
    path.reverse();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edmonds_karp_example() {
        // CLRS Figure 26.1
        let mut network: FlowNetwork<i32> = FlowNetwork::new(6);
        network.add_edge(0, 1, 16);
        network.add_edge(0, 2, 13);
        network.add_edge(1, 2, 10);
        network.add_edge(1, 3, 12);
        network.add_edge(2, 1, 4);
        network.add_edge(2, 4, 14);
        network.add_edge(3, 2, 9);
        network.add_edge(3, 5, 20);
        network.add_edge(4, 3, 7);
        network.add_edge(4, 5, 4);

        let max_flow = edmonds_karp(&mut network, 0, 5);
        assert_eq!(max_flow, 23);
    }
}
