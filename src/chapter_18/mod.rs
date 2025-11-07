//! Chapter 18: B-Trees
//!
//! This chapter introduces B-trees, a class of balanced search trees well
//! suited for storage systems that read and write large blocks of data. The
//! implementation here follows the CLRS presentation, including search,
//! insertion, and deletion operations for a B-tree with configurable minimum
//! degree.

pub mod b_tree;

pub use b_tree::*;
