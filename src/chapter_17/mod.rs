//! Chapter 17: Amortized Analysis
//!
//! This chapter covers amortized analysis, which provides a way to analyze
//! the average performance of a sequence of operations, even when some
//! individual operations are expensive.

pub mod binary_counter;
pub mod dynamic_table;
pub mod stack_operations;

pub use binary_counter::*;
pub use dynamic_table::*;
pub use stack_operations::*;

