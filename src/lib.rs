#![doc(html_no_source)]

//! Basic 2D geometric operations.
//!
//! This library provides core 2D geometric primitives and operations including:
//! - **Primitives:** Points, Segments, Circles, Arcs, Polylines, Intervals
//! - **Distance Calculations:** Between points, segments, circles, and arcs
//! - **Intersection Detection:** Line-line, circle-circle, arc-arc, and mixed geometric types
//! - **Geometric Algorithms:** Convex hull, area calculation, bounding shapes
//!
//! > **Important:** All arcs in this library are **CCW (counter-clockwise)** oriented.
//!
//! # Examples
//!
//! ## Distance and intersection calculations
//!
//! ```
//! use togo::prelude::*;
//!
//! // Create primitives
//! let p = point(10.0, 0.0);
//! let c = circle(point(0.0, 0.0), 5.0);
//! let seg = segment(point(0.0, 0.0), point(5.0, 0.0));
//! let a1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
//!
//! // Compute distances
//! let (dist_to_circle, _, _) = dist_point_circle(&p, &c);
//! assert_eq!(dist_to_circle, 5.0);
//!
//! let (dist_to_segment, _) = dist_point_segment(&point(5.0, 5.0), &seg);
//! assert_eq!(dist_to_segment, 5.0);
//!
//! // Distance between arcs
//! let a2 = arc(point(4.0, 0.0), point(2.0, 0.0), point(3.0, 0.0), 1.0);
//! let dist = dist_arc_arc(&a1, &a2);
//! assert!(dist > 0.0);
//! ```
//!
//! ## Intersection tests
//!
//! ```
//! use togo::prelude::*;
//!
//! // Segment intersection
//! let seg1 = segment(point(0.0, 0.0), point(2.0, 2.0));
//! let seg2 = segment(point(0.0, 2.0), point(2.0, 0.0));
//! match int_segment_segment(&seg1, &seg2) {
//!     SegmentSegmentConfig::OnePoint(pt, ..) => {
//!         assert!(point(1.0, 1.0).close_enough(pt, 1e-10));
//!     },
//!     _ => assert!(false, "Expected segment intersection"),
//! }
//!
//! // Circle intersection
//! let c1 = circle(point(0.0, 0.0), 3.0);
//! let c2 = circle(point(4.0, 0.0), 3.0);
//! match int_circle_circle(c1, c2) {
//!     CircleCircleConfig::NoncocircularTwoPoints(_, _) => {
//!         assert!(true); // Two points found
//!     },
//!     _ => assert!(false, "Expected two intersection points"),
//! }
//! ```
//!
//! ## Geometric algorithms
//!
//! ```
//! use togo::prelude::*;
//! use togo::algo::{pointline_area, points_convex_hull, arc_bounding_circle};
//!
//! // Polygon area
//! let triangle = vec![
//!     point(0.0, 0.0),
//!     point(4.0, 0.0),
//!     point(2.0, 3.0),
//!     point(0.0, 0.0),
//! ];
//! let area = pointline_area(&triangle);
//! assert_eq!(area, 6.0);
//!
//! // Convex hull computation
//! let points = vec![
//!     point(0.0, 0.0), point(2.0, 1.0), point(1.0, 2.0),
//!     point(3.0, 0.0), point(2.0, 3.0), point(0.0, 2.0),
//! ];
//! let hull = points_convex_hull(&points);
//! assert_eq!(hull.len(), 4); // 4 points on convex hull
//!
//! // Bounding shape for an arc
//! let quarter_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
//! let bounding = arc_bounding_circle(&quarter_arc);
//! assert_eq!(bounding.r, 0.7071067811865476); // sqrt(2)/2
//! ```
//!
//! ## Distance computations
//!
//! ```
//! use togo::prelude::*;
//!     let l = line(point(0.0, 3.0), point(1.0, 0.0)); // Line with point and direction
//!     let c = circle(point(0.0, 0.0), 2.0);
//!     let result = dist_line_circle(&l, &c);
//!     match result {
//!         DistLineCircleConfig::OnePair(dist, _param, _line_pt, _circle_pt) => {
//!             assert_eq!(1.0, dist);
//!         }
//!         _ => assert!(false),
//!     }
//!
//!     // Distance from point to arc
//!     let p = point(2.0, 0.0);
//!     let a = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
//!     match dist_point_arc(&p, &a) {
//!         DistPointArcConfig::OnePoint(dist, _) => {
//!             assert_eq!(1.0, dist);
//!         }
//!         _ => assert!(false),
//!     }
//!
//!     // Distance from segment to arc
//!     let seg = segment(point(3.0, 0.0), point(4.0, 0.0));
//!     let a = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
//!     let dist = dist_segment_arc(&seg, &a);
//!     assert_eq!(2.0, dist);
//! ```
//!
//! ```
//! use togo::prelude::*;
//!     // Distance from segment to circle
//!     let seg = segment(point(3.0, 0.0), point(4.0, 0.0));
//!     let c = circle(point(0.0, 0.0), 1.0);
//!     let result = dist_segment_circle(&seg, &c);
//!     // Function returns DistSegmentCircleConfig enum
//!     match result {
//!         DistSegmentCircleConfig::OnePoint(dist, closest) => {
//!             assert_eq!(2.0, dist); // Distance should be non-negative
//!         }
//!         _ => assert!(false),
//!     }
//!
//!     // Distance between two segments
//!     let seg1 = segment(point(0.0, 0.0), point(1.0, 0.0));
//!     let seg2 = segment(point(0.0, 2.0), point(1.0, 2.0));
//!     let dist = dist_segment_segment(&seg1, &seg2);
//!     assert_eq!(dist, 2.0); // Parallel segments 2 units apart
//!
//! ```
//!
//! ## Intersection computations
//!
//! ```
//! use togo::prelude::*;
//!
//! // Interval-interval intersection
//! let iv1 = interval(1.0, 5.0);
//! let iv2 = interval(3.0, 7.0);
//! let result = int_interval_interval(iv1, iv2);
//! match result {
//!     IntervalConfig::Overlap(start, end) => {
//!         // Intervals overlap from 3.0 to 5.0
//!         assert_eq!(start, 3.0);
//!         assert_eq!(end, 5.0);
//!     },
//!     _ => assert!(false),
//! }
//!
//! // Line-line intersection
//! let l1 = line(point(0.0, 0.0), point(1.0, 0.0)); // Line with origin and direction
//! let l2 = line(point(0.0, 0.0), point(0.0, 1.0)); // Line with origin and direction
//! let result = int_line_line(&l1, &l2);
//! match result {
//!     LineLineConfig::OnePoint(pt, _param1, _param2) => {
//!         // Lines intersect at origin
//!         assert_eq!(point(0.0, 0.0), pt);
//!     },
//!     _ => assert!(false),
//! }
//! ```

