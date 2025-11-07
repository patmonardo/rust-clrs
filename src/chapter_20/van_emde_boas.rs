//! van Emde Boas Trees (Chapter 20)
//!
//! A van Emde Boas tree (vEB tree) is a recursive structure supporting
//! O(log log u) time operations for a universe of keys `0..u`. The implementation
//! follows the CLRS presentation and assumes `u` is a power of two.

fn u_size(power: usize) -> usize {
    1 << power
}

fn upper_power(power: usize) -> usize {
    (power + 1) / 2
}

fn lower_power(power: usize) -> usize {
    power / 2
}

/// Decomposes a key `x` into cluster index and position within the cluster.
fn high(power: usize, x: usize) -> usize {
    x >> lower_power(power)
}

fn low(power: usize, x: usize) -> usize {
    let lower = lower_power(power);
    if lower == 0 {
        0
    } else {
        x & ((1 << lower) - 1)
    }
}

fn index(power: usize, high: usize, low: usize) -> usize {
    (high << lower_power(power)) | low
}

/// van Emde Boas tree data structure.
#[derive(Debug, Clone)]
pub struct VanEmdeBoasTree {
    universe_power: usize,
    pub min: Option<usize>,
    pub max: Option<usize>,
    summary: Option<Box<VanEmdeBoasTree>>,
    clusters: Vec<Option<Box<VanEmdeBoasTree>>>,
}

impl VanEmdeBoasTree {
    /// Creates a new empty van Emde Boas tree with universe size `2^universe_power`.
    pub fn new(universe_power: usize) -> Self {
        assert!(universe_power >= 1, "universe_power must be at least 1");
        if universe_power == 1 {
            Self {
                universe_power,
                min: None,
                max: None,
                summary: None,
                clusters: vec![],
            }
        } else {
            let upper = upper_power(universe_power);
            let cluster_count = 1 << upper;
            let clusters = vec![None; cluster_count];
            Self {
                universe_power,
                min: None,
                max: None,
                summary: Some(Box::new(Self::new(upper))),
                clusters,
            }
        }
    }

    pub fn universe_size(&self) -> usize {
        u_size(self.universe_power)
    }

    pub fn is_empty(&self) -> bool {
        self.min.is_none()
    }

    pub fn member(&self, x: usize) -> bool {
        if Some(x) == self.min || Some(x) == self.max {
            true
        } else if self.universe_power == 1 {
            false
        } else {
            let cluster_index = high(self.universe_power, x);
            let cluster = &self.clusters[cluster_index];
            match cluster {
                None => false,
                Some(cluster_tree) => cluster_tree.member(low(self.universe_power, x)),
            }
        }
    }

    pub fn minimum(&self) -> Option<usize> {
        self.min
    }

    pub fn maximum(&self) -> Option<usize> {
        self.max
    }

    pub fn insert(&mut self, x: usize) {
        assert!(x < self.universe_size(), "key out of bounds");
        if self.is_empty() {
            self.empty_insert(x);
        } else {
            if x < self.min.unwrap() {
                // swap with minimum to maintain invariant
                let current_min = self.min.unwrap();
                self.min = Some(x);
                self.insert(current_min);
                return;
            }

            if self.universe_power == 1 {
                if x > self.max.unwrap() {
                    self.max = Some(x);
                }
            } else {
                let cluster_index = high(self.universe_power, x);
                let position = low(self.universe_power, x);

                if self.clusters[cluster_index].is_none() {
                    let lower = lower_power(self.universe_power);
                    self.clusters[cluster_index] = Some(Box::new(Self::new(lower)));
                }

                if self.clusters[cluster_index].as_ref().unwrap().is_empty() {
                    if let Some(summary) = self.summary.as_mut() {
                        summary.insert(cluster_index);
                    }
                    self.clusters[cluster_index]
                        .as_mut()
                        .unwrap()
                        .empty_insert(position);
                } else {
                    self.clusters[cluster_index]
                        .as_mut()
                        .unwrap()
                        .insert(position);
                }

                if self.max.map_or(true, |current_max| x > current_max) {
                    self.max = Some(x);
                }
            }
        }
    }

    fn empty_insert(&mut self, x: usize) {
        self.min = Some(x);
        self.max = Some(x);
    }

