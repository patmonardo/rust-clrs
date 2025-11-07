//! Chapter 9: Medians and Order Statistics
//!
//! This chapter covers algorithms for finding the ith smallest element
//! in a set, including minimum/maximum, randomized selection, and
//! worst-case linear time selection.

pub mod min_max;
pub mod randomized_select;
pub mod select;

pub use min_max::*;
pub use randomized_select::*;
pub use select::*;