// Core geometric primitives
mod arc;
mod circle;
mod line;
mod point;
mod polyline;
mod rect;
mod segment;

// Centralized constants for numeric stability
pub mod constants;

// Geometric algorithms and utilities
pub mod algo;
pub mod poly;
mod interval;
mod utils;

// Distance computation modules
pub mod distance;

// Intersection computation modules
pub mod intersection;

#[doc(hidden)]
// Bézier curve support (experimental)
pub mod bezier;

// Visualization and debugging
mod svg;

pub mod prelude {
    // Re-export core types and functions
    pub use crate::algo::{
        is_convex_pointline, pointline_area, arcline_area, points_convex_hull, pointline_convex_hull,
        arc_bounding_circle, arc_bounding_rect, arcline_has_self_intersection, 
        arcline_self_intersections, arcline_self_intersection_status, SelfIntersectionStatus
    };
    pub use aabb::HilbertRTree;
    pub use crate::arc::{
        Arc, Arcline, arc, bulge_from_arc, arc_from_bulge,
        arcline_translate, arcline_scale, arcline_reverse, arcline_is_valid, arcseg, is_really_intersecting,
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
        almost_equal_as_int, close_enough, diff_of_prod, min_3, min_4, min_5,
        perturbed_ulps_as_int, sum_of_prod,
    };

    // Note: Bézier curve support is experimental and not yet exported
    // pub use crate::bezier::*;
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod inline_tests {
    use super::prelude::*;

    #[test]
    fn test_distance_computations() {
        let a1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let a2 = arc(point(1.0, 1.0), point(0.0, 0.0), point(1.0, 0.0), 1.0);
        let result = int_arc_arc(&a1, &a2);
        match result {
            ArcArcConfig::NonCocircularOnePoint(pt) => {
                // Arcs intersect at one point
                assert_eq!(point(0.5, 0.8660254037844386), pt);
            }
            _ => {
                // Could be two points, no intersection, or other cases
                assert!(false);
            }
        }
    }

    #[test]
    fn test_intersection_tests() {
        // Distance from segment to circle
        let seg = segment(point(3.0, 0.0), point(4.0, 0.0));
        let c = circle(point(0.0, 0.0), 1.0);
        let result = dist_segment_circle(&seg, &c);
        // Function returns DistSegmentCircleConfig enum
        match result {
            DistSegmentCircleConfig::OnePoint(dist, _closest) => {
                assert_eq!(2.0, dist); // Distance should be non-negative
            }
            _ => assert!(false),
        }

        // Distance between two segments
        let seg1 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let seg2 = segment(point(0.0, 2.0), point(1.0, 2.0));
        let dist = dist_segment_segment(&seg1, &seg2);
        assert_eq!(dist, 2.0); // Parallel segments 2 units apart
    }
}
