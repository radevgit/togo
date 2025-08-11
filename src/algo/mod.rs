//! Algorithm module containing various geometric algorithms.
//!
//! This module provides implementations of geometric algorithms that operate
//! on the basic geometric primitives defined in the parent module.

#![allow(dead_code)]

use crate::prelude::*;

// Re-export algorithm submodules here when they are added
// pub mod triangulation;
pub mod convex_hull;
// pub mod closest_pair;

// Re-export all public types and functions for easy access
pub use convex_hull::pointline_convex_hull;


/// Calculates the area of a simple polygon defined by a series of points.
///
/// Uses the shoelace formula (also known as the surveyor's formula) to compute
/// the area of a polygon given its vertices in order.
///
/// # Arguments
///
/// * `points` - A slice of points defining the polygon vertices in order
///
/// # Returns
///
/// The area of the polygon (positive for counter-clockwise orientation)
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
/// use basegeom::algo::pointline_area;
///
/// let square = vec![
///     point(0.0, 0.0),
///     point(1.0, 0.0),
///     point(1.0, 1.0),
///     point(0.0, 1.0),
/// ];
/// let area = pointline_area(&square);
/// assert_eq!(area, 1.0);
/// ```
pub fn pointline_area(points: &Pointline) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    let n = points.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].x * points[j].y;
        area -= points[j].x * points[i].y;
    }
    
    area / 2.0
}

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
    fn test_pointline_area_square() {
        let square = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        let area = pointline_area(&square);
        assert_eq!(area, 1.0);
    }

    #[test]
    fn test_pointline_area_triangle() {
        let triangle = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(1.0, 2.0),
        ];
        let area = pointline_area(&triangle);
        assert_eq!(area, 2.0);
    }

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

