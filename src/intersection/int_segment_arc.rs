#![allow(dead_code)]

use crate::prelude::*;

// #00024
/// Represents the configuration of the intersection between a segment and an arc.
#[derive(Debug, PartialEq)]
pub enum SegmentArcConfig {
    NoIntersection(),
    OnePoint(Point, f64),
    OnePointTouching(Point, f64),
    TwoPoints(Point, Point, f64, f64),
    TwoPointsTouching(Point, Point, f64, f64),
}

/// Computes the intersection of a segment and an arc.
///
/// This function checks if a segment intersects with an arc defined by its center and radius.
///
/// # Arguments
/// * `segment` - The segment to check for intersection
/// * `arc` - The arc to check for intersection
///
/// # Returns
/// A `SegmentArcConfig` enum indicating the type of intersection:
/// - `NoIntersection` if the segment does not intersect the arc
/// - `OnePoint(p, t)` if the segment intersects the arc at one point `p` with parameter `t`
/// - `OnePointTouching(p, t)` if the segment touches the arc at one point `p` with parameter `t`
/// - `TwoPoints(p0, p1, t0, t1)` if the segment intersects the arc at two points `p0` and `p1` with parameters `t0` and `t1`
/// - `TwoPointsTouching(p0, p1, t0, t1)` if the segment touches the arc at two points `p0` and `p1` with parameters `t0` and `t1`
///
/// # Examples
/// ```
/// use togo::prelude::*;
/// let segment = Segment::new(point(0.0, 0.0), point(1.0, 1.0));
/// let arc = Arc::new(point(0.0, 1.0), point(1.0, 0.0), point(1.0, 1.0), 1.0);
/// let result = int_segment_arc(&segment, &arc);
/// assert_eq!(result, SegmentArcConfig::OnePoint(point(0.2928932188134525, 0.2928932188134525), -0.2928932188134524));
/// ```
pub fn int_segment_arc(segment: &Segment, arc: &Arc) -> SegmentArcConfig {
    let circle = circle(arc.c, arc.r);
    let sc_res = int_segment_circle(segment, &circle);
    match sc_res {
        SegmentCircleConfig::NoIntersection() => SegmentArcConfig::NoIntersection(),
        SegmentCircleConfig::OnePoint(p0, t0) => {
            if arc.contains(p0) {
                if are_ends_touching(arc, segment) {
                    SegmentArcConfig::OnePointTouching(p0, t0)
                } else {
                    SegmentArcConfig::OnePoint(p0, t0)
                }
            } else {
                SegmentArcConfig::NoIntersection()
            }
        }
        SegmentCircleConfig::TwoPoints(p0, p1, t0, t1) => {
            let b0 = arc.contains(p0);
            let b1 = arc.contains(p1);
            if b0 && b1 {
                if are_both_ends_touching(arc, segment) {
                    return SegmentArcConfig::TwoPointsTouching(p0, p1, t0, t1);
                } else {
                    return SegmentArcConfig::TwoPoints(p0, p1, t0, t1);
                }
            }
            if b0 {
                if are_ends_touching(arc, segment) {
                    return SegmentArcConfig::OnePointTouching(p0, t0);
                } else {
                    return SegmentArcConfig::OnePoint(p0, t0);
                }
            }
            if b1 {
                if are_ends_touching(arc, segment) {
                    return SegmentArcConfig::OnePointTouching(p1, t1);
                } else {
                    return SegmentArcConfig::OnePoint(p1, t1);
                }
            }
            SegmentArcConfig::NoIntersection()
        }
    }
}

fn are_ends_touching(arc: &Arc, segment: &Segment) -> bool {
    arc.a == segment.a || arc.a == segment.b || arc.b == segment.a || arc.b == segment.b
}

fn are_both_ends_touching(arc: &Arc, segment: &Segment) -> bool {
    (arc.a == segment.a && arc.b == segment.b) || (arc.b == segment.a && arc.a == segment.b)
}

/// If segment and arc are really intersecting, but not just touching at ends.
pub fn if_really_intersecting_segment_arc(segment: &Segment, arc: &Arc) -> bool {
    let sc_res = int_segment_arc(segment, arc);
    match sc_res {
        SegmentArcConfig::NoIntersection()
        | SegmentArcConfig::OnePointTouching(_, _)
        | SegmentArcConfig::TwoPointsTouching(_, _, _, _) => false,
        SegmentArcConfig::OnePoint(_, _) | SegmentArcConfig::TwoPoints(_, _, _, _) => true,
    }
}

#[cfg(test)]
mod test_int_segment_arc {
    use crate::{arc::arc_circle_parametrization, point::point, segment::segment, svg::svg};

    use super::*;
    const ONE: f64 = 1f64;
    const ZERO: f64 = 0f64;

