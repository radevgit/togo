//! Algorithm module containing various geometric algorithms.
//!
//! This module provides implementations of geometric algorithms that operate
//! on the basic geometric primitives defined in the parent module.

#![allow(dead_code)]

use crate::prelude::*;

// Re-export algorithm submodules here when they are added
// pub mod triangulation;
pub mod convex_hull;
pub mod tangent;
pub mod area;
// pub mod closest_pair;

// Re-export all public types and functions for easy access
pub use convex_hull::{pointline_convex_hull, arcline_convex_hull};
pub use area::{arcline_area, pointline_area};
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

        let cross = (p2 - p1).perp(p3 - p2);
        
        if cross != 0.0 {
            let current_sign = if cross > 0.0 { 1 } else { -1 };
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
}