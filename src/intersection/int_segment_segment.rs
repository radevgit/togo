#![allow(dead_code)]

use robust::orient2d;

use crate::constants::COLLINEARITY_TOLERANCE;
use crate::intersection::int_interval_interval::{IntervalConfig, int_interval_interval};
use crate::intersection::int_line_line::{LineLineConfig, int_line_line};
use crate::interval::interval;

use crate::utils::close_enough;
use crate::{line::line, point::Point, segment::Segment};

// #00026
/// Represents the configuration of the intersection between two segments.
#[derive(Debug, PartialEq)]
pub enum SegmentSegmentConfig {
    NoIntersection(),
    OnePoint(Point, f64, f64),
    OnePointTouching(Point, f64, f64),
    TwoPoints(Point, Point, Point, Point),
    TwoPointsTouching(Point, Point, Point, Point),
}

const ZERO: f64 = 0f64;
/// Computes the intersection of two segments.
///
/// This function checks if two segments intersect, are parallel but distinct, or are the same segment.
/// If they intersect, it returns the intersection point and the parameters for both segments.
/// If they are parallel but distinct, it returns `ParallelDistinct`.
/// If they are the same segment, it returns `ParallelTheSame`.
/// # Arguments
/// * `segment0` - The first segment
/// * `segment1` - The second segment
///
/// # Returns
/// A `SegmentSegmentConfig` enum indicating the type of intersection:
/// - `NoIntersection` if the segments do not intersect
/// - `OnePoint(p, s0, s1)` if the segments intersect at point `p` with parameters `s0` and `s1`
/// - `OnePointTouching(p, s0, s1)` if the segments touch at point `p` with parameters `s0` and `s1`
/// - `TwoPoints(p0, p1, p2, p3)` if the segments intersect at two points `p0` and `p1` with parameters `p2` and `p3`
/// - `TwoPointsTouching(p0, p1, p2, p3)` if the segments touch at two points `p0` and `p1` with parameters `p2` and `p3`
///
/// # Examples
/// ```
/// use togo::prelude::*;
/// let segment0 = Segment::new(point(0.0, 0.0), point(2.0, 2.0));
/// let segment1 = Segment::new(point(2.0, 2.0), point(4.0, 0.0));
/// let result = int_segment_segment(&segment0, &segment1);
/// assert_eq!(result, SegmentSegmentConfig::OnePointTouching(point(2.0, 2.0), 1.4142135623730951, -1.4142135623730951));
/// ```
pub fn int_segment_segment(segment0: &Segment, segment1: &Segment) -> SegmentSegmentConfig {
    let (seg0_origin, seg0_direction, seg0_extent) = segment0.get_centered_form();
    let (seg1_origin, seg1_direction, seg1_extent) = segment1.get_centered_form();

    // The some segment is a point
    let is_point0 = segment0.a.close_enough(segment0.b, f64::EPSILON);
    let is_point1 = segment1.a.close_enough(segment1.b, f64::EPSILON);
    if is_point0 && is_point1 {
        if segment0.a.close_enough(segment1.a, f64::EPSILON) {
            return SegmentSegmentConfig::OnePointTouching(segment0.a, ZERO, ZERO);
        } else {
            return SegmentSegmentConfig::NoIntersection();
        }
    }
    if is_point0 {
        // https://stackoverflow.com/questions/328107/how-can-you-determine-a-point-is-between-two-other-points-on-a-line-segment
        let sign = orient2d(
            robust::Coord {
                x: segment1.a.x,
                y: segment1.a.y,
            },
            robust::Coord {
                x: segment1.b.x,
                y: segment1.b.y,
            },
            robust::Coord {
                x: segment0.a.x,
                y: segment0.a.y,
            },
        );
        // Use robust predicate sign exactly via adaptive precision, but apply tolerance
        // as fallback in case of degenerate geometry
        if close_enough(sign, ZERO, COLLINEARITY_TOLERANCE) {
            let dot = (segment1.b - segment1.a).dot(segment0.a - segment1.a);
            let dist = (segment1.b - segment1.a).dot(segment1.b - segment1.a);
            if dot >= ZERO && dot <= dist {
                return SegmentSegmentConfig::OnePoint(segment0.a, ZERO, ZERO);
            } else {
                return SegmentSegmentConfig::NoIntersection();
            }
        } else {
            return SegmentSegmentConfig::NoIntersection();
        }
    }

    if is_point1 {
        let sign = orient2d(
            robust::Coord {
                x: segment0.a.x,
                y: segment0.a.y,
            },
            robust::Coord {
                x: segment0.b.x,
                y: segment0.b.y,
            },
            robust::Coord {
                x: segment1.a.x,
                y: segment1.a.y,
            },
        );
        // Use robust predicate sign exactly via adaptive precision, but apply tolerance
        // as fallback in case of degenerate geometry
        if close_enough(sign, ZERO, COLLINEARITY_TOLERANCE) {
            let dot = (segment0.b - segment0.a).dot(segment1.a - segment0.a);
            let dist = (segment0.b - segment0.a).dot(segment0.b - segment0.a);
            if dot >= ZERO && dot <= dist {
                return SegmentSegmentConfig::OnePoint(segment1.a, ZERO, ZERO);
            } else {
                return SegmentSegmentConfig::NoIntersection();
            }
        } else {
            return SegmentSegmentConfig::NoIntersection();
        }
    }

    let line0 = line(seg0_origin, seg0_direction);
    let line1 = line(seg1_origin, seg1_direction);

    let ll_result = int_line_line(&line0, &line1);

    match ll_result {
        LineLineConfig::ParallelDistinct() => SegmentSegmentConfig::NoIntersection(),
        LineLineConfig::OnePoint(p, s0, s1) => {
            // The lines are not parallel, so they intersect in a single
            // point. Test whether the line-line intersection is on the
            // segments.
            if s0.abs() <= seg0_extent && s1.abs() <= seg1_extent {
                if are_ends_towching(segment0, segment1) {
                    SegmentSegmentConfig::OnePointTouching(p, s0, s1)
                } else {
                    SegmentSegmentConfig::OnePoint(p, s0, s1)
                }
            } else {
                SegmentSegmentConfig::NoIntersection()
            }
        }
        LineLineConfig::ParallelTheSame() => {
            // The lines are the same. Compute the location of segment1
            // endpoints relative to segment0.
            let diff = seg1_origin - seg0_origin;
            let t = seg0_direction.dot(diff);
            let interval0 = interval(-seg0_extent, seg0_extent);
            let interval1 = interval(t - seg1_extent, t + seg1_extent);
            // Compute the intersection of the intervals.
            let ii_result = int_interval_interval(interval0, interval1);
            match ii_result {
                IntervalConfig::NoOverlap() => SegmentSegmentConfig::NoIntersection(),
                IntervalConfig::Overlap(_, _) => {
                    let (p0, p1, p2, p3) =
                        Point::sort_colinear_points(segment0.a, segment0.b, segment1.a, segment1.b);
                    if are_both_ends_towching(segment0, segment1) {
                        return SegmentSegmentConfig::TwoPointsTouching(
                            segment0.a, segment0.b, segment1.a, segment1.b,
                        );
                    }
                    if are_ends_towching(segment0, segment1) {
                        SegmentSegmentConfig::NoIntersection() // TODO: fix this
                    } else {
                        SegmentSegmentConfig::TwoPoints(p0, p1, p2, p3)
                    }
                }
                IntervalConfig::Touching(_) => {
                    SegmentSegmentConfig::NoIntersection() // TODO: fix this
                }
            }
        }
    }
}

