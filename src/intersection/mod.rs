#![allow(dead_code)]

//! Intersection algorithms module.
//!
//! This module contains algorithms for computing intersections between
//! various geometric primitives such as arcs, circles, lines, segments, and intervals.

// Module declarations
pub mod int_arc_arc;
pub mod int_circle_circle;
pub mod int_interval_interval;
pub mod int_line_arc;
pub mod int_line_circle;
pub mod int_line_line;
pub mod int_segment_arc;
pub mod int_segment_circle;
pub mod int_segment_point;
pub mod int_segment_segment;

// Re-export all public types and functions for easy access
pub use int_arc_arc::{ArcArcConfig, int_arc_arc, if_really_intersecting_arc_arc};
pub use int_circle_circle::{CircleCircleConfig, int_circle_circle};
pub use int_interval_interval::{IntervalConfig, int_interval_interval};
pub use int_line_arc::{LineArcConfig, int_line_arc};
pub use int_line_circle::{LineCircleConfig, int_line_circle};
pub use int_line_line::{LineLineConfig, int_line_line};
pub use int_segment_arc::{SegmentArcConfig, int_segment_arc, if_really_intersecting_segment_arc};
pub use int_segment_circle::{SegmentCircleConfig, int_segment_circle};
pub use int_segment_point::{SegmentPointConfig, int_segment_point};
pub use int_segment_segment::{SegmentSegmentConfig, int_segment_segment, if_really_intersecting_segment_segment};