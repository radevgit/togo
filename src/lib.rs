#![doc(html_no_source)]

//! Basic 2D geometric operations.
//!
//! The intention of this library is to provide a foundation for 2D geometric operations.
//! It includes basic operations like point manipulation and distance/intersection
//! between line segments and circle arcs.
//!  
//! It is intended for use in My other projects, and may not implement all possible geometric operations.
//!
//! # Examples
//!
//! ## Creating and working with points
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create points using the constructor or convenience function
//! let p1 = Point::new(1.0, 2.0);
//! let p2 = point(3.0, 4.0);
//!
//! // Points support arithmetic operations
//! let sum = p1 + p2;
//! assert_eq!(sum.x, 4.0);
//! assert_eq!(sum.y, 6.0);
//!
//! // Calculate distance between points
//! let distance = (p2 - p1).norm();
//! assert!((distance - 2.828427124746190).abs() < 1e-10);
//! ```
//!
//! ## Working with geometric primitives
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create a circle and segment
//! let center = point(0.0, 0.0);
//! let c = circle(center, 5.0);
//! let seg = segment(point(-3.0, 0.0), point(3.0, 0.0));
//!
//! assert_eq!(c.c, center);  // Circle center field is 'c'
//! assert_eq!(c.r, 5.0);     // Circle radius field is 'r'
//! assert_eq!(seg.a.x, -3.0);
//! assert_eq!(seg.b.x, 3.0);
//! ```
//!
//! ## Distance computations
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Distance from point to circle returns (distance, closest_point, is_equidistant)
//! let p = point(10.0, 0.0);
//! let c = circle(point(0.0, 0.0), 5.0);
//! let (dist, closest, _is_equidistant) = dist_point_circle(&p, &c);
//! assert_eq!(dist, 5.0); // Point is 5 units outside the circle
//!
//! // Distance from point to segment returns (distance, closest_point)
//! let seg = segment(point(0.0, 0.0), point(5.0, 0.0));
//! let p = point(2.5, 3.0);
//! let (dist, _closest) = dist_point_segment(&p, &seg);
//! assert_eq!(dist, 3.0); // Point is 3 units above the segment
//! ```
//!
//! ## Intersection tests
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Test intersection between two circles
//! let c1 = circle(point(0.0, 0.0), 3.0);
//! let c2 = circle(point(4.0, 0.0), 3.0);
//!
//! let result = int_circle_circle(c1, c2);
//! // Two circles with overlapping areas should intersect at two points
//! match result {
//!     CircleCircleConfig::NoncocircularTwoPoints(_, _) => {
//!         // Two intersection points found
//!         assert!(true);
//!     },
//!     _ => {
//!         // No intersection or other cases
//!         assert!(false);
//!     }
//! }
//! ```
//!
//! ## Working with arcs
//!
//! <div class="warning">NOTE: Arcs are always CCW (counter-clockwise) in this library.</div>
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create an arc from three points and radius (start, end, center, radius)
//! let start = point(1.0, 0.0);
//! let end = point(0.0, 1.0);
//! let center = point(0.0, 0.0);
//! let a = arc(start, end, center, 1.0);
//!
//! assert_eq!(a.a, start);   // Arc start point field is 'a'
//! assert_eq!(a.b, end);     // Arc end point field is 'b'
//! assert_eq!(a.c, center);  // Arc center field is 'c'
//! assert_eq!(a.r, 1.0);     // Arc radius field is 'r'
//! ```
//!
//! ## Working with lines
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create a line from a point and direction vector
//! let origin = point(0.0, 0.0);
//! let direction = point(1.0, 1.0);
//! let l = line(origin, direction);
//!
//! assert_eq!(l.origin, origin);
//! assert_eq!(l.dir, direction);
//! ```
//!
//! ## Working with intervals
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create an interval (tuple struct with two f64 values)
//! let iv = interval(1.0, 5.0);
//! assert_eq!(iv.0, 1.0);  // First endpoint
//! assert_eq!(iv.1, 5.0);  // Second endpoint
//!
//! // Test if a value is contained in the interval
//! assert!(iv.contains(3.0));
//! assert!(!iv.contains(6.0));
//! ```
//!
//! ## Working with polylines (PVertex)
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create vertices for a polyline
//! let p1 = pvertex(point(0.0, 0.0), 0.0);
//! let p2 = pvertex(point(1.0, 0.0), 0.0);
//! let p3 = pvertex(point(1.0, 1.0), 0.0);
//!
//! let polyline = vec![p1, p2, p3];
//!
//! // Translate the polyline (returns a new polyline)
//! let pp = point(2.0, 3.0);
//! let translated = polyline_translate(&polyline, pp);
//! assert_eq!(translated[0].p.x, 2.0);
//! assert_eq!(translated[0].p.y, 3.0);
//! ```
//!
//! ## Arc-arc distance computation
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create two separate arcs
//! let a1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
//! let a2 = arc(point(4.0, 0.0), point(2.0, 0.0), point(3.0, 0.0), 1.0);
//!
//! // Compute distance between arcs (returns just the distance as f64)
//! let dist = dist_arc_arc(&a1, &a2);
//! assert!(dist > 0.0); // Arcs should be separated
//! ```
//!
//! ## Line-circle intersection
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create a line and circle that intersect
//! let l = line(point(-3.0, 0.0), point(1.0, 0.0)); // Horizontal line through origin
//! let c = circle(point(0.0, 0.0), 2.0);
//!
//! let result = int_line_circle(&l, &c);
//! match result {
//!     LineCircleConfig::TwoPoints(..) => {
//!         // Line intersects circle at two points
//!         assert!(true);
//!     },
//!     _ => assert!(false),
//! }
//! ```
//!
//! ## Segment-segment intersection
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create two intersecting segments
//! let seg1 = segment(point(0.0, 0.0), point(2.0, 2.0));
//! let seg2 = segment(point(0.0, 2.0), point(2.0, 0.0));
//!
//! let result = int_segment_segment(&seg1, &seg2);
//! match result {
//!     SegmentSegmentConfig::OnePoint(pt, ..) => {
//!         // Segments intersect at one point (should be around (1,1))
//!         assert!(point(1.0, 1.0).close_enough(pt, 1e-10));
//!     },
//!     _ => assert!(false),
//! }
//! ```
//!
//! ## Utility functions
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Test floating point equality with tolerance
//! assert!(close_enough(1.0, 1.0000001, 1e-5));
//! assert!(!close_enough(1.0, 1.1, 1e-5));
//!
//! // Check if two floats are almost equal using integer comparison
//! assert!(float_equal(1.0, 1.0, 0));
//!
//! ```
//!
//! ## Arc-arc intersection
//!
//! ```
//! use basegeom::prelude::*;
//!
//! // Create two intersecting arcs
//! let a1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
//!     let a2 = arc(point(1.0, 1.0), point(0.0, 0.0), point(1.0, 0.0), 1.0);
//!     let result = int_arc_arc(&a1, &a2);
//!     match result {
//!         ArcArcConfig::NonCocircularOnePoint(pt) => {
//!             // Arcs intersect at one point
//!             assert_eq!(point(0.5, 0.8660254037844386), pt);
//!         },
//!         _ => {
//!             assert!(false);
//!         }
//!     }
//! ```
//!
//! ## Distance computations
//!
//! ```
//! use basegeom::prelude::*;
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
//! use basegeom::prelude::*;
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
//! use basegeom::prelude::*;
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

// Geometric algorithms and utilities
pub mod algo;
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

// Prelude module for convenient imports
pub mod prelude;


#[cfg(test)]
mod tests {
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
