//! Chapter 20: van Emde Boas Trees
//!
//! This module exposes a recursive, segment-sized implementation following the CLRS
//! presentation. It maintains the standard interface for dynamic-sets over a
//! bounded universe: `insert`, `delete`, `member`, `minimum`, `maximum`, `successor`,
//! and `predecessor`.

pub mod van_emde_boas;

pub use van_emde_boas::*;
