use std::ops::{AddAssign, Sub, SubAssign};

use super::FlowNetwork;

/// Computes the maximum flow using the relabel-to-front (preflow-push) algorithm.
pub fn relabel_to_front<W>(network: &mut FlowNetwork<W>, source: usize, sink: usize) -> W
where
    W: Copy + Ord + Default + AddAssign + SubAssign + Sub<Output = W>,
{
    assert!(source < network.vertex_count(), "source out of bounds");
    assert!(sink < network.vertex_count(), "sink out of bounds");
    assert!(source != sink, "source and sink must differ");

    let n = network.vertex_count();
    let mut height = vec![0usize; n];
    let mut excess = vec![W::default(); n];
    let mut seen = vec![0usize; n];
    let mut vertices: Vec<usize> = (0..n).filter(|&v| v != source && v != sink).collect();

    height[source] = n;
    initialize_preflow(network, source, &mut excess);

    let mut index = 0usize;
    while index < vertices.len() {
        let u = vertices[index];
        let old_height = height[u];
        discharge(network, u, &mut height, &mut excess, &mut seen);
        if height[u] > old_height {
            vertices.remove(index);
            vertices.insert(0, u);
            index = 0;
        } else {
            index += 1;
        }
    }

    excess[sink]
}

fn initialize_preflow<W>(network: &mut FlowNetwork<W>, source: usize, excess: &mut [W])
where
    W: Copy + Default + AddAssign + SubAssign + Sub<Output = W> + PartialOrd,
{
    let outgoing = network.adjacency(source).to_vec();
    for edge_index in outgoing {
        let (capacity, target) = {
            let edge = &network.edges()[edge_index];
            (edge.capacity, edge.to)
        };
        if capacity > W::default() {
            network.augment_edge(edge_index, capacity);
            excess[source] -= capacity;
            excess[target] += capacity;
        }
    }
}

fn discharge<W>(
    network: &mut FlowNetwork<W>,
    u: usize,
    height: &mut [usize],
    excess: &mut [W],
    seen: &mut [usize],
) where
    W: Copy + Ord + Default + AddAssign + SubAssign + Sub<Output = W>,
{
    while excess[u] > W::default() {
        if seen[u] == network.adjacency(u).len() {
            relabel(network, u, height);
            seen[u] = 0;
        } else {
            let edge_index = network.adjacency(u)[seen[u]];
            if push(network, edge_index, u, height, excess) {
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
    height: &[usize],
    excess: &mut [W],
) -> bool
where
    W: Copy + Ord + Default + AddAssign + SubAssign + Sub<Output = W>,
{
    let target = network.edges()[edge_index].to;
    let residual = network.residual_capacity(edge_index);
    if residual == W::default() || height[u] <= height[target] {
        return false;
    }

    let amount = excess[u].min(residual);
    network.augment_edge(edge_index, amount);
    excess[u] -= amount;
    excess[target] += amount;
    true
}

fn relabel<W>(network: &FlowNetwork<W>, u: usize, height: &mut [usize])
where
    W: Copy + Default + Sub<Output = W> + PartialOrd,
{
    let mut min_height: Option<usize> = None;
    for &edge_index in network.adjacency(u) {
        let residual = network.residual_capacity(edge_index);
        if residual > W::default() {
            let edge_height = height[network.edges()[edge_index].to];
            min_height = Some(match min_height {
                None => edge_height,
                Some(current) => current.min(edge_height),
            });
        }
    }
    if let Some(h) = min_height {
        height[u] = h + 1;
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
