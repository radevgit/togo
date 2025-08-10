#![allow(dead_code)]

use crate::{arc::Arc, circle::circle, int_line_circle::int_line_circle, line::Line, point::Point};

// #00021
/// Represents the configuration of the intersection between a line and an arc.
#[derive(Debug, PartialEq)]
pub enum LineArcConfig {
    NoIntersection(),
    OnePoint(Point, f64),
    TwoPoints(Point, Point, f64, f64),
}

/// Computes the intersection of a line and an arc.
///
/// This function checks if a line intersects with an arc defined by its center and radius.
///
/// # Arguments
/// * `line` - The line to check for intersection
/// * `arc` - The arc to check for intersection
///
/// # Returns
/// A `LineArcConfig` enum indicating the type of intersection:
/// - `NoIntersection` if the line does not intersect the arc
/// - `OnePoint(p, t)` if the line intersects the arc at one point `p` with parameter `t`
/// - `TwoPoints(p0, p1, t0, t1)` if the line intersects the arc at two points `p0` and `p1` with parameters `t0` and `t1`
///
/// # Examples
/// ```
/// use basegeom::prelude::*;
/// let line = Line::new(point(0.0, 0.0), point(1.0, 1.0));
/// let arc = Arc::new(point(0.0, 1.0), point(1.0, 0.0), point(1.0, 1.0), 1.0);
/// let result = int_line_arc(&line, &arc);
/// assert_eq!(result, LineArcConfig::OnePoint(point(0.29289321881345254, 0.29289321881345254), 0.41421356237309515));
/// ```
pub fn int_line_arc(line: &Line, arc: &Arc) -> LineArcConfig {
    let circle = circle(arc.c, arc.r);
    let lc_result = int_line_circle(line, &circle);
    match lc_result {
        crate::int_line_circle::LineCircleConfig::NoIntersection() => {
            return LineArcConfig::NoIntersection();
        }
        crate::int_line_circle::LineCircleConfig::OnePoint(p0, t0) => {
            if arc.contains(p0) {
                return LineArcConfig::OnePoint(p0, t0);
            } else {
                return LineArcConfig::NoIntersection();
            }
        }
        crate::int_line_circle::LineCircleConfig::TwoPoints(p0, p1, t0, t1) => {
            let b0 = arc.contains(p0); // TODO: with eps?
            let b1 = arc.contains(p1);
            if b0 && b1 {
                return LineArcConfig::TwoPoints(p0, p1, t0, t1);
            }
            if b0 {
                return LineArcConfig::OnePoint(p0, t0);
            }
            if b1 {
                return LineArcConfig::OnePoint(p1, t1);
            }
            return LineArcConfig::NoIntersection();
        }
    }
}

// Line Arc Intersect
#[cfg(test)]
mod line_arc_tests {
    use crate::{
        arc::{arc, arc_circle_parametrization},
        line::line,
        point::point,
    };

    use super::*;

    #[test]
    fn test_no_intersection() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let l0 = line(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let arc0 =
            arc_circle_parametrization(point(1.0, 0.0), point(2.0, 1.0), -1.0 + f64::EPSILON);
        assert_eq!(int_line_arc(&l0, &arc0), LineArcConfig::NoIntersection());
    }

    #[test]
    fn test_no_intersection2() {
        let l0 = Line::new(point(-0.5, 1.0), point(1.0, 0.0));
        let arc0 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_line_arc(&l0, &arc0);
        assert_eq!(res, LineArcConfig::NoIntersection());
    }

    #[test]
    fn test_no_intersection3() {
        // circle line return two points but none is on the arc
        let l0 = Line::new(point(-1.0, 0.5), point(1.0, 0.0));
        let arc0 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_line_arc(&l0, &arc0);
        assert_eq!(res, LineArcConfig::NoIntersection());
    }

    #[test]
    fn test_two_points() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let l0 = line(point(-1.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let arc1 = arc(point(1.0, 1.0), point(0.0, 0.0), point(0.5, 0.5), sgrt_2_2);
        let res = int_line_arc(&l0, &arc1);
        match res {
            LineArcConfig::TwoPoints(p0, p1, _, _) => {
                assert!(p0.close_enough(point(0.0, 1.0), 1E-7));
                assert!(p1.close_enough(point(0.0, 1.0), 1E-7));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_one_point() {
        let l0 = Line::new(point(-0.5, 1.0), point(1.0, 0.0));
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_line_arc(&l0, &arc0);
        assert_eq!(res, LineArcConfig::OnePoint(point(0.0, 1.0), 0.5));
    }

    #[test]
    fn test_one_point2() {
        // circle line return two points but one is not on the arc
        let l0 = Line::new(point(-1.0, 0.0), point(1.0, 0.0));
        let arc0 = arc(point(0.0, -1.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = int_line_arc(&l0, &arc0);
        assert_eq!(res, LineArcConfig::OnePoint(point(1.0, 0.0), 2.0));
    }

    #[test]
    fn test_one_point3() {
        // circle line return two points but one is not on the arc
        let l0 = Line::new(point(-2.0, 0.0), point(1.0, 0.0));
        let arc0 = arc(point(0.0, 1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let res = int_line_arc(&l0, &arc0);
        assert_eq!(res, LineArcConfig::OnePoint(point(-1.0, 0.0), 1.0));
    }
}
