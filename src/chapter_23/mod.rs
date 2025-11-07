//! Chapter 23: Minimum Spanning Trees
//!
//! This module provides implementations of Kruskal's and Prim's algorithms for
//! computing minimum spanning trees (MSTs) of weighted, undirected graphs.

pub mod weighted_graph;
pub mod kruskal;
pub mod prim;

pub use weighted_graph::*;
pub use kruskal::*;
pub use prim::*;