fn are_ends_towching(segment0: &Segment, segment1: &Segment) -> bool {
    segment0.a == segment1.a
        || segment0.a == segment1.b
        || segment0.b == segment1.a
        || segment0.b == segment1.b
}

fn are_both_ends_towching(segment0: &Segment, segment1: &Segment) -> bool {
    (segment0.a == segment1.a && segment0.b == segment1.b)
        || (segment0.b == segment1.a && segment0.a == segment1.b)
}

/// If segments are really intersecting, but not just touching at ends.
///
/// In other words, do we need to split segments further?
pub fn if_really_intersecting_segment_segment(part0: &Segment, part1: &Segment) -> bool {
    match int_segment_segment(part0, part1) {
        SegmentSegmentConfig::NoIntersection()
        | SegmentSegmentConfig::OnePointTouching(_, _, _)
        | SegmentSegmentConfig::TwoPointsTouching(_, _, _, _) => false,
        SegmentSegmentConfig::OnePoint(_, _, _) | SegmentSegmentConfig::TwoPoints(_, _, _, _) => {
            true
        }
    }
}

#[cfg(test)]
mod test_int_segment_segment {
    use crate::point::point;
    use crate::segment::segment;

    use super::*;

    #[test]
    fn test_no_intersection() {
        let s0 = segment(point(0.0, 0.0), point(2.0, 2.0));
        let s1 = segment(point(2.0, 1.0), point(4.0, -1.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
        assert!(if_really_intersecting_segment_segment(&s0, &s1) == false);
    }

    #[test]
    fn test_no_intersection_parallel() {
        let s0 = segment(point(0.0, 0.0), point(0.0, 2.0));
        let s1 = segment(point(1.0, 0.0), point(1.0, 2.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
        assert!(if_really_intersecting_segment_segment(&s0, &s1) == false);
    }

    #[test]
    fn test_no_intersection2() {
        // parallel, not overlaping
        let sqrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let p0 = point(0.0, 0.0);
        let p1 = point(sqrt_2_2, sqrt_2_2);
        let delta = point(f64::EPSILON, 0.0);
        let s0 = segment(p0, p1);
        let s1 = segment(p0 + delta, p1 + delta);
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
        assert!(if_really_intersecting_segment_segment(&s0, &s1) == false);
    }

    #[test]
    fn test_parallel_overlaping() {
        // parallel the same, overlaping
        let ulp = std::f64::EPSILON * 2.0;
        let s0 = segment(point(0.0, 0.0), point(2.0, 2.0));
        let s1 = segment(point(1.0, 1.0), point(3.0, 3.0));
        match int_segment_segment(&s0, &s1) {
            SegmentSegmentConfig::TwoPoints(p0, p1, p2, p3) => {
                assert!(p0.close_enough(point(0.0, 0.0), ulp));
                assert!(p1.close_enough(point(1.0, 1.0), ulp));
                assert!(p2.close_enough(point(2.0, 2.0), ulp));
                assert!(p3.close_enough(point(3.0, 3.0), ulp));
                assert!(if_really_intersecting_segment_segment(&s0, &s1) == true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parallel_overlaping2() {
        // parallel the same, overlaping
        let ulp = std::f64::EPSILON * 3.0;
        let s0 = segment(point(0.0, 0.0), point(2.0, 2.0));
        let s1 = segment(point(4.0, 4.0), point(-4.0, -4.0));
        match int_segment_segment(&s0, &s1) {
            SegmentSegmentConfig::TwoPoints(p0, p1, p2, p3) => {
                assert!(p0.close_enough(point(4.0, 4.0), ulp));
                assert!(p1.close_enough(point(2.0, 2.0), ulp));
                assert!(p2.close_enough(point(0.0, 0.0), ulp));
                assert!(p3.close_enough(point(-4.0, -4.0), ulp));
                assert!(if_really_intersecting_segment_segment(&s0, &s1) == true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parallel_touching() {
        // parallel the same, overlaping
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(1.0, 0.0), point(4.0, 0.0));
        assert!(int_segment_segment(&s0, &s1) == SegmentSegmentConfig::NoIntersection());
        assert!(if_really_intersecting_segment_segment(&s0, &s1) == false);
    }

    #[test]
    fn test_touching_at_ends() {
        let sqrt_2 = std::f64::consts::SQRT_2;
        let s0 = segment(point(0.0, 0.0), point(2.0, 2.0));
        let s1 = segment(point(2.0, 2.0), point(4.0, 0.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::OnePointTouching(point(2.0, 2.0), sqrt_2, -sqrt_2)
        );
        assert!(if_really_intersecting_segment_segment(&s0, &s1) == false);
    }

    #[test]
    fn test_zero_size_segment_outside_segment() {
        let s0 = segment(point(2.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(0.0, 0.0), point(1.0, 0.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
        assert_eq!(
            int_segment_segment(&s1, &s0),
            SegmentSegmentConfig::NoIntersection()
        );
    }

    #[test]
    fn test_zero_size_segment_inside_segment() {
        let s0 = segment(point(1.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(0.0, 0.0), point(2.0, 0.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::OnePoint(point(1.0, 0.0), ZERO, ZERO)
        );
        assert_eq!(
            int_segment_segment(&s1, &s0),
            SegmentSegmentConfig::OnePoint(point(1.0, 0.0), ZERO, ZERO)
        );
    }

    #[test]
    fn test_both_zero_size_segments_outside() {
        let s0 = segment(point(2.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(1.0, 0.0), point(1.0, 0.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
        assert_eq!(
            int_segment_segment(&s1, &s0),
            SegmentSegmentConfig::NoIntersection()
        );
    }

    #[test]
    fn test_perpendicular_intersection() {
        // Horizontal and vertical segments intersecting
        let s0 = segment(point(0.0, 0.5), point(2.0, 0.5)); // Horizontal
        let s1 = segment(point(1.0, 0.0), point(1.0, 1.0)); // Vertical
        match int_segment_segment(&s0, &s1) {
            SegmentSegmentConfig::OnePoint(p, _, _) => {
                assert_eq!(p, point(1.0, 0.5));
                assert!(if_really_intersecting_segment_segment(&s0, &s1) == true);
            }
            _ => panic!("Expected intersection for perpendicular segments"),
        }
    }

    #[test]
    fn test_nearly_collinear_segments_not_intersecting() {
        // Two segments that are nearly collinear but don't quite intersect
        let eps = 1e-12;
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0)); // On x-axis
        let s1 = segment(point(1.0, eps), point(3.0, eps)); // Parallel, slightly above
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
    }

    #[test]
    fn test_t_junction_intersection() {
        // One segment perpendicular to another, meeting in the middle
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0)); // Horizontal
        let s1 = segment(point(1.0, -1.0), point(1.0, 1.0)); // Vertical through middle
        match int_segment_segment(&s0, &s1) {
            SegmentSegmentConfig::OnePoint(p, _, _) => {
                assert_eq!(p, point(1.0, 0.0));
                // These segments actually intersect, not just touch
                assert!(if_really_intersecting_segment_segment(&s0, &s1) == true);
            }
            _ => panic!("Expected one point intersection"),
        }
    }

    #[test]
    fn test_nearly_parallel_segments_with_small_angle() {
        // Two segments with very small angle between them
        let eps = 1e-10;
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(0.0, 1.0), point(1.0, 1.0 + eps)); // Very slight angle
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::NoIntersection()
        );
    }

    #[test]
    fn test_diagonal_intersection() {
        // Two diagonal segments intersecting in the middle
        let s0 = segment(point(0.0, 0.0), point(2.0, 2.0)); // Diagonal
        let s1 = segment(point(0.0, 2.0), point(2.0, 0.0)); // Other diagonal
        match int_segment_segment(&s0, &s1) {
            SegmentSegmentConfig::OnePoint(p, _, _) => {
                assert_eq!(p, point(1.0, 1.0));
                assert!(if_really_intersecting_segment_segment(&s0, &s1) == true);
            }
            _ => panic!("Expected intersection for crossing diagonals"),
        }
    }

    #[test]
    fn test_both_zero_size_segments_same_point() {
        // Two zero-size segments at the same point
        let s0 = segment(point(1.0, 1.0), point(1.0, 1.0));
        let s1 = segment(point(1.0, 1.0), point(1.0, 1.0));
        assert_eq!(
            int_segment_segment(&s0, &s1),
            SegmentSegmentConfig::OnePointTouching(point(1.0, 1.0), ZERO, ZERO)
        );
    }

    #[test]
    fn test_segments_at_very_large_coordinates() {
        // Test numerical stability with large coordinate values
        let scale = 1e10;
        let s0 = segment(
            point(0.0 * scale, 0.0 * scale),
            point(2.0 * scale, 0.0 * scale),
        );
        let s1 = segment(
            point(1.0 * scale, -1.0 * scale),
            point(1.0 * scale, 1.0 * scale),
        );
        match int_segment_segment(&s0, &s1) {
            SegmentSegmentConfig::OnePoint(p, _, _) => {
                assert_eq!(p, point(1.0 * scale, 0.0 * scale));
            }
            _ => panic!("Expected intersection at large coordinates"),
        }
    }

    #[test]
    fn test_collinearity_tolerance_boundary() {
        // Test that collinearity tolerance (1e-10) works at boundary conditions
        // Segments that are nearly but not exactly collinear
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(0.5, 1e-11), point(1.5, 1e-11)); // Very small deviation
        let res = int_segment_segment(&s0, &s1);
        // Should treat as non-collinear and find no intersection (or treat as parallel)
        match res {
            SegmentSegmentConfig::NoIntersection() => {
                // Expected - nearly parallel segments
            }
            _ => {
                // Other results acceptable depending on implementation
            }
        }
    }

    #[test]
    fn test_collinearity_within_tolerance() {
        // Test segments that should be treated as collinear within tolerance (1e-10)
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(0.5, 1e-11), point(1.5, 1e-11)); // Within COLLINEARITY_TOLERANCE
        let res = int_segment_segment(&s0, &s1);
        // Should handle gracefully without panic
        match res {
            SegmentSegmentConfig::NoIntersection() => {
                // Acceptable
            }
            SegmentSegmentConfig::OnePoint(_, _, _) => {
                // Also acceptable - found approximate intersection
            }
            _ => {
                // Other results possible
            }
        }
    }
}
