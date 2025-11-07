use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::{FlowEdge, FlowNetwork};

/// Computes the maximum flow using the relabel-to-front (preflow-push) algorithm.
pub fn relabel_to_front<W>(
    network: &mut FlowNetwork<W>,
    source: usize,
    sink: usize,
) -> W
where
    W: Copy
        + Ord
        + Default
        + AddAssign
        + SubAssign
        + Add<Output = W>
        + Sub<Output = W>
        + From<u8>,
{
    assert!(source < network.vertex_count(), "source out of bounds");
    assert!(sink < network.vertex_count(), "sink out of bounds");
    assert!(source != sink, "source and sink must differ");

    let n = network.vertex_count();
    let mut height = vec![W::default(); n];
    let mut excess = vec![W::default(); n];
    let mut seen = vec![0usize; n];
    let mut vertices: VecDeque<usize> = (0..n).filter(|&v| v != source && v != sink).collect();

    height[source] = W::from(n as u8);
    initialize_preflow(network, source, &mut excess);

    while let Some(u) = vertices.pop_front() {
        let old_height = height[u];
        discharge(
            network,
            u,
            sink,
            &mut height,
            &mut excess,
            &mut seen,
        );
        if height[u] > old_height {
            vertices.push_front(u);
        } else {
            vertices.push_back(u);
        }
    }

    excess[sink]
}

fn initialize_preflow<W>(
    network: &mut FlowNetwork<W>,
    source: usize,
    excess: &mut [W],
) where
    W: Copy + Default + AddAssign + SubAssign,
{
    for &edge_index in network.adjacency(source) {
        let capacity = network.edges()[edge_index].capacity;
        if capacity > W::default() {
            network.augment_edge(edge_index, capacity);
            excess[network.edges()[edge_index].to] += capacity;
        }
    }
}

fn discharge<W>(
    network: &mut FlowNetwork<W>,
    u: usize,
    sink: usize,
    height: &mut [W],
    excess: &mut [W],
    seen: &mut [usize],
) where
    W: Copy
        + Ord
        + Default
        + AddAssign
        + SubAssign
        + Add<Output = W>
        + Sub<Output = W>
        + From<u8>,
{
    while excess[u] > W::default() {
        if seen[u] == network.adjacency(u).len() {
            relabel(network, u, height);
            seen[u] = 0;
        } else {
            let edge_index = network.adjacency(u)[seen[u]];
            if push(network, edge_index, u, sink, height, excess) {
                continue;
            }
            seen[u] += 1;
        }
    }
}

fn push<W>(
    network: &mut FlowNetwork<W>,
    edge_index: usize,
    u: usize,
    sink: usize,
    height: &[W],
    excess: &mut [W],
) -> bool
where
    W: Copy
        + Ord
        + Default
        + AddAssign
        + SubAssign
        + Add<Output = W>
        + Sub<Output = W>,
{
    let edge = network.edges()[edge_index].clone();
    let residual = network.residual_capacity(edge_index);
    if residual == W::default() || height[u] <= height[edge.to] {
        return false;
    }

    let amount = excess[u].min(residual);
    network.augment_edge(edge_index, amount);
    excess[u] -= amount;
    if edge.to != sink {
        excess[edge.to] += amount;
    }
    true
}

fn relabel<W>(network: &FlowNetwork<W>, u: usize, height: &mut [W])
where
    W: Copy + Ord + Default + AddAssign + From<u8>,
{
    let mut min_height = None;
    for &edge_index in network.adjacency(u) {
        let residual = network.residual_capacity(edge_index);
        if residual > W::default() {
            let edge_height = height[network.edges()[edge_index].to];
            min_height = match min_height {
                None => Some(edge_height),
                Some(current) => Some(current.min(edge_height)),
            };
        }
    }
    if let Some(h) = min_height {
        height[u] = h + W::from(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relabel_to_front_example() {
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

        let max_flow = relabel_to_front(&mut network, 0, 5);
        assert_eq!(max_flow, 23);
    }
}

