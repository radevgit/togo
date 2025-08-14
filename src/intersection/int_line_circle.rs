#![allow(dead_code)]

use crate::{circle::Circle, line::Line, point::Point};

// #00022
/// Represents the configuration of the intersection between a line and a circle.
#[derive(Debug, PartialEq)]
pub enum LineCircleConfig {
    NoIntersection(),
    OnePoint(Point, f64), // intersection point, and intersection parameter
    TwoPoints(Point, Point, f64, f64),
}

const ZERO: f64 = 0f64;
/// Computes the intersection of a line and a circle.
///
/// This function checks if a line intersects with a circle defined by its center and radius.
///
/// # Arguments
/// * `line_orig` - The line to check for intersection
/// * `circle` - The circle to check for intersection
///
/// # Returns
/// A `LineCircleConfig` enum indicating the type of intersection:
/// - `NoIntersection` if the line does not intersect the circle
/// - `OnePoint(p, t)` if the line intersects the circle at one point `p` with parameter `t`
/// - `TwoPoints(p0, p1, t0, t1)` if the line intersects the circle at two points `p0` and `p1` with parameters `t0` and `t1`
///
/// # Examples
/// ```
/// use basegeom::prelude::*;
/// let line = Line::new(point(0.0, 0.0), point(1.0, 1.0));
/// let circle = Circle::new(point(0.0, 1.0), 1.0);
/// let result = int_line_circle(&line, &circle);
/// assert_eq!(result, LineCircleConfig::TwoPoints(point(0.0, 0.0), point(0.9999999999999998, 0.9999999999999998), 0.0, 1.414213562373095));
/// ```
pub fn int_line_circle(line_orig: &Line, circle: &Circle) -> LineCircleConfig {
    // Kahan’s Algorithm for a Correct Discriminant Computation at Last Formally Proven 2009.pdf
    // https://www.cs.ubc.ca/~rbridson/docs/kahan_discriminant.pdf
    // Line direction can be non-unit length
    let line = line_orig.unitdir();
    let diff = line.origin - circle.c;
    let a0 = diff.dot(diff) - circle.r * circle.r;
    let a1 = line.dir.dot(diff);
    //let discr1 = a1 * a1 - a0;
    let discr = a1.mul_add(a1, -a0);
    //let discr2 = khan_discriminant(a0, a1, 1.0);
    //debug_assert!(discr==discr2);
    if discr > ZERO {
        let root = discr.sqrt();
        let parameter0 = -a1 - root;
        let parameter1 = -a1 + root;
        let point0 = line.origin + line.dir * parameter0;
        let point1 = line.origin + line.dir * parameter1;
        LineCircleConfig::TwoPoints(point0, point1, parameter0, parameter1)
    } else if discr < ZERO {
        LineCircleConfig::NoIntersection()
    } else {
        // discr == 0
        // The line is tangent to the circle. Set the parameters to
        // the same number because other queries involving linear
        // components and circular components use interval-interval
        // intersection tests which consume both parameters.
        let parameter0 = -a1;
        let point0 = line.origin + line.dir * parameter0;
        LineCircleConfig::OnePoint(point0, parameter0)
    }
}

#[cfg(test)]
mod test_intersect_line_circle {
    use crate::{circle::circle, point::point, utils::perturbed_ulps_as_int};

    use super::*;

    #[test]
    fn test_no_intersection() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let l0 = Line::new(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let c0 = circle(point(3.0, 1.0), 1.0);
        assert_eq!(
            int_line_circle(&l0, &c0),
            LineCircleConfig::NoIntersection()
        );
    }

    #[test]
    fn test_one_point() {
        let l0 = Line::new(point(0.0, 1.0), point(1.0, 0.0));
        let c0 = circle(point(0.0, 0.0), 1.0);
        assert_eq!(
            int_line_circle(&l0, &c0),
            LineCircleConfig::OnePoint(point(0.0, 1.0), 0.0)
        );
    }

    #[test]
    fn test_two_points() {
        let _1_eps = perturbed_ulps_as_int(1.0, -1);
        let l0 = Line::new(point(0.0, _1_eps), point(1.0, 0.0));
        let c0 = circle(point(0.0, 0.0), 1.0);
        let res = int_line_circle(&l0, &c0);
        match res {
            LineCircleConfig::TwoPoints(p0, p1, t0, t1) => {
                assert_eq!(p0.y, _1_eps);
                assert_eq!(p1.y, _1_eps);
                assert_eq!(p0.x + p1.x, 0.0);
                assert_eq!(t0 + t1, 0.0);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_one_point_dir_notunit_length() {
        let l0 = Line::new(point(1.5, 0.0), point(-3.0, 0.0));
        let c0 = circle(point(-1.5, 0.0), 1.0);
        let res = int_line_circle(&l0, &c0);
        assert_eq!(
            res,
            LineCircleConfig::TwoPoints(point(-0.5, 0.0), point(-2.5, 0.0), 2.0, 4.0)
        );
    }
}
