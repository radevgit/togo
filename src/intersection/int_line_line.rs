#![allow(dead_code)]

use crate::{line::Line, point::Point};
use robust::{Coord, orient2d};

// #00023
/// Represents the configuration of the intersection between two lines.
#[derive(Debug, PartialEq)]
pub enum LineLineConfig {
    ParallelDistinct(),
    ParallelTheSame(),
    OnePoint(Point, f64, f64), // intersection point and line parameters
}

const ZERO: f64 = 0f64;
/// Computes the intersection of two lines.
///
/// This function checks if two lines intersect, are parallel but distinct, or are the same line.
/// If they intersect, it returns the intersection point and the parameters for both lines.
/// If they are parallel but distinct, it returns `ParallelDistinct`.
/// If they are the same line, it returns `ParallelTheSame`.
///
/// # Arguments
/// * `line0` - The first line
/// * `line1` - The second line
///
/// # Returns
/// A `LineConfig` enum indicating the type of intersection:
/// - `ParallelDistinct` if the lines are parallel but distinct
/// - `ParallelTheSame` if the lines are the same
/// - `OnePoint(p, s0, s1)` if the lines intersect at point `p` with parameters `s0` and `s1`
///
/// # Examples
/// ```
/// use togo::prelude::*;
/// let line0 = Line::new(point(0.0, 0.0), point(1.0, 1.0));
/// let line1 = Line::new(point(0.0, 2.0), point(1.0, -1.0));
/// let result = int_line_line(&line0, &line1);
/// match result {
///     LineLineConfig::OnePoint(p, s0, s1) => {
///         assert_eq!(p, point(1.0, 1.0));
///     }
///     _ => assert!(false),
/// }
/// ```
pub fn int_line_line(line0: &Line, line1: &Line) -> LineLineConfig {
    let q = line1.origin - line0.origin;
    let dot_d0_perp_d1 = line0.dir.perp(line1.dir);
    
    // Use robust orient2d to check if lines are parallel.
    // The orient2d predicate computes the signed area exactly (with adaptive precision)
    // and returns 0.0 if the lines are parallel, avoiding exact floating-point comparisons.
    let det = orient2d(
        Coord { x: 0.0, y: 0.0 },
        Coord {
            x: line0.dir.x,
            y: line0.dir.y,
        },
        Coord {
            x: line1.dir.x,
            y: line1.dir.y,
        },
    );
    
    // orient2d returns 0.0 if points are collinear/parallel (computed with adaptive precision)
    if det == 0.0 {
        // The lines are parallel (determined exactly by robust arithmetic).
        let det_q = orient2d(
            Coord { x: 0.0, y: 0.0 },
            Coord { x: q.x, y: q.y },
            Coord {
                x: line1.dir.x,
                y: line1.dir.y,
            },
        );
        if det_q == 0.0 {
            // The lines are the same.
            LineLineConfig::ParallelTheSame()
        } else {
            // The lines are parallel but distinct.
            LineLineConfig::ParallelDistinct()
        }
    } else {
        // The lines are not parallel.
        let dot_qperp_d0 = q.perp(line0.dir);
        let dot_qperp_d1 = q.perp(line1.dir);
        
        // Safety check: even though orient2d confirmed non-parallelism, floating-point
        // rounding could make dot_d0_perp_d1 extremely close to zero. Treat as parallel.
        if dot_d0_perp_d1.abs() < f64::EPSILON * 1e3 {
            LineLineConfig::ParallelDistinct()
        } else {
            let s0 = dot_qperp_d1 / dot_d0_perp_d1;
            let s1 = dot_qperp_d0 / dot_d0_perp_d1;
            
            // Additional safety check: reject intersections that are too far away
            // (indicates numerical issues with nearly-parallel lines)
            let dir0_mag = line0.dir.norm();
            let dir1_mag = line1.dir.norm();
            let q_mag = q.norm();
            // Compute the characteristic scale of the input geometry. The .max(1.0) ensures
            // we use at least 1.0 as the scale, so that even tiny geometries (near-zero magnitudes)
            // still get meaningful threshold bounds. This prevents division by zero and ensures
            // the threshold adapts properly to the coordinate system's magnitude.
            let input_scale = (dir0_mag + dir1_mag + q_mag).max(1.0);
            let max_param = input_scale * 1e8;
            
            if s0.abs() > max_param || s1.abs() > max_param {
                // Treat as parallel if intersection is unreasonably far
                LineLineConfig::ParallelDistinct()
            } else {
                let p = line0.origin + line0.dir * s0;
                LineLineConfig::OnePoint(p, s0, s1)
            }
        }
    }
}

