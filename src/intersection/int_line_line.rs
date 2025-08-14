#![allow(dead_code)]

use crate::{line::Line, point::Point};

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
/// use basegeom::prelude::*;
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
    if dot_d0_perp_d1 == ZERO {
        // The lines are parallel.
        let dot_qperp_d1 = q.perp(line1.dir);
        if dot_qperp_d1.abs() == ZERO {
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
        let s0 = dot_qperp_d1 / dot_d0_perp_d1;
        let s1 = dot_qperp_d0 / dot_d0_perp_d1;
        let p = line0.origin + line0.dir * s0;
        LineLineConfig::OnePoint(p, s0, s1)
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
}
