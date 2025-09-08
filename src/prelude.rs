//! Prelude module for convenient importing of commonly used types and functions.
//!
//! This module re-exports the most commonly used items from the basegeom crate,
//! allowing users to import everything they need with a single `use` statement:
//!
//! ```
//! use basegeom::prelude::*;
//! ```


pub const UPLS_ARC_IS_VALID: u64 = 100;

// Re-export core types and functions
pub use crate::algo::{is_convex_pointline, pointline_area, arcline_area, pointline_convex_hull, arc_bounding_circle, arc_bounding_rect};
pub use crate::arc::{
    Arc, Arcline, arc, arc_bulge_from_points, arc_circle_parametrization,
    arcline_translate, arcline_reverse, arcline_is_valid, arcseg, is_really_intersecting,
    ArclineValidation
};
pub use crate::circle::{Circle, circle};
pub use crate::interval::{Interval, interval};
pub use crate::line::{Line, line};
pub use crate::point::{Point, Pointline, point, points_order};
pub use crate::polyline::{
    PVertex, Polyline, polyline_reverse, polyline_scale, polyline_translate, polylines_reverse,
    pvertex,
};
pub use crate::rect::{Rect, rect};
pub use crate::segment::{Segment, segment};
pub use crate::svg::{SVG, svg};

// Re-export distance computation functions
pub use crate::distance::{
    DistLineCircleConfig, DistPointArcConfig, DistSegmentCircleConfig, dist_arc_arc,
    dist_line_circle, dist_point_arc, dist_point_arc_dist, dist_point_circle,
    dist_point_segment, dist_segment_arc, dist_segment_circle, dist_segment_segment,
};

// Re-export intersection computation functions
pub use crate::intersection::{
    ArcArcConfig, CircleCircleConfig, IntervalConfig, LineArcConfig, LineCircleConfig,
    LineLineConfig, SegmentArcConfig, SegmentCircleConfig,
    SegmentSegmentConfig, if_really_intersecting_arc_arc, if_really_intersecting_segment_arc,
    if_really_intersecting_segment_segment, int_arc_arc, int_circle_circle,
    int_interval_interval, int_line_arc, int_line_circle, int_line_line, int_segment_arc,
    int_segment_circle, int_segment_segment,
};

// Re-export utility functions
pub use crate::utils::{
    float_equal, diff_of_prod, min_3, min_4, min_5,
    float_perturbed_as_int64, sum_of_prod,
};

// Note: Bézier curve support is experimental and not yet exported
// pub use crate::bezier::*;
