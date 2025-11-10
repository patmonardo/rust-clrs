use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeRef {
    pub source: usize,
    pub target: usize,
}

/// Flow network represented by adjacency lists with residual capacities.
#[derive(Clone)]
pub struct FlowNetwork<W> {
    adjacency_list: Vec<Vec<usize>>,
    edges: Vec<FlowEdge<W>>,
}

#[derive(Clone)]
pub struct FlowEdge<W> {
    pub to: usize,
    pub capacity: W,
    pub flow: W,
    pub reverse: usize,
}

impl<W> FlowNetwork<W>
where
    W: Copy + Default,
{
    pub fn new(vertex_count: usize) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); vertex_count],
            edges: Vec::new(),
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize, capacity: W) {
        assert!(u < self.vertex_count(), "vertex {} out of bounds", u);
        assert!(v < self.vertex_count(), "vertex {} out of bounds", v);

        let forward_index = self.edges.len();
        let reverse_index = forward_index + 1;

        self.adjacency_list[u].push(forward_index);
        self.edges.push(FlowEdge {
            to: v,
            capacity,
            flow: W::default(),
            reverse: reverse_index,
        });

        self.adjacency_list[v].push(reverse_index);
        self.edges.push(FlowEdge {
            to: u,
            capacity: W::default(),
            flow: W::default(),
            reverse: forward_index,
        });
    }

    pub fn edges(&self) -> &[FlowEdge<W>] {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut [FlowEdge<W>] {
        &mut self.edges
    }

    pub fn adjacency(&self, u: usize) -> &[usize] {
        &self.adjacency_list[u]
    }

    pub fn residual_capacity(&self, edge_index: usize) -> W
    where
        W: std::ops::Sub<Output = W>,
    {
        let edge = &self.edges[edge_index];
        edge.capacity - edge.flow
    }

    pub fn augment_edge(&mut self, edge_index: usize, amount: W)
    where
        W: std::ops::AddAssign + std::ops::SubAssign,
    {
        let reverse_index = self.edges[edge_index].reverse;
        self.edges[edge_index].flow += amount;
        self.edges[reverse_index].flow -= amount;
    }
}

impl<W> fmt::Debug for FlowNetwork<W>
where
    W: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlowNetwork")
            .field("adjacency_list", &self.adjacency_list)
            .field("edges", &self.edges.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge_creates_reverse_edge() {
        let mut network: FlowNetwork<i32> = FlowNetwork::new(3);
        network.add_edge(0, 1, 5);

        assert_eq!(network.adjacency(0).len(), 1);
        assert_eq!(network.adjacency(1).len(), 1);

        let forward = network.adjacency(0)[0];
        let reverse = network.adjacency(1)[0];

        assert_eq!(network.edges()[forward].to, 1);
        assert_eq!(network.edges()[reverse].to, 0);
        assert_eq!(network.edges()[forward].reverse, reverse);
        assert_eq!(network.edges()[reverse].reverse, forward);
    }
}
