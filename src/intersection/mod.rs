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
pub mod int_segment_segment;

// Re-export all public types and functions for easy access
pub use int_arc_arc::{ArcArcConfig, if_really_intersecting_arc_arc, int_arc_arc};
pub use int_circle_circle::{CircleCircleConfig, int_circle_circle};
pub use int_interval_interval::{IntervalConfig, int_interval_interval};
pub use int_line_arc::{LineArcConfig, int_line_arc};
pub use int_line_circle::{LineCircleConfig, int_line_circle};
pub use int_line_line::{LineLineConfig, int_line_line};
pub use int_segment_arc::{SegmentArcConfig, if_really_intersecting_segment_arc, int_segment_arc};
pub use int_segment_circle::{SegmentCircleConfig, int_segment_circle};
pub use int_segment_segment::{
    SegmentSegmentConfig, if_really_intersecting_segment_segment, int_segment_segment,
};