    #[test]
    fn test_intersect_segment_arc() {
        let mut svg = svg(300.0, 350.0);
        // vertical segment and arc
        let v0 = point(100.0, 100.0);
        let v1 = point(150.0, 150.0);
        let v2 = point(130.0, 200.0);
        let v3 = point(130.0, 0.0);
        let b = -0.5;
        let arc = arc_circle_parametrization(v0, v1, b);
        let segment = segment(v2, v3);
        let res = int_segment_arc(&segment, &arc);
        let (pc, pd) = match res {
            SegmentArcConfig::NoIntersection() => (point(0.0, 0.0), point(0.0, 0.0)),
            SegmentArcConfig::OnePoint(p, _) => (p, p),
            SegmentArcConfig::TwoPoints(p0, p1, _, _) => (p0, p1),
            SegmentArcConfig::OnePointTouching(p, _) => (p, p),
            SegmentArcConfig::TwoPointsTouching(p, d, _, _) => (p, d),
        };

        svg.arc(&arc, "red");
        svg.segment(&segment, "green");
        svg.circle(&circle(pc, 1.0), "black");
        svg.circle(&circle(pd, 1.0), "black");
        svg.write();
        //assert_eq!(res, SegmentArcConfig::OnePoint(point(0.0, 0.0)));
    }
}

// Line Arc Intersect
#[cfg(test)]
mod tests_segment_arc {
    use crate::{
        arc::{arc, arc_circle_parametrization},
        point::point,
        segment::segment,
    };

    use super::*;

    #[test]
    fn test_no_intersection() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let s0 = segment(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let arc0 =
            arc_circle_parametrization(point(1.0, 0.0), point(2.0, 1.0), -1.0 + f64::EPSILON);
        assert_eq!(
            int_segment_arc(&s0, &arc0),
            SegmentArcConfig::NoIntersection()
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_no_intersection2() {
        let s0 = segment(point(-0.5, 1.0), point(0.5, 1.0));
        let arc0 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(res, SegmentArcConfig::NoIntersection());
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_no_intersection3() {
        // segment circle return two points but none is on the arc
        let s0 = segment(point(-1.0, 0.5), point(1.0, 0.5));
        let arc0 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(res, SegmentArcConfig::NoIntersection());
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_no_intersection4() {
        // circle line return one point
        let s0 = segment(point(-1.0, 1.0), point(-0.0, 1.0));
        let arc0 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(res, SegmentArcConfig::NoIntersection());
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_two_points() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let s0 = segment(point(-1.0, 0.0), point(0.0, 1.0));
        let arc1 = arc(point(1.0, 1.0), point(0.0, 0.0), point(0.5, 0.5), sgrt_2_2);
        let res = int_segment_arc(&s0, &arc1);
        match res {
            SegmentArcConfig::OnePoint(p0, _) => {
                assert!(p0.close_enough(point(0.0, 1.0), 1E-8));
            }
            _ => assert!(false),
        }
        assert!(if_really_intersecting_segment_arc(&s0, &arc1) == true);
    }

    #[test]
    fn test_one_point_01() {
        // Segment is touching arc in degenerate interval
        let s0 = segment(point(-0.5, 1.0), point(0.5, 1.0));
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(res, SegmentArcConfig::OnePoint(point(0.0, 1.0), 0.0));
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == true);
    }

    #[test]
    fn test_one_point_02() {
        // circle line return two points but one is not on the arc
        let s0 = segment(point(-1.0, 0.0), point(1.0, 0.0));
        let arc0 = arc(point(0.0, -1.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(res, SegmentArcConfig::OnePoint(point(1.0, 0.0), 1.0));
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == true);
    }

    #[test]
    fn test_one_point_03() {
        // circle line return two points but one is not on the arc
        let s0 = segment(point(-2.0, 0.0), point(2.0, 0.0));
        let arc0 = arc(point(0.0, 1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(res, SegmentArcConfig::OnePoint(point(-1.0, 0.0), -1.0));
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == true);
    }

    #[test]
    fn test_one_point_04() {
        let s0 = segment(point(-1.0, 1.0), point(-0.0, 1.0));
        let arc0 = arc(point(0.0, 1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(
            res,
            SegmentArcConfig::OnePointTouching(point(0.0, 1.0), 0.5)
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_two_points_01() {
        let s0 = segment(point(-1.0, 0.5), point(1.0, 0.5));
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        let x = f64::cos((30.0_f64).to_radians()) - 1.0e-16; // small correction
        assert_eq!(
            res,
            SegmentArcConfig::TwoPoints(point(-x, 0.5), point(x, 0.5), -x, x)
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == true);
    }

    #[test]
    fn test_two_points_02() {
        let s0 = segment(point(-1.0, 0.0), point(1.0, 0.0));
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(
            res,
            SegmentArcConfig::TwoPointsTouching(point(-1.0, 0.0), point(1.0, 0.0), -1.0, 1.0)
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_two_points_02b() {
        let s0 = segment(point(1.0, 0.0), point(-1.0, 0.0));
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(
            res,
            SegmentArcConfig::TwoPointsTouching(point(1.0, 0.0), point(-1.0, 0.0), -1.0, 1.0)
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_two_points_03() {
        let s0 = segment(point(-1.0, 0.0), point(1.0, 0.0));
        let e = std::f64::EPSILON;
        let arc0 = arc(point(1.0, 0.0 + e), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(
            res,
            SegmentArcConfig::OnePointTouching(point(-1.0, 0.0), -1.0)
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }

    #[test]
    fn test_two_points_04() {
        let s0 = segment(point(-1.0, 0.0), point(1.0, 0.0));
        let e = std::f64::EPSILON;
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0 + e), point(0.0, 0.0), 1.0);
        let res = int_segment_arc(&s0, &arc0);
        assert_eq!(
            res,
            SegmentArcConfig::OnePointTouching(point(1.0, 0.0), 1.0)
        );
        assert!(if_really_intersecting_segment_arc(&s0, &arc0) == false);
    }
}
