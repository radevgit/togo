//! Spatial data structures and algorithms for efficient geometric queries
//!
//! This module provides spatial indexing structures to accelerate geometric
//! queries and operations on collections of geometric primitives.

pub mod hilbert_rtree;

pub use hilbert_rtree::HilbertRTree;
