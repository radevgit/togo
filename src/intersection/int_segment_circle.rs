#![allow(dead_code)]

use crate::prelude::*;

// #00025
/// Represents the configuration of the intersection between a segment and a circle.
#[derive(Debug, PartialEq)]
pub enum SegmentCircleConfig {
    NoIntersection(),
    OnePoint(Point, f64),
    TwoPoints(Point, Point, f64, f64),
}

/// Computes the intersection of a segment and a circle.
///
/// This function checks if a segment intersects with a circle defined by its center and radius.
///
/// # Arguments
/// * `seg` - The segment to check for intersection
/// * `circle` - The circle to check for intersection
///
/// # Returns
/// A `SegmentCircleConfig` enum indicating the type of intersection:
/// - `NoIntersection` if the segment does not intersect the circle
/// - `OnePoint(p, t)` if the segment intersects the circle at one point `p` with parameter `t`
/// - `TwoPoints(p0, p1, t0, t1)` if the segment intersects the circle at two points `p0` and `p1` with parameters `t0` and `t1`
///
/// # Examples
/// ```
/// use basegeom::prelude::*;
/// let seg = Segment::new(point(0.0, 0.0), point(1.0, 1.0));
/// let circle = Circle::new(point(0.0, 1.0), 1.0);
/// let result = int_segment_circle(&seg, &circle);
/// assert_eq!(result, SegmentCircleConfig::TwoPoints(point(-1.1102230246251565e-16, -1.1102230246251565e-16), point(1.0, 1.0), -0.7071067811865476, 0.7071067811865476));
/// ```
pub fn int_segment_circle(seg: &Segment, circle: &Circle) -> SegmentCircleConfig {
    let (seg_origin, seg_direction, seg_extent) = seg.get_centered_form();
    let lc_res = int_line_circle(&line(seg_origin, seg_direction), circle);
    match lc_res {
        LineCircleConfig::NoIntersection() => SegmentCircleConfig::NoIntersection(),
        LineCircleConfig::OnePoint(p0, param0) => {
            // [-segExtent,+segExtent].
            let seg_interval = interval(-seg_extent, seg_extent);
            if seg_interval.contains(param0) {
                SegmentCircleConfig::OnePoint(p0, param0)
            } else {
                SegmentCircleConfig::NoIntersection()
            }
        }
        LineCircleConfig::TwoPoints(p0, p1, param0, param1) => {
            // [-segExtent,+segExtent].
            let seg_interval = interval(-seg_extent, seg_extent);
            let b0 = seg_interval.contains(param0);
            let b1 = seg_interval.contains(param1);
            if b0 && b1 {
                return SegmentCircleConfig::TwoPoints(p0, p1, param0, param1);
            }
            if b0 {
                return SegmentCircleConfig::OnePoint(p0, param0);
            }
            if b1 {
                return SegmentCircleConfig::OnePoint(p1, param1);
            }
            SegmentCircleConfig::NoIntersection()
        }
    }
}

#[cfg(test)]
mod tests_segment_circle {
    use crate::{circle::circle, point::point, segment::segment, utils::perturbed_ulps_as_int};

    use super::*;

    #[test]
    fn test_no_intersection() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let s0 = segment(point(0.0, 0.0), point(sgrt_2_2, sgrt_2_2));
        let c0 = circle(point(3.0, 1.0), 1.0);
        assert_eq!(
            int_segment_circle(&s0, &c0),
            SegmentCircleConfig::NoIntersection()
        );
    }

    #[test]
    fn test_interval_degenerate() {
        let s0 = segment(point(-1.0, 1.0), point(1.0, 1.0));
        let c0 = circle(point(0.0, 0.0), 1.0);
        assert_eq!(
            int_segment_circle(&s0, &c0),
            SegmentCircleConfig::OnePoint(point(0.0, 1.0), 0.0)
        );
    }

    #[test]
    fn test_one_point() {
        let s0 = segment(point(-1.0, 1.0), point(-0.0, 1.0));
        let c0 = circle(point(0.0, 0.0), 1.0);
        assert_eq!(
            int_segment_circle(&s0, &c0),
            SegmentCircleConfig::OnePoint(point(0.0, 1.0), 0.5)
        );
    }

    #[test]
    fn test_one_point2() {
        // Segment touches circle.
        let s0 = segment(point(-2.0, 0.0), point(-1.0, 0.0));
        let c0 = circle(point(0.0, 0.0), 1.0);
        let res = int_segment_circle(&s0, &c0);
        assert_eq!(res, SegmentCircleConfig::OnePoint(point(-1.0, 0.0), 0.5)); // TODO it should be param: 1.0?
    }

    #[test]
    fn test_two_points() {
        let _1_eps = perturbed_ulps_as_int(1.0, -1);
        let s0 = segment(point(-1.0, _1_eps), point(1.0, _1_eps));
        let c0 = circle(point(0.0, 0.0), 1.0);
        let res = int_segment_circle(&s0, &c0);
        if let SegmentCircleConfig::TwoPoints(p0, p1, t0, t1) = res {
            assert_eq!(p0.y, _1_eps);
            assert_eq!(p1.y, _1_eps);
            assert_eq!(p0.x + p1.x, 0.0);
            assert_eq!(t0 + t1, 0.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_two_points_issue() {
        let s0 = segment(point(144.0, 192.0), point(144.0, 205.0));
        let c0 = circle(point(136.0, 197.0), 16.0);
        let res = int_segment_circle(&s0, &c0);
        assert_eq!(res, SegmentCircleConfig::NoIntersection());
    }
}