    pub fn delete(&mut self, x: usize) {
        if self.min == self.max {
            self.min = None;
            self.max = None;
            return;
        }

        if self.universe_power == 1 {
            // universe size 2
            if x == 0 {
                self.min = Some(1);
            } else {
                self.min = Some(0);
            }
            self.max = self.min;
        } else {
            if x == self.min.unwrap() {
                if let Some(summary) = self.summary.as_ref() {
                    if let Some(summary_min) = summary.minimum() {
                        let cluster = self.clusters[summary_min].as_ref().unwrap();
                        let new_min_low = cluster.minimum().unwrap();
                        let new_min = index(self.universe_power, summary_min, new_min_low);
                        self.min = Some(new_min);
                        let cluster_index = summary_min;
                        let position = new_min_low;
                        if let Some(cluster_tree) = self.clusters[cluster_index].as_mut() {
                            cluster_tree.delete(position);
                            if cluster_tree.is_empty() {
                                if let Some(summary_mut) = self.summary.as_mut() {
                                    summary_mut.delete(cluster_index);
                                }
                                self.clusters[cluster_index] = None;
                            }
                        }
                    } else {
                        self.min = self.max;
                    }
                }
            } else {
                let cluster_index = high(self.universe_power, x);
                let position = low(self.universe_power, x);
                if let Some(cluster) = self.clusters[cluster_index].as_mut() {
                    cluster.delete(position);
                    if cluster.is_empty() {
                        if let Some(summary) = self.summary.as_mut() {
                            summary.delete(cluster_index);
                        }
                        self.clusters[cluster_index] = None;
                    }
                }
            }

            if let Some(summary) = self.summary.as_ref() {
                if let Some(max_value) = summary.maximum() {
                    let cluster = self.clusters[max_value].as_ref().unwrap();
                    let cluster_max = cluster.maximum().unwrap();
                    self.max = Some(index(self.universe_power, max_value, cluster_max));
                } else {
                    self.max = self.min;
                }
            }
        }
    }

    pub fn successor(&self, x: usize) -> Option<usize> {
        if self.universe_power == 1 {
            if x == 0 && self.max == Some(1) {
                Some(1)
            } else {
                None
            }
        } else if self.min.is_some() && x < self.min.unwrap() {
            self.min
        } else {
            let cluster_index = high(self.universe_power, x);
            let position = low(self.universe_power, x);
            if let Some(cluster) = self.clusters[cluster_index].as_ref() {
                if let Some(cluster_max) = cluster.maximum() {
                    if position < cluster_max {
                        let offset = cluster.successor(position).unwrap();
                        return Some(index(self.universe_power, cluster_index, offset));
                    }
                }
            }

            if let Some(summary) = self.summary.as_ref() {
                if let Some(successor_cluster) = summary.successor(cluster_index) {
                    let cluster = self.clusters[successor_cluster].as_ref().unwrap();
                    let offset = cluster.minimum().unwrap();
                    return Some(index(self.universe_power, successor_cluster, offset));
                }
            }
            None
        }
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        if self.universe_power == 1 {
            if x == 1 && self.min == Some(0) {
                Some(0)
            } else {
                None
            }
        } else if self.max.is_some() && x > self.max.unwrap() {
            self.max
        } else {
            let cluster_index = high(self.universe_power, x);
            let position = low(self.universe_power, x);
            if let Some(cluster) = self.clusters[cluster_index].as_ref() {
                if let Some(cluster_min) = cluster.minimum() {
                    if position > cluster_min {
                        let offset = cluster.predecessor(position).unwrap();
                        return Some(index(self.universe_power, cluster_index, offset));
                    }
                }
            }

            if let Some(summary) = self.summary.as_ref() {
                if let Some(predecessor_cluster) = summary.predecessor(cluster_index) {
                    let cluster = self.clusters[predecessor_cluster].as_ref().unwrap();
                    let offset = cluster.maximum().unwrap();
                    return Some(index(self.universe_power, predecessor_cluster, offset));
                }
            }

            if self.min.is_some() && x > self.min.unwrap() {
                self.min
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_member() {
        let mut veb = VanEmdeBoasTree::new(4); // universe size 16
        veb.insert(2);
        veb.insert(3);
        veb.insert(4);
        assert!(veb.member(2));
        assert!(veb.member(3));
        assert!(veb.member(4));
        assert!(!veb.member(5));
    }

    #[test]
    fn test_minimum_maximum() {
        let mut veb = VanEmdeBoasTree::new(4);
        assert_eq!(veb.minimum(), None);
        veb.insert(8);
        veb.insert(1);
        veb.insert(14);
        assert_eq!(veb.minimum(), Some(1));
        assert_eq!(veb.maximum(), Some(14));
    }

    #[test]
    fn test_successor_predecessor() {
        let mut veb = VanEmdeBoasTree::new(4);
        for &value in &[2, 3, 4, 7, 9, 14] {
            veb.insert(value);
        }

        assert_eq!(veb.successor(4), Some(7));
        assert_eq!(veb.successor(8), Some(9));
        assert_eq!(veb.successor(14), None);

        assert_eq!(veb.predecessor(7), Some(4));
        assert_eq!(veb.predecessor(2), None);
        assert_eq!(veb.predecessor(15), Some(14));
    }

    #[test]
    fn test_delete() {
        let mut veb = VanEmdeBoasTree::new(3); // universe size 8
        for value in 0..8 {
            veb.insert(value);
        }

        veb.delete(3);
        assert!(!veb.member(3));
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(7));

        veb.delete(0);
        assert_eq!(veb.minimum(), Some(1));

        veb.delete(7);
        assert_eq!(veb.maximum(), Some(6));
    }
}
