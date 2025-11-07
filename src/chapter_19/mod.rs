//! Chapter 19: Fibonacci Heaps
//!
//! This module provides a safe Rust translation of the Fibonacci heap
//! data structure presented in CLRS. Fibonacci heaps support a collection of
//! operations with excellent amortized bounds, which becomes particularly
//! important for algorithms such as Dijkstra's shortest path.

pub mod fibonacci_heap;

pub use fibonacci_heap::*;
