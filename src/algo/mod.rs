//! Algorithm module containing various geometric algorithms.
//!
//! This module provides implementations of geometric algorithms that operate
//! on the basic geometric primitives defined in the parent module.

#![allow(dead_code)]

use crate::prelude::*;
use robust::{orient2d, Coord};

// Re-export algorithm submodules here when they are added
// pub mod triangulation;
pub mod area;
pub mod bounding;
pub mod convex_hull;
pub mod tangent;
pub mod self_intersect;
// pub mod closest_pair;

// Re-export all public types and functions for easy access
pub use area::{arcline_area, pointline_area};
pub use bounding::{arc_bounding_circle, arc_bounding_rect};
pub use convex_hull::{arcline_convex_hull, points_convex_hull};
pub use self_intersect::{
    arcline_has_self_intersection, arcline_self_intersections, arcline_self_intersection_status,
    arcline_has_self_intersection_aabb, arcline_self_intersections_aabb,
    SelfIntersectionStatus,
};
//pub use tangent::{tangent_arc_arc, TangentArcArc};

/// Checks if a polygon defined by points is convex.
///
/// # Arguments
///
/// * `points` - A slice of points defining the polygon vertices in order
///
/// # Returns
///
/// `true` if the pointline is convex, `false` otherwise
pub fn is_convex_pointline(points: &Pointline) -> bool {
    if points.len() < 3 {
        return false;
    }

    let n = points.len();
    let mut sign = 0;

    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        let p3 = points[(i + 2) % n];

        // Use robust orient2d predicate for orientation test
        // orient2d returns exactly 0.0 for collinear points (via exact arithmetic),
        // so comparing to 0.0 is safe here (not a floating-point precision issue)
        let orientation = orient2d(
            Coord { x: p1.x, y: p1.y },
            Coord { x: p2.x, y: p2.y },
            Coord { x: p3.x, y: p3.y },
        );

        if orientation != 0.0 {
            let current_sign = if orientation > 0.0 { 1 } else { -1 };
            if sign == 0 {
                sign = current_sign;
            } else if sign != current_sign {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_convex_pointline_square() {
        let square = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        assert!(is_convex_pointline(&square));
    }

    #[test]
    fn test_is_convex_polygon_concave() {
        let concave = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(1.0, 1.0),
            point(2.0, 2.0),
            point(0.0, 2.0),
        ];
        assert!(!is_convex_pointline(&concave));
    }

    #[test]
    fn test_is_convex_polygon_with_collinear_segments() {
        // Rectangle with an extra point on one edge (collinear with adjacent points)
        // Points: (0,0) → (1,0) → (2,0) [collinear] → (2,1) → (2,2) → (0,2) → (0,0)
        // This should be considered convex (collinear segments don't violate convexity)
        let polygon_with_collinear = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),  // collinear with next two
            point(2.0, 0.0),  // on the edge from (0,0) to (2,0)
            point(2.0, 1.0),
            point(2.0, 2.0),
            point(0.0, 2.0),
        ];
        assert!(is_convex_pointline(&polygon_with_collinear));
    }

    #[test]
    fn test_is_convex_polygon_multiple_collinear() {
        // A more complex case with collinear points on multiple edges
        // This forms a rectangle: (0,0)→(3,0)→(3,3)→(0,3)→back to (0,0)
        // But with intermediate collinear points on edges
        let polygon = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),  // collinear on bottom edge
            point(2.0, 0.0),  // collinear on bottom edge
            point(3.0, 0.0),
            point(3.0, 1.0),  // collinear on right edge
            point(3.0, 2.0),  // collinear on right edge
            point(3.0, 3.0),
            point(2.0, 3.0),  // collinear on top edge
            point(1.0, 3.0),  // collinear on top edge
            point(0.0, 3.0),
        ];
        assert!(is_convex_pointline(&polygon));
    }
}
