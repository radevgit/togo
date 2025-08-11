#![allow(dead_code)]

//! Distance computation algorithms module.
//!
//! This module contains algorithms for computing distances between
//! various geometric primitives such as points, segments, arcs, circles, and lines.

// Module declarations
pub mod dist_arc_arc;
pub mod dist_line_circle;
pub mod dist_point_arc;
pub mod dist_point_circle;
pub mod dist_point_segment;
pub mod dist_segment_arc;
pub mod dist_segment_circle;
pub mod dist_segment_segment;

// Re-export all public types and functions for easy access
pub use dist_arc_arc::dist_arc_arc;
pub use dist_line_circle::{DistLineCircleConfig, dist_line_circle};
pub use dist_point_arc::{DistPointArcConfig, dist_point_arc, dist_point_arc_dist};
pub use dist_point_circle::dist_point_circle;
pub use dist_point_segment::dist_point_segment;
pub use dist_segment_arc::dist_segment_arc;
pub use dist_segment_circle::{DistSegmentCircleConfig, dist_segment_circle};
pub use dist_segment_segment::dist_segment_segment;