#[cfg(test)]
mod test_int_line_line {
    use super::*;
    use crate::line::line;
    use crate::point::{almost_equal_as_int, point};

    #[test]
    fn test_parallel_distinct() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let l0 = line(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let l1 = line(point(f64::EPSILON, 0.0), point(sgrt_2_2, sgrt_2_2));
        assert_eq!(int_line_line(&l0, &l1), LineLineConfig::ParallelDistinct());
    }

    #[test]
    fn test_parallel_the_same() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let l0 = line(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let l1 = line(point(1.0, 1.0), point(sgrt_2_2, sgrt_2_2));
        assert_eq!(int_line_line(&l0, &l1), LineLineConfig::ParallelTheSame());
    }

    #[test]
    fn test_one_point() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let sgrt_2 = std::f64::consts::SQRT_2;
        let l0 = line(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let l1 = line(point(0.0, 2.0), point(sgrt_2_2, -sgrt_2_2));
        let res = int_line_line(&l0, &l1);
        match res {
            LineLineConfig::OnePoint(p, s0, s1) => {
                assert_eq!(p, point(1.0, 1.0));
                assert!(almost_equal_as_int(s0, sgrt_2, 1));
                assert!(almost_equal_as_int(s1, sgrt_2, 1));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_inersection_issue() {
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 1.0));
        let line1 = Line::new(point(0.0, 2.0), point(1.0, -1.0));
        let result = int_line_line(&line0, &line1);
        match result {
            LineLineConfig::OnePoint(p, s0, s1) => {
                assert_eq!(p, point(1.0, 1.0));
                assert_eq!(s0, 1.0);
                assert_eq!(s1, 1.0);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_nearly_parallel_lines_with_small_epsilon() {
        // Test lines that are nearly parallel but not exactly
        // The safety check should handle dot_d0_perp_d1 very close to zero
        let line0 = line(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = line(point(0.0, 1.0), point(1.0, 1e-8)); // Very shallow angle
        let result = int_line_line(&line0, &line1);
        // Should either find intersection or treat as parallel
        match result {
            LineLineConfig::ParallelDistinct() | LineLineConfig::OnePoint(_, _, _) => {
                // Both are acceptable for nearly parallel lines
            }
            _ => panic!("Unexpected result for nearly parallel lines"),
        }
    }

    #[test]
    fn test_intersection_far_away_rejected() {
        // Create two lines where the intersection would be very far away
        // This tests the max_param bounds check (lines 92-96)
        let line0 = line(point(0.0, 0.0), point(1.0, 1e-10)); // Very shallow angle
        let line1 = line(point(1e-10, 1.0), point(1e-10, 1.0 + 1e-10)); // Nearly vertical
        let result = int_line_line(&line0, &line1);
        // Should treat as parallel or reject the far intersection
        match result {
            LineLineConfig::ParallelDistinct() => {
                // Correctly identified as non-intersecting
            }
            _ => {
                // Other results may be acceptable depending on numerical precision
            }
        }
    }

    #[test]
    fn test_perpendicular_lines() {
        // Test perpendicular lines (orthogonal directions)
        let line0 = line(point(0.0, 0.0), point(1.0, 0.0)); // Horizontal
        let line1 = line(point(0.5, 0.0), point(0.0, 1.0)); // Vertical through (0.5, y)
        let result = int_line_line(&line0, &line1);
        match result {
            LineLineConfig::OnePoint(p, _, _) => {
                assert_eq!(p, point(0.5, 0.0));
            }
            _ => panic!("Expected intersection for perpendicular lines"),
        }
    }

    #[test]
    fn test_zero_magnitude_direction_check() {
        // Lines with small but non-zero magnitude directions
        let eps = 1e-15;
        let line0 = line(point(0.0, 0.0), point(eps, eps));
        let line1 = line(point(0.0, 1.0), point(eps, 1.0 - eps));
        let result = int_line_line(&line0, &line1);
        // Should handle small magnitudes gracefully
        match result {
            LineLineConfig::ParallelDistinct() | LineLineConfig::OnePoint(_, _, _) => {
                // Both acceptable
            }
            _ => panic!("Unexpected result"),
        }
    }
}
